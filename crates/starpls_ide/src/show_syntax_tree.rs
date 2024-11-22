use starpls_common::parse;
use starpls_common::Db as _;
use starpls_common::FileId;

use crate::Database;

pub(crate) fn show_syntax_tree(db: &Database, file_id: FileId) -> Option<String> {
    let file = db.get_file(file_id)?;
    let parse = parse(db, file);
    Some(format!("{:#?}", parse.syntax(db)))
}
