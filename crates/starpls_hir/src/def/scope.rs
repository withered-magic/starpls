use crate::{
    def::{Declaration, Expression, ExpressionId, Parameter, Statement, StatementId},
    Module, Name,
};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;
use std::collections::{hash_map::Entry, VecDeque};

pub(crate) type ScopeId = Id<Scope>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub(crate) declarations: FxHashMap<Name, Vec<Declaration>>,
    pub(crate) parent: Option<ScopeId>,
}

pub struct Scopes {
    scopes: Arena<Scope>,
    scope_by_expression: FxHashMap<ExpressionId, ScopeId>,
}

struct DeferredScope {
    parent: ScopeId,
    deferred_function: DeferredFunctionData,
}

struct DeferredFunctionData {
    parameters: Box<[Parameter]>,
    statements: Box<[StatementId]>,
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
        compute_scopes_for_statements_with_deferred(
            &mut scopes,
            &mut deferred_scopes,
            &module.top_level,
            module,
            root,
        );

        // Compute deferred scopes. This mainly applies to function definitions.
        while let Some(DeferredScope {
            parent,
            deferred_function,
        }) = deferred_scopes.pop_front()
        {
            let scope = scopes.alloc_scope(module, parent);
            for parameter in deferred_function.parameters.into_iter() {
                match parameter {
                    Parameter::Simple { name, .. }
                    | Parameter::ArgsList { name }
                    | Parameter::KwargsList { name } => {
                        scopes.add_declaration(scope, *name, Declaration::Parameter {});
                    }
                }
            }
            compute_scopes_for_statements_with_deferred(
                &mut scopes,
                &mut deferred_scopes,
                &deferred_function.statements,
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
}

fn compute_scopes_for_expression(
    scopes: &mut Scopes,
    deferred_scopes: &mut VecDeque<DeferredScope>,
    expression: ExpressionId,
    module: &Module,
    current_scope: ScopeId,
    is_assign_target: bool,
) {
    scopes.scope_by_expression.insert(expression, current_scope);
    let mut compute_and_assign = |expression| {
        compute_scopes_for_expression(
            scopes,
            deferred_scopes,
            expression,
            module,
            current_scope,
            is_assign_target,
        )
    };

    // TODO(withered-magic): Handle list and dict comprehensions, whose CompClauses create scopes.
    match &module.expressions[expression] {
        Expression::Name { name } => {
            if is_assign_target {
                scopes.add_declaration(
                    current_scope,
                    *name,
                    Declaration::Variable { id: expression },
                );
            }
        }
        Expression::Lambda { parameters, body } => {}
        Expression::Tuple { expressions } => {
            expressions.iter().copied().for_each(compute_and_assign)
        }
        Expression::Paren { expression } => compute_and_assign(*expression),
        expression => expression.walk_child_expressions(|expression| {
            compute_scopes_for_expression(
                scopes,
                deferred_scopes,
                expression,
                module,
                current_scope,
                false,
            )
        }),
    }
}

fn compute_scopes_for_statements_with_deferred(
    scopes: &mut Scopes,
    deferred_scopes: &mut VecDeque<DeferredScope>,
    statements: &Box<[StatementId]>,
    module: &Module,
    mut current: ScopeId,
) {
    let mut deferred_functions = VecDeque::new();
    for statement in statements.iter().copied() {
        compute_scopes_for_statement(
            scopes,
            &mut deferred_functions,
            statement,
            module,
            &mut current,
        );
    }
    while let Some(deferred_function) = deferred_functions.pop_front() {
        deferred_scopes.push_back(DeferredScope {
            parent: current,
            deferred_function,
        });
    }
}

fn compute_scopes_for_statements(
    scopes: &mut Scopes,
    deferred_functions: &mut VecDeque<DeferredFunctionData>,
    statements: &Box<[StatementId]>,
    module: &Module,
    current: &mut ScopeId,
) {
    for statement in statements.iter().copied() {
        compute_scopes_for_statement(scopes, deferred_functions, statement, module, current);
    }
}

fn compute_scopes_for_statement(
    scopes: &mut Scopes,
    deferred_functions: &mut VecDeque<DeferredFunctionData>,
    statement: StatementId,
    module: &Module,
    current: &mut ScopeId,
) -> Option<DeferredFunctionData> {
    match &module.statements[statement] {
        Statement::Def {
            name,
            parameters,
            statements,
        } => {
            scopes.add_declaration(*current, *name, Declaration::Function { id: statement });
            *current = scopes.alloc_scope(module, *current);
            deferred_functions.push_back(DeferredFunctionData {
                parameters: parameters.clone(),
                statements: statements.clone(),
            });
        }
        Statement::If {
            if_statements,
            elif_statement,
            else_statements,
            ..
        } => {
            compute_scopes_for_statements(
                scopes,
                deferred_functions,
                if_statements,
                module,
                current,
            );
            if let Some(statement) = elif_statement {
                compute_scopes_for_statement(
                    scopes,
                    deferred_functions,
                    *statement,
                    module,
                    current,
                );
            }
            compute_scopes_for_statements(
                scopes,
                deferred_functions,
                else_statements,
                module,
                current,
            );
        }
        Statement::For {
            iterable,
            targets,
            statements,
        } => {}
        Statement::Assign { lhs, .. } => {}
        Statement::Load { items } => {}
        _ => {}
    }
    None
}
