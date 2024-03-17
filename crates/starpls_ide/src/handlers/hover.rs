use crate::{
    hover::{self, Hover},
    Database, FilePosition,
};

pub(crate) fn hover(db: &Database, pos: FilePosition) -> Option<Hover> {
    hover::hover(db, pos)
}
