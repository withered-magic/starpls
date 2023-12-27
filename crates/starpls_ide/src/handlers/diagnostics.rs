use crate::Database;
use starpls_common::{parse, Db, Diagnostic, Diagnostics, FileId};

pub(crate) fn diagnostics(db: &Database, file_id: FileId) -> Vec<Diagnostic> {
    let file = match db.get_file(file_id) {
        Some(file) => file,
        None => return Vec::new(),
    };
    let res = parse(db, file);

    // Limit the amount of syntax errors we send, as this many syntax errors probably means something
    // is really wrong with the file being analyzed.
    parse::accumulated::<Diagnostics>(db, file)
        .into_iter()
        .take(128)
        .collect()
}
