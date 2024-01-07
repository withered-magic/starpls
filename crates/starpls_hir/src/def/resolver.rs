use crate::{
    def::{
        scope::{module_scopes, Scope, ScopeHirId, ScopeId, Scopes},
        Declaration, ExprId, ModuleSourceMap,
    },
    Db, ModuleInfo, Name,
};
use starpls_syntax::{TextRange, TextSize};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

/// Resolves things like variables, function definition, etc. For now this is implemented as a simple list
/// of "module" scopes that hold variable declarations, but will need to be updated later to support other
/// features, e.g. type declarations, builtins, etc.
pub struct Resolver {
    scopes: Arc<Scopes>,
    scope_chain: Vec<ScopeId>,
}

impl Resolver {
    pub fn resolve_name(&self, name: Name) -> Option<Vec<Declaration>> {
        for scope in self.scopes() {
            if let Some(declarations) = scope.declarations.get(&name) {
                return Some(declarations.clone());
            }
        }
        None
    }

    pub fn names(&self) -> HashMap<Name, Declaration> {
        let mut names = HashMap::new();
        for scope in self.scopes() {
            for (name, decl) in scope.declarations.iter() {
                if let Entry::Vacant(entry) = names.entry(*name) {
                    if let Some(decl) = decl.first().cloned() {
                        entry.insert(decl);
                    }
                }
            }
        }
        names
    }

    fn scopes(&self) -> impl Iterator<Item = &Scope> {
        self.scope_chain
            .iter()
            .rev()
            .map(|scope| &self.scopes.scopes[*scope])
    }

    pub fn new_for_expr(db: &dyn Db, info: ModuleInfo, expr: ExprId) -> Self {
        let scopes = module_scopes(db, info).scopes(db);
        let scope = scopes.scope_for_hir_id(expr);
        let mut scope_chain = scopes.scope_chain(scope).collect::<Vec<_>>();
        scope_chain.reverse();
        Self {
            scopes,
            scope_chain,
        }
    }

    pub fn new_for_offset(db: &dyn Db, info: ModuleInfo, offset: TextSize) -> Self {
        let scopes = module_scopes(db, info).scopes(db);
        let source_map = info.source_map(db);
        let scope = scopes
            .scopes_by_hir_id
            .iter()
            .map(|(hir, scope)| {
                let ptr = match *hir {
                    ScopeHirId::Module => source_map.root.syntax_node_ptr(),
                    ScopeHirId::Expr(expr) => source_map
                        .expr_map_back
                        .get(&expr)
                        .unwrap()
                        .syntax_node_ptr(),
                    ScopeHirId::Stmt(stmt) => source_map
                        .stmt_map_back
                        .get(&stmt)
                        .unwrap()
                        .syntax_node_ptr(),
                };
                (ptr.text_range(), *scope)
            })
            .filter(|(range, _)| range.start() <= offset && offset <= range.end())
            .min_by_key(|(range, _)| range.len())
            .map(|(hir_range, scope)| {
                eprintln!("scope after initial filter: {:?}", scope);
                find_nearest_predecessor(&scopes, &source_map, hir_range, offset).unwrap_or(scope)
            });
        let mut scope_chain = scopes.scope_chain(scope).collect::<Vec<_>>();
        scope_chain.reverse();
        eprintln!("scope chain: {:?}", scope_chain);
        Self {
            scopes,
            scope_chain,
        }
    }
}

fn find_nearest_predecessor(
    scopes: &Scopes,
    source_map: &ModuleSourceMap,
    hir_range: TextRange,
    offset: TextSize,
) -> Option<ScopeId> {
    scopes
        .scopes_by_hir_id
        .iter()
        .map(|(hir, scope)| {
            let ptr = match *hir {
                ScopeHirId::Module => source_map.root.syntax_node_ptr(),
                ScopeHirId::Expr(expr) => source_map
                    .expr_map_back
                    .get(&expr)
                    .unwrap()
                    .syntax_node_ptr(),
                ScopeHirId::Stmt(stmt) => source_map
                    .stmt_map_back
                    .get(&stmt)
                    .unwrap()
                    .syntax_node_ptr(),
            };
            eprintln!("narrow {:?} {:?} {:?}", ptr.text_range(), *scope, hir);
            (ptr.text_range(), *scope)
        })
        .filter(|(range, _)| {
            range.start() <= offset && hir_range.contains_range(*range) && hir_range != *range
        })
        .max_by(|(lhs, _), (rhs, _)| {
            eprintln!("cmp");
            if lhs.contains_range(*rhs) {
                std::cmp::Ordering::Greater
            } else if lhs.contains_range(*lhs) {
                std::cmp::Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
        .map(|(_, scope)| scope)
}
