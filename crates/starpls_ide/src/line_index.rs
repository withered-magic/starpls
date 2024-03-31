use crate::Database;
use starpls_common::{Db as _, FileId};
use starpls_syntax::LineIndex;

pub(crate) fn line_index<'a>(db: &'a Database, file_id: FileId) -> Option<&'a LineIndex> {
    let file = db.get_file(file_id)?;
    Some(starpls_common::line_index(db, file))
}
