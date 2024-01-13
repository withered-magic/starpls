use crate::Db;
use std::fmt;

pub trait DisplayWithDb {
    fn fmt(&self, db: &dyn Db, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;

    fn display<'a>(&'a self, db: &'a dyn Db) -> DisplayWithDbWrapper<'a, Self> {
        DisplayWithDbWrapper { db, item: self }
    }
}

pub struct DisplayWithDbWrapper<'a, T: DisplayWithDb + ?Sized> {
    db: &'a dyn Db,
    item: &'a T,
}

impl<'a, T: DisplayWithDb> fmt::Display for DisplayWithDbWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.item.fmt(self.db, f)
    }
}
