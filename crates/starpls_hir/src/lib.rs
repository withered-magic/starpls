use starpls_common::ParseResult;

pub use crate::def::{Module, Name};

mod api;
mod def;

#[salsa::tracked]
pub struct LowerResult {
    pub module: Module,
}

#[salsa::jar(db = Db)]
pub struct Jar(lower, LowerResult, Name);

pub trait Db: salsa::DbWithJar<Jar> + starpls_common::Db {}

#[salsa::tracked]
pub fn lower(db: &dyn Db, parse: ParseResult) -> LowerResult {
    let module = Module::new(db, parse.inner(db).tree());
    LowerResult::new(db, module)
}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> + starpls_common::Db {}
