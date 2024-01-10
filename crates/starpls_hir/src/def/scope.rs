use crate::{
    def::{CompClause, Declaration, Expr, ExprId, Param, ParamId, Stmt, StmtId},
    lower, Db, Module, ModuleInfo, Name,
};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use starpls_common::File;
use std::collections::{hash_map::Entry, VecDeque};
use std::sync::Arc;

pub(crate) type ScopeId = Id<Scope>;

#[salsa::tracked]
pub(crate) struct ModuleScopes {
    pub(crate) scopes: Arc<Scopes>,
}

#[salsa::tracked]
pub(crate) fn module_scopes_query(db: &dyn Db, info: ModuleInfo) -> ModuleScopes {
    let scopes = Scopes::new_for_module(&info.module(db));
    ModuleScopes::new(db, Arc::new(scopes))
}

#[salsa::tracked]
pub(crate) fn module_scopes(db: &dyn Db, file: File) -> ModuleScopes {
    let info = lower(db, file);
    module_scopes_query(db, info)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub(crate) declarations: FxHashMap<Name, Vec<Declaration>>,
    pub(crate) parent: Option<ScopeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ScopeHirId {
    Module,
    Expr(ExprId),
    Stmt(StmtId),
}

impl From<ExprId> for ScopeHirId {
    fn from(value: ExprId) -> Self {
        Self::Expr(value)
    }
}

impl From<StmtId> for ScopeHirId {
    fn from(value: StmtId) -> Self {
        Self::Stmt(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scopes {
    pub(crate) scopes: Arena<Scope>,
    pub(crate) scopes_by_hir_id: FxHashMap<ScopeHirId, ScopeId>,
}

struct DeferredScope {
    parent: ScopeId,
    data: DeferredFunctionData,
}

struct DeferredFunctionData {
    params: Box<[ParamId]>,
    stmts: Box<[StmtId]>,
}

impl Scopes {
    fn new_for_module(module: &Module) -> Self {
        let mut scopes = Scopes {
            scopes: Default::default(),
            scopes_by_hir_id: Default::default(),
        };

        // Allocate the root module scope.
        let root = scopes.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: None,
        });
        scopes.scopes_by_hir_id.insert(ScopeHirId::Module, root);

        let mut defer = VecDeque::new();

        // Compute scopes by walking the module HIR, starting at the top-level statements.
        compute_stmt_list_scopes_deferred(&mut scopes, &mut defer, &module.top_level, module, root);

        // Compute deferred scopes. This mainly applies to function definitions.
        while let Some(DeferredScope { parent, data }) = defer.pop_front() {
            let scope = scopes.alloc_scope(parent);
            for param in data.params.into_iter().copied() {
                match &module.params[param] {
                    Param::Simple { name, .. }
                    | Param::ArgsList { name, .. }
                    | Param::KwargsList { name, .. } => {
                        scopes.add_decl(scope, name.clone(), Declaration::Parameter { id: param });
                    }
                }
            }
            compute_stmt_list_scopes_deferred(&mut scopes, &mut defer, &data.stmts, module, scope);
        }

        scopes
    }

    fn alloc_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: Some(parent),
        })
    }

    fn add_decl(&mut self, scope: ScopeId, name: Name, decl: Declaration) {
        match self.scopes[scope].declarations.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(decl);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![decl]);
            }
        }
    }

    pub fn scope_for_hir_id(&self, id: impl Into<ScopeHirId>) -> Option<ScopeId> {
        self.scopes_by_hir_id.get(&id.into()).copied()
    }

    pub(crate) fn scope_chain(&self, scope: Option<ScopeId>) -> impl Iterator<Item = ScopeId> + '_ {
        std::iter::successors(scope, |scope| self.scopes[*scope].parent)
    }
}

fn compute_expr_scopes(
    scopes: &mut Scopes,
    expr: ExprId,
    module: &Module,
    current: ScopeId,
    is_assign_target: bool,
) {
    // TODO(withered-magic): Handle list and dict comprehensions, whose CompClauses create scopes.
    match &module.exprs[expr] {
        Expr::Missing => {}
        Expr::Name { name } => {
            if is_assign_target {
                scopes.add_decl(
                    current,
                    name.clone(),
                    Declaration::Variable {
                        id: expr,
                        source: None,
                    },
                );
            } else {
                scopes.scopes_by_hir_id.insert(expr.into(), current);
            }
        }
        Expr::Lambda { params, body } => {
            let scope = scopes.alloc_scope(current);
            for param in params.into_iter().copied() {
                match &module.params[param] {
                    Param::Simple { name, .. }
                    | Param::ArgsList { name, .. }
                    | Param::KwargsList { name, .. } => {
                        scopes.add_decl(scope, name.clone(), Declaration::Parameter { id: param });
                    }
                }
            }
            compute_expr_scopes(scopes, *body, module, scope, false);
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        Expr::Tuple { exprs } => {
            exprs.iter().copied().for_each(|expr| {
                compute_expr_scopes(scopes, expr, module, current, is_assign_target);
            });
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        Expr::Paren { expr: paren_expr } => {
            compute_expr_scopes(scopes, *paren_expr, module, current, is_assign_target);
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        Expr::List { exprs } => {
            exprs.iter().copied().for_each(|expr| {
                compute_expr_scopes(scopes, expr, module, current, is_assign_target);
            });
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        Expr::DictComp {
            entry,
            comp_clauses,
        } => {
            let mut comp = current;
            compute_comp_clause_scopes(scopes, module, comp_clauses, &mut comp);
            compute_expr_scopes(scopes, entry.key, module, comp, false);
            compute_expr_scopes(scopes, entry.value, module, comp, false);
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        Expr::ListComp {
            expr: list_expr,
            comp_clauses,
        } => {
            let mut comp = current;
            compute_comp_clause_scopes(scopes, module, comp_clauses, &mut comp);
            compute_expr_scopes(scopes, *list_expr, module, comp, false);
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
        hir_expr => {
            hir_expr
                .walk_child_exprs(|expr| compute_expr_scopes(scopes, expr, module, current, false));
            scopes.scopes_by_hir_id.insert(expr.into(), current);
        }
    }
}

fn compute_stmt_list_scopes_deferred(
    scopes: &mut Scopes,
    defer: &mut VecDeque<DeferredScope>,
    stmts: &Box<[StmtId]>,
    module: &Module,
    mut current: ScopeId,
) {
    let mut deferred_functions = VecDeque::new();
    for stmt in stmts.iter().copied() {
        compute_stmt_scopes(scopes, &mut deferred_functions, stmt, module, &mut current);
    }
    while let Some(data) = deferred_functions.pop_front() {
        defer.push_back(DeferredScope {
            parent: current,
            data,
        });
    }
}

fn compute_stmt_list_scopes(
    scopes: &mut Scopes,
    deferred_functions: &mut VecDeque<DeferredFunctionData>,
    stmts: &Box<[StmtId]>,
    module: &Module,
    current: &mut ScopeId,
) {
    for stmt in stmts.iter().copied() {
        compute_stmt_scopes(scopes, deferred_functions, stmt, module, current);
    }
}

fn compute_stmt_scopes(
    scopes: &mut Scopes,
    deferred_functions: &mut VecDeque<DeferredFunctionData>,
    stmt: StmtId,
    module: &Module,
    current: &mut ScopeId,
) {
    match &module.stmts[stmt] {
        Stmt::Def {
            name,
            params,
            stmts,
        } => {
            *current = scopes.alloc_scope(*current);
            scopes.add_decl(*current, name.clone(), Declaration::Function { id: stmt });
            deferred_functions.push_back(DeferredFunctionData {
                params: params.clone(),
                stmts: stmts.clone(),
            });
        }
        Stmt::If {
            if_stmts,
            elif_stmt,
            else_stmts,
            ..
        } => {
            compute_stmt_list_scopes(scopes, deferred_functions, if_stmts, module, current);
            if let Some(elif_stmt) = elif_stmt {
                compute_stmt_scopes(scopes, deferred_functions, *elif_stmt, module, current);
            }
            compute_stmt_list_scopes(scopes, deferred_functions, else_stmts, module, current);
        }
        Stmt::For {
            iterable,
            targets,
            stmts,
        } => {
            compute_expr_scopes(scopes, *iterable, module, *current, false);
            targets.iter().copied().for_each(|expression| {
                compute_expr_scopes(scopes, expression, module, *current, true)
            });
            compute_stmt_list_scopes(scopes, deferred_functions, stmts, module, current);
        }
        Stmt::Assign { lhs, rhs, .. } => {
            compute_expr_scopes(scopes, *rhs, module, *current, false);
            *current = scopes.alloc_scope(*current);
            compute_expr_scopes(scopes, *lhs, module, *current, true);
        }
        Stmt::Load { items } => {}
        Stmt::Return { expr } => {
            if let Some(expr) = expr {
                compute_expr_scopes(scopes, *expr, module, *current, false);
            }
        }
        Stmt::Expr { expr } => {
            compute_expr_scopes(scopes, *expr, module, *current, false);
        }
        _ => return,
    }
    scopes.scopes_by_hir_id.insert(stmt.into(), *current);
}

fn compute_comp_clause_scopes(
    scopes: &mut Scopes,
    module: &Module,
    comp_clauses: &Box<[CompClause]>,
    current: &mut ScopeId,
) {
    for comp_clause in comp_clauses.into_iter() {
        match comp_clause {
            CompClause::For { iterable, targets } => {
                compute_expr_scopes(scopes, *iterable, module, *current, false);
                targets.iter().copied().for_each(|expr| {
                    compute_expr_scopes(scopes, expr, module, *current, true);
                });
            }
            CompClause::If { test } => {
                compute_expr_scopes(scopes, *test, module, *current, false);
            }
        }
    }
}
