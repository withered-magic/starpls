use crate::{
    def::{Declaration, Expr, ExprId, Parameter, Stmt, StmtId},
    Db, LowerResult, Module, Name,
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
pub(crate) fn module_scopes(db: &dyn Db, lower_res: LowerResult) -> ModuleScopes {
    let scopes = Scopes::new_for_module(&lower_res.module(db));
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
    pub(crate) scope_by_expression: FxHashMap<ExprId, ScopeId>,
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
            scope_by_expression: Default::default(),
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
            let scope = scopes.alloc_scope(module, parent);
            for parameter in data.params.into_iter() {
                match parameter {
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

    fn alloc_scope(&mut self, module: &Module, parent: ScopeId) -> ScopeId {
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
    scopes.scope_by_expression.insert(expr, current);
    let mut compute_and_assign =
        |expression| compute_expr_scopes(scopes, expression, module, current, is_assign_target);

    // TODO(withered-magic): Handle list and dict comprehensions, whose CompClauses create scopes.
    match &module.exprs[expr] {
        Expr::Name { name } => {
            if is_assign_target {
                scopes.add_declaration(current, *name, Declaration::Variable { id: expr });
            }
        }
        Expr::Lambda { params, body } => {}
        Expr::Tuple { exprs } => exprs.iter().copied().for_each(compute_and_assign),
        Expr::Paren { expr } => compute_and_assign(*expr),
        Expr::DictComp {
            entry,
            comp_clauses,
        } => {}
        Expr::ListComp { expr, comp_clauses } => {}
        expression => expression.walk_child_exprs(|expression| {
            compute_expr_scopes(scopes, expression, module, current, false)
        }),
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
            *current = scopes.alloc_scope(module, *current);
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
                compute_expr_scopes(scopes, expression, module, *current, false)
            });
            compute_stmt_list_scopes(scopes, deferred_functions, stmts, module, current);
        }
        Stmt::Assign { lhs, .. } => {
            *current = scopes.alloc_scope(module, *current);
            compute_expr_scopes(scopes, *lhs, module, *current, true);
        }
        Stmt::Load { items } => {}
        _ => {}
    }
}
