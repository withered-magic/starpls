use crate::Database;
use starpls_common::{Db as _, FileId};

pub(crate) fn view_syntax_tree(db: &Database, file_id: FileId) -> Option<String> {
    let _file = db.get_file(file_id)?;
    Some("syntax tree".to_string())
}
