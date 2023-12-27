use crate::Database;
use starpls_common::{Db as _, FileId};
use starpls_syntax::LineIndex;

pub(crate) fn line_index(db: &Database, file_id: FileId) -> Option<LineIndex> {
    let file = db.get_file(file_id)?;
    let line_index = starpls_common::line_index(db, file);
    Some(line_index.inner(db))
}
