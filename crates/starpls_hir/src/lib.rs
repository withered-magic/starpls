pub use crate::def::Name;

mod api;
mod def;

#[salsa::jar(db = Db)]
pub struct Jar(Name);

pub trait Db: salsa::DbWithJar<Jar> {}
