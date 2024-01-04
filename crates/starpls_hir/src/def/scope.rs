use crate::{
    def::{CompClause, Declaration, Expr, ExprId, Parameter, Stmt, StmtId},
    Db, Module, ModuleInfo, Name,
};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use std::collections::{hash_map::Entry, VecDeque};
use std::sync::Arc;

pub(crate) type ScopeId = Id<Scope>;

#[salsa::tracked]
pub(crate) struct ModuleScopes {
    pub(crate) scopes: Arc<Scopes>,
}

#[salsa::tracked]
pub(crate) fn module_scopes(db: &dyn Db, info: ModuleInfo) -> ModuleScopes {
    let scopes = Scopes::new_for_module(&info.module(db));
    ModuleScopes::new(db, Arc::new(scopes))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub(crate) declarations: FxHashMap<Name, Vec<Declaration>>,
    pub(crate) parent: Option<ScopeId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scopes {
    pub(crate) scopes: Arena<Scope>,
    pub(crate) scope_by_expr: FxHashMap<ExprId, ScopeId>,
}

struct DeferredScope {
    parent: ScopeId,
    data: DeferredFunctionData,
}

struct DeferredFunctionData {
    params: Box<[Parameter]>,
    stmts: Box<[StmtId]>,
}

impl Scopes {
    fn new_for_module(module: &Module) -> Self {
        let mut scopes = Scopes {
            scopes: Default::default(),
            scope_by_expr: Default::default(),
        };

        // Allocate the root module scope.
        let root = scopes.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: None,
        });

        let mut deferred_scopes = VecDeque::new();

        // Compute scopes by walking the module HIR, starting at the top-level statements.
        compute_stmt_list_scopes_deferred(
            &mut scopes,
            &mut deferred_scopes,
            &module.top_level,
            module,
            root,
        );

        // Compute deferred scopes. This mainly applies to function definitions.
        while let Some(DeferredScope { parent, data }) = deferred_scopes.pop_front() {
            let scope = scopes.alloc_scope(parent);
            for param in data.params.into_iter() {
                match param {
                    Parameter::Simple { name, .. }
                    | Parameter::ArgsList { name }
                    | Parameter::KwargsList { name } => {
                        scopes.add_declaration(scope, *name, Declaration::Parameter {});
                    }
                }
            }
            compute_stmt_list_scopes_deferred(
                &mut scopes,
                &mut deferred_scopes,
                &data.stmts,
                module,
                scope,
            );
        }

        scopes
    }

    fn alloc_scope(&mut self, parent: ScopeId) -> ScopeId {
        self.scopes.alloc(Scope {
            declarations: Default::default(),
            parent: Some(parent),
        })
    }

    fn add_declaration(&mut self, scope: ScopeId, name: Name, declaration: Declaration) {
        match self.scopes[scope].declarations.entry(name) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(declaration);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![declaration]);
            }
        }
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
    scopes.scope_by_expr.insert(expr, current);

    // TODO(withered-magic): Handle list and dict comprehensions, whose CompClauses create scopes.
    match &module.exprs[expr] {
        Expr::Name { name } => {
            if is_assign_target {
                scopes.add_declaration(current, *name, Declaration::Variable { id: expr });
            }
        }
        Expr::Lambda { params, body } => {
            let scope = scopes.alloc_scope(current);
            for param in params.into_iter() {
                match param {
                    Parameter::Simple { name, .. }
                    | Parameter::ArgsList { name }
                    | Parameter::KwargsList { name } => {
                        scopes.add_declaration(scope, *name, Declaration::Parameter {});
                    }
                }
            }
            compute_expr_scopes(scopes, *body, module, scope, false);
        }
        Expr::Tuple { exprs } => exprs.iter().copied().for_each(|expr| {
            compute_expr_scopes(scopes, expr, module, current, is_assign_target);
        }),
        Expr::Paren { expr } => {
            compute_expr_scopes(scopes, *expr, module, current, is_assign_target)
        }
        Expr::DictComp {
            entry,
            comp_clauses,
        } => {
            let mut comp = current;
            compute_comp_clause_scopes(scopes, module, comp_clauses, &mut comp);
            compute_expr_scopes(scopes, entry.key, module, comp, false);
            compute_expr_scopes(scopes, entry.value, module, comp, false);
        }
        Expr::ListComp { expr, comp_clauses } => {
            let mut comp = current;
            compute_comp_clause_scopes(scopes, module, comp_clauses, &mut comp);
            compute_expr_scopes(scopes, *expr, module, comp, false);
        }
        expr => {
            expr.walk_child_exprs(|expr| compute_expr_scopes(scopes, expr, module, current, false));
        }
    }
}

fn compute_stmt_list_scopes_deferred(
    scopes: &mut Scopes,
    deferred_scopes: &mut VecDeque<DeferredScope>,
    stmts: &Box<[StmtId]>,
    module: &Module,
    mut current: ScopeId,
) {
    let mut deferred_functions = VecDeque::new();
    for stmt in stmts.iter().copied() {
        compute_stmt_scopes(scopes, &mut deferred_functions, stmt, module, &mut current);
    }
    while let Some(data) = deferred_functions.pop_front() {
        deferred_scopes.push_back(DeferredScope {
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
    statement: StmtId,
    module: &Module,
    current: &mut ScopeId,
) {
    match &module.stmts[statement] {
        Stmt::Def {
            name,
            params,
            stmts,
        } => {
            scopes.add_declaration(*current, *name, Declaration::Function { id: statement });
            *current = scopes.alloc_scope(*current);
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
        Stmt::Expr { expr } => compute_expr_scopes(scopes, *expr, module, *current, false),
        _ => {}
    }
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
