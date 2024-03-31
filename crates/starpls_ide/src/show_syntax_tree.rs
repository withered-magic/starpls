use crate::Database;
use starpls_common::{parse, Db as _, FileId};

pub(crate) fn show_syntax_tree(db: &Database, file_id: FileId) -> Option<String> {
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    Some(format!("{:#?}", parse.syntax(db)))
}
