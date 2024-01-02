use crate::Database;
use starpls_common::{parse, Db, FileId};
use starpls_hir::lower;

pub(crate) fn show_hir(db: &Database, file_id: FileId) -> Option<String> {
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    let lower = lower(db, parse);
    Some(format!("{:#?}", lower.module(db)))
}
