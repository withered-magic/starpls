use crate::Database;
use starpls_common::FileId;

pub(crate) fn show_hir(_db: &Database, _file_id: FileId) -> Option<String> {
    Some("Note: This functionality is now deprecated.".to_string())
}
