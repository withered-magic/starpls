pub use crate::def::{resolver::Resolver, Declaration, Module, Name};
use def::ModuleSourceMap;
use starpls_common::Parse;

mod api;
mod def;
mod ty;

#[salsa::tracked]
pub struct ModuleInfo {
    #[return_ref]
    pub module: Module,
    #[return_ref]
    pub source_map: ModuleSourceMap,
}

// #[salsa::tracked]
// pub struct BindResult {
//     pub module_info: ModuleInfo,
// }

#[salsa::jar(db = Db)]
pub struct Jar(
    lower,
    ModuleInfo,
    Name,
    def::scope::ModuleScopes,
    def::scope::module_scopes,
);

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {}

#[salsa::tracked]
pub fn lower(db: &dyn Db, parse: Parse) -> ModuleInfo {
    let (module, source_map) = Module::new_with_source_map(db, parse.tree(db));
    ModuleInfo::new(db, module, source_map)
}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> + starpls_common::Db {}
