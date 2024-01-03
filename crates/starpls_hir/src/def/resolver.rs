use crate::{
    def::{
        scope::{module_scopes, Scope, ScopeId, Scopes},
        Declaration, ExpressionId,
    },
    Db, LowerResult, Name,
};
use std::sync::Arc;

/// Resolves things like variables, function definition, etc. For now this is implemented as a simple list
/// of "module" scopes that hold variable declarations, but will need to be updated later to support other
/// features, e.g. type declarations, builtins, etc.
pub struct Resolver {
    scopes: Arc<Scopes>,
    scope_chain: Vec<ScopeId>,
}

impl Resolver {
    fn resolve_name(&self, name: Name) -> Option<Vec<Declaration>> {
        for scope in self.scopes() {
            if let Some(declarations) = scope.declarations.get(&name) {
                return Some(declarations.clone());
            }
        }
        None
    }

    fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scope_chain
            .iter()
            .rev()
            .map(|scope| &self.scopes.scopes[*scope])
    }

    fn new_for_expression(
        db: &dyn Db,
        lower_result: LowerResult,
        expression: ExpressionId,
    ) -> Self {
        let scopes = module_scopes(db, lower_result);
        let scopes = scopes.scopes(db);
        let scope = scopes.scope_by_expression.get(&expression).cloned();
        let mut scope_chain = scopes.scope_chain(scope).collect::<Vec<_>>();
        scope_chain.reverse();
        Self {
            scopes,
            scope_chain,
        }
    }
}
