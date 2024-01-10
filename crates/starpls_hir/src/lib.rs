use crate::def::ModuleSourceMap;
use starpls_common::{parse, File, Parse};

pub use crate::{
    def::{resolver::Resolver, Declaration, Module, Name},
    typeck::{TyCtxt, TyCtxtSnapshot},
};

mod api;
mod def;
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
);

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {}

#[salsa::tracked]
fn lower_query(db: &dyn Db, parse: Parse) -> ModuleInfo {
    let (module, source_map) = Module::new_with_source_map(db, parse.tree(db));
    ModuleInfo::new(db, module, source_map)
}

#[salsa::tracked]
pub fn lower(db: &dyn Db, file: File) -> ModuleInfo {
    let parse = parse(db, file);
    lower_query(db, parse)
}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> + starpls_common::Db {}
