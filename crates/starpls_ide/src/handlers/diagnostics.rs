use crate::Database;
use starpls_common::{Db, Diagnostic, Diagnostics, FileId};
use starpls_hir::module_scopes;

pub(crate) fn diagnostics(db: &Database, file_id: FileId) -> Vec<Diagnostic> {
    let file = match db.get_file(file_id) {
        Some(file) => file,
        None => return Vec::new(),
    };

    let diagnostics = db.gcx.with_tcx(db, |tcx| {
        tcx.infer_all_exprs(file);
        tcx.diagnostics_for_file(file)
    });

    // Limit the amount of syntax errors we send, as this many syntax errors probably means something
    // is really wrong with the file being analyzed.
    module_scopes::accumulated::<Diagnostics>(db, file)
        .into_iter()
        .take(128)
        .chain(diagnostics.into_iter())
        .collect()
}
