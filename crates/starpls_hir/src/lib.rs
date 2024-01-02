pub use crate::{
    def::{Module, Name},
    ty::ModuleInfo,
};
use starpls_common::ParseResult;

mod api;
mod def;
mod ty;

#[salsa::tracked]
pub struct LowerResult {
    pub module: Module,
}

#[salsa::tracked]
pub struct BindResult {
    pub module_info: ModuleInfo,
}

#[salsa::jar(db = Db)]
pub struct Jar(bind, lower, BindResult, LowerResult, Name);

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {}

#[salsa::tracked]
pub fn lower(db: &dyn Db, parse: ParseResult) -> LowerResult {
    let module = Module::new(db, parse.inner(db).tree());
    LowerResult::new(db, module)
}

#[salsa::tracked]
pub fn bind(db: &dyn Db, lower: LowerResult) -> BindResult {
    let info = ty::bind_module(lower.module(db));
    BindResult::new(db, info)
}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> + starpls_common::Db {}
