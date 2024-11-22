use starpls_common::Db as _;
use starpls_common::FileId;
use starpls_syntax::LineIndex;

use crate::Database;

pub(crate) fn line_index(db: &Database, file_id: FileId) -> Option<&LineIndex> {
    let file = db.get_file(file_id)?;
    Some(starpls_common::line_index(db, file))
}
