use crate::Db;
use std::fmt;

pub trait DisplayWithDb {
    fn fmt(&self, db: &dyn Db, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn fmt_alt(&self, db: &dyn Db, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(db, f)
    }

    fn display<'a>(&'a self, db: &'a dyn Db) -> DisplayWithDbWrapper<'a, Self> {
        DisplayWithDbWrapper {
            db,
            item: self,
            alt: false,
        }
    }
}

pub struct DisplayWithDbWrapper<'a, T: DisplayWithDb + ?Sized> {
    db: &'a dyn Db,
    item: &'a T,
    alt: bool,
}

impl<'a, T: DisplayWithDb> DisplayWithDbWrapper<'a, T> {
    pub fn alt(self) -> Self {
        Self { alt: true, ..self }
    }
}

impl<'a, T: DisplayWithDb> fmt::Display for DisplayWithDbWrapper<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alt {
            self.item.fmt_alt(self.db, f)
        } else {
            self.item.fmt(self.db, f)
        }
    }
}
