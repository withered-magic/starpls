use crate::def::ModuleSourceMap;
use starpls_common::{parse, File, Parse};

pub use crate::{
    def::{resolver::Resolver, scope::module_scopes, Declaration, ExprId, Module, Name},
    display::{DisplayWithDb, DisplayWithDbWrapper},
    typeck::{
        builtins::BuiltinClass, Cancelled, FileExprId, GlobalCtxt, Ty, TyCtxt, TyKind, TypeRef,
    },
};

mod api;
mod def;
mod display;
mod test_database;
mod typeck;

#[salsa::tracked]
pub struct ModuleInfo {
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
    def::scope::ModuleScopes,
    def::scope::module_scopes,
    def::scope::module_scopes_query,
    typeck::builtins::BuiltinClass,
    typeck::builtins::BuiltinFieldTypes,
    typeck::builtins::BuiltinFunction,
    typeck::builtins::BuiltinTypes,
    typeck::builtins::builtin_field_types,
    typeck::builtins::builtin_types,
);

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {
    fn infer_expr(&self, file: File, expr: ExprId) -> Ty;
}

#[salsa::tracked]
fn lower_query(db: &dyn Db, file: File, parse: Parse) -> ModuleInfo {
    let (module, source_map) = Module::new_with_source_map(db, file, parse.tree(db));
    ModuleInfo::new(db, module, source_map)
}

#[salsa::tracked]
pub fn lower(db: &dyn Db, file: File) -> ModuleInfo {
    let parse = parse(db, file);
    lower_query(db, file, parse)
}
