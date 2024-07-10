use std::{collections::hash_map::Entry, iter};

use rustc_hash::FxHashMap;
use starpls_bazel::APIContext;
use starpls_common::File;
use starpls_syntax::{TextRange, TextSize};

use crate::{
    def::{
        scope::{
            module_scopes, ExecutionScopeId, Scope, ScopeDef, ScopeHirId, ScopeId, Scopes,
            VariableDef,
        },
        ExprId, Function, ModuleSourceMap,
    },
    source_map,
    typeck::{
        builtins::{builtin_globals, APIGlobals},
        intrinsics::intrinsic_functions,
    },
    Db, Name,
};

/// Resolves things like variables, function definition, etc. For now this is implemented as a simple list
/// of "module" scopes that hold variable declarations, but will need to be updated later to support other
/// features, e.g. type declarations, builtins, etc.
pub(crate) struct Resolver<'a> {
    db: &'a dyn Db,
    file: File,
    scopes: &'a Scopes,
    scope_chain: Vec<ScopeId>,
}

#[derive(Clone, Debug)]
pub(crate) enum Export {
    Variable(VariableDef),
    Function(Function),
}

impl From<Export> for ScopeDef {
    fn from(value: Export) -> Self {
        match value {
            Export::Variable(def) => ScopeDef::Variable(def),
            Export::Function(func) => ScopeDef::Function(func),
        }
    }
}

impl<'a> Resolver<'a> {
    pub(crate) fn resolve_export_in_file(
        db: &'a dyn Db,
        file: File,
        name: &Name,
    ) -> Option<Export> {
        Self::new_for_module(db, file).resolve_export(name)
    }

    pub(crate) fn resolve_export(&self, name: &Name) -> Option<Export> {
        if name.as_str().starts_with('_') {
            return None;
        }

        self.scopes().find_map(|scope| {
            scope
                .defs
                .get(name)
                .and_then(|decls| decls.last())
                .and_then(|decl| {
                    Some(match decl {
                        ScopeDef::Variable(def) => Export::Variable(def.clone()),
                        ScopeDef::Function(func) => Export::Function(*func),
                        _ => return None,
                    })
                })
        })
    }

    pub(crate) fn resolve_name(
        &'a self,
        name: &'a Name,
    ) -> Option<(ExecutionScopeId, impl Iterator<Item = SymbolDef<'a>> + '_)> {
        let mut defs = self
            .scopes_with_id()
            .filter_map(move |(scope_id, scope)| {
                scope
                    .defs
                    .get(&name)
                    .map(|defs| (scope_id, scope.execution_scope, defs))
            })
            .flat_map(|(scope, execution_scope, defs)| {
                defs.iter().map(move |def| SymbolDef {
                    scope,
                    execution_scope,
                    def,
                })
            });
        let first = defs.next()?;
        let first_execution_scope = first.execution_scope;
        let defs = iter::once(first)
            .chain(defs.take_while(move |def| def.execution_scope == first_execution_scope));
        Some((first_execution_scope, defs))
    }

    pub(crate) fn resolve_name_in_prelude_or_builtins(&self, name: &Name) -> Option<Vec<ScopeDef>> {
        // Fall back to prelude, and then to the builtins scope.
        let mut defs = None;
        if self.file.api_context(self.db) == Some(APIContext::Build) {
            defs = self
                .db
                .get_bazel_prelude_file()
                .and_then(|prelude_file_id| {
                    let prelude_file = self.db.get_file(prelude_file_id)?;
                    Resolver::resolve_export_in_file(self.db, prelude_file, name)
                })
                .map(|export| vec![export.into()])
        }

        defs.or_else(|| {
            intrinsic_functions(self.db)
                .functions(self.db)
                .get(name)
                .copied()
                .map(|func| vec![ScopeDef::IntrinsicFunction(func)])
        })
        .or_else(|| self.resolve_name_in_builtin_globals(name))
    }

    fn resolve_name_in_builtin_globals(&self, name: &Name) -> Option<Vec<ScopeDef>> {
        let api_context = self.file.api_context(self.db)?;
        let globals = builtin_globals(self.db, self.file.dialect(self.db));
        let resolve_in_api_globals = |api_globals: &APIGlobals| {
            api_globals
                .functions
                .get(name.as_str())
                .copied()
                .map(|func| vec![ScopeDef::BuiltinFunction(func)])
                .or_else(|| {
                    api_globals
                        .variables
                        .get(name.as_str())
                        .cloned()
                        .map(|type_ref| vec![ScopeDef::BuiltinVariable(type_ref)])
                })
        };

        if api_context == APIContext::Repo {
            return resolve_in_api_globals(globals.repo_globals(self.db));
        }

        resolve_in_api_globals(globals.bzl_globals(self.db)).or_else(|| match api_context {
            APIContext::Module => resolve_in_api_globals(globals.bzlmod_globals(self.db)),
            APIContext::Workspace => resolve_in_api_globals(globals.workspace_globals(self.db)),
            _ => None,
        })
    }

    pub(crate) fn names(&self) -> FxHashMap<Name, ScopeDef> {
        let builtin_globals = builtin_globals(self.db, self.file.dialect(self.db));

        // Add names from this module.
        let mut names = self.module_names();

        // Add names from Starlark intrinsics.
        for (key, func) in intrinsic_functions(self.db).functions(self.db).iter() {
            names.insert(key.clone(), ScopeDef::IntrinsicFunction(*func));
        }

        let api_context = match self.file.api_context(self.db) {
            Some(api_context) => api_context,
            None => return names,
        };

        // If this is a BUILD file, add names from the prelude.
        if api_context == APIContext::Build && self.file.is_external(self.db) == Some(false) {
            if let Some(prelude_file) = self
                .db
                .get_bazel_prelude_file()
                .and_then(|prelude_file_id| self.db.get_file(prelude_file_id))
            {
                let prelude_resolver = Resolver::new_for_module(self.db, prelude_file);
                names.extend(
                    prelude_resolver
                        .module_defs(true)
                        .into_iter()
                        .filter(|(_, def)| {
                            matches!(def, ScopeDef::Variable(_) | ScopeDef::Function(_))
                        }),
                );
            }
        }

        // Add names from builtins, taking the current Bazel API context into account.
        let mut add_builtins = |api_globals: &APIGlobals| {
            for (name, func) in api_globals.functions.iter() {
                names.insert(Name::from_str(&name), ScopeDef::BuiltinFunction(*func));
            }
            for (name, type_ref) in api_globals.variables.iter() {
                names.insert(
                    Name::from_str(&name),
                    ScopeDef::BuiltinVariable(type_ref.clone()),
                );
            }
        };

        if api_context == APIContext::Repo {
            add_builtins(builtin_globals.repo_globals(self.db));
        } else {
            add_builtins(builtin_globals.bzl_globals(self.db));
            match api_context {
                APIContext::Module => add_builtins(builtin_globals.bzlmod_globals(self.db)),
                APIContext::Workspace => add_builtins(builtin_globals.workspace_globals(self.db)),
                _ => {}
            }
        }

        names
    }

    pub(crate) fn module_names(&self) -> FxHashMap<Name, ScopeDef> {
        self.module_defs(false)
    }

    pub(crate) fn module_defs(&self, filter_unexported: bool) -> FxHashMap<Name, ScopeDef> {
        let mut names = FxHashMap::default();
        for scope in self.scopes() {
            for (name, def) in scope.defs.iter() {
                if (filter_unexported && name.as_str().starts_with('_')) || name.is_missing() {
                    continue;
                }
                if let Entry::Vacant(entry) = names.entry(name.clone()) {
                    if let Some(def) = def.first().cloned() {
                        entry.insert(def);
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

    fn scopes_with_id(&self) -> impl Iterator<Item = (ScopeId, &Scope)> {
        self.scope_chain
            .iter()
            .rev()
            .map(|scope| (*scope, &self.scopes.scopes[*scope]))
    }

    pub(crate) fn new_for_module(db: &'a dyn Db, file: File) -> Self {
        let scopes = module_scopes(db, file).scopes(db);
        let scope = scopes.scope_for_hir_id(ScopeHirId::Module);
        Self::from_parts(db, file, scopes, scope)
    }

    pub(crate) fn new_for_expr(db: &'a dyn Db, file: File, expr: ExprId) -> Self {
        let scopes = module_scopes(db, file).scopes(db);
        let scope = scopes.scope_for_hir_id(expr);
        Self::from_parts(db, file, scopes, scope)
    }

    pub(crate) fn new_for_expr_execution_scope(db: &'a dyn Db, file: File, expr: ExprId) -> Self {
        let scopes = module_scopes(db, file).scopes(db);
        let scope = scopes.scope_for_expr_execution_scope(expr);
        Self::from_parts(db, file, scopes, scope)
    }

    pub(crate) fn new_for_offset(db: &'a dyn Db, file: File, offset: TextSize) -> Self {
        let scopes = module_scopes(db, file).scopes(db);
        let source_map = source_map(db, file);
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
                find_nearest_predecessor(&scopes, &source_map, hir_range, offset).unwrap_or(scope)
            });
        Self::from_parts(db, file, scopes, scope)
    }

    pub(crate) fn scope_for_expr(&self, expr: ExprId) -> Option<ScopeId> {
        self.scopes.scope_for_hir_id(expr)
    }

    pub(crate) fn execution_scope_for_expr(&self, expr: ExprId) -> Option<ExecutionScopeId> {
        self.scopes.execution_scope_for_expr(expr)
    }

    fn from_parts(db: &'a dyn Db, file: File, scopes: &'a Scopes, scope: Option<ScopeId>) -> Self {
        let mut scope_chain = scopes.scope_chain(scope).collect::<Vec<_>>();
        scope_chain.reverse();
        Self {
            db,
            file,
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
            (ptr.text_range(), *scope)
        })
        .filter(|(range, _)| {
            range.start() <= offset && hir_range.contains_range(*range) && hir_range != *range
        })
        .max_by(|(lhs, _), (rhs, _)| {
            if lhs.contains_range(*rhs) {
                std::cmp::Ordering::Greater
            } else if rhs.contains_range(*lhs) {
                std::cmp::Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
        .map(|(_, scope)| scope)
}

#[derive(Clone, Debug)]
pub(crate) struct SymbolDef<'a> {
    pub(crate) scope: ScopeId,
    pub(crate) execution_scope: ExecutionScopeId,
    pub(crate) def: &'a ScopeDef,
}
