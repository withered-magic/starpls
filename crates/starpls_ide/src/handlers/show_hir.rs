use crate::Database;
use starpls_common::{Db, FileId};
use starpls_hir::lower;

pub(crate) fn show_hir(db: &Database, file_id: FileId) -> Option<String> {
    let file = db.get_file(file_id)?;
    let lower = lower(db, file);
    Some(format!("{:#?}", lower.module(db)))
}
