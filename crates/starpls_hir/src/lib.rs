use crate::def::ModuleSourceMap;
use starpls_bazel::Builtins;
use starpls_common::{parse, File, Parse};

pub use crate::{
    api::*,
    def::{resolver::Resolver, scope::module_scopes, Declaration, ExprId, Module, Name},
    display::{DisplayWithDb, DisplayWithDbWrapper},
    typeck::{
        builtins::BuiltinDefs, intrinsics::IntrinsicClass, Cancelled, FileExprId, GlobalCtxt, Ty,
        TyCtxt, TyKind, TypeRef,
    },
};

mod api;
mod def;
mod display;
mod test_database;
mod typeck;

#[salsa::tracked]
pub struct ModuleInfo {
    pub file: File,
    #[return_ref]
    pub module: Module,
    #[return_ref]
    pub source_map: ModuleSourceMap,
}

#[salsa::jar(db = Db)]
pub struct Jar(
    lower,
    lower_query,
    ModuleInfo,
    def::Function,
    def::scope::ModuleScopes,
    def::scope::module_scopes,
    def::scope::module_scopes_query,
    typeck::builtins::BuiltinDefs,
    typeck::builtins::BuiltinFunction,
    typeck::builtins::BuiltinGlobals,
    typeck::builtins::BuiltinType,
    typeck::builtins::BuiltinTypes,
    typeck::builtins::builtin_globals_query,
    typeck::builtins::builtin_types_query,
    typeck::intrinsics::Intrinsics,
    typeck::intrinsics::IntrinsicClass,
    typeck::intrinsics::IntrinsicFieldTypes,
    typeck::intrinsics::IntrinsicFunction,
    typeck::intrinsics::IntrinsicFunctions,
    typeck::intrinsics::intrinsic_types,
    typeck::intrinsics::intrinsic_field_types,
    typeck::intrinsics::intrinsic_functions,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dialect {
    Bazel,
}

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {
    fn infer_expr(&self, file: File, expr: ExprId) -> Ty;

    fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins);

    fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs;
}

#[salsa::tracked]
fn lower_query(db: &dyn Db, parse: Parse) -> ModuleInfo {
    let file = parse.file(db);
    let (module, source_map) = Module::new_with_source_map(db, file, parse.tree(db));
    ModuleInfo::new(db, file, module, source_map)
}

#[salsa::tracked]
pub fn lower(db: &dyn Db, file: File) -> ModuleInfo {
    let parse = parse(db, file);
    lower_query(db, parse)
}

/// Shortcut to immediately access a `lower` query's `Module`.
pub fn module(db: &dyn Db, file: File) -> &Module {
    lower(db, file).module(db)
}

/// Shortcut to immediately access a `lower` query's `ModuleSourceMap`.
pub fn source_map(db: &dyn Db, file: File) -> &ModuleSourceMap {
    lower(db, file).source_map(db)
}
