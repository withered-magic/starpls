use crate::{handlers::*, hover::Hover};
use completions::CompletionItem;
use dashmap::{mapref::entry::Entry, DashMap};
use salsa::ParallelDatabase;
use starpls_bazel::Builtins;
use starpls_common::{Db, Diagnostic, File, FileId};
use starpls_hir::{CustomDefs, Db as _, Dialect, ExprId, GlobalCtxt, Ty};
use starpls_syntax::{LineIndex, TextRange, TextSize};
use std::sync::Arc;

mod handlers;
mod util;

pub mod completions;
pub mod hover;

pub type Cancellable<T> = Result<T, starpls_hir::Cancelled>;

#[derive(Default)]
#[salsa::db(starpls_common::Jar, starpls_hir::Jar)]
pub(crate) struct Database {
    custom_defs: Arc<DashMap<Dialect, CustomDefs>>,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    gcx: Arc<GlobalCtxt>,
}

impl Database {
    fn apply_file_changes(&mut self, changes: Vec<(FileId, String)>) {
        let gcx = self.gcx.clone();
        let _guard = gcx.cancel();
        for (file_id, contents) in changes {
            self.set_file_contents(file_id, contents);
        }
    }
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            custom_defs: self.custom_defs.clone(),
            files: self.files.clone(),
            gcx: self.gcx.clone(),
            storage: self.storage.snapshot(),
        })
    }
}

impl starpls_common::Db for Database {
    fn set_file_contents(&mut self, file_id: FileId, contents: String) -> File {
        let file = match self.files.entry(file_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => return *entry.insert(File::new(self, file_id, contents)),
        };
        file.set_contents(self).to(contents);
        file
    }

    fn get_file(&self, file_id: FileId) -> Option<File> {
        self.files.get(&file_id).map(|file| *file)
    }
}

impl starpls_hir::Db for Database {
    fn infer_expr(&self, file: File, expr: ExprId) -> Ty {
        self.gcx.with_tcx(self, |tcx| tcx.infer_expr(file, expr))
    }

    fn set_custom_defs(&mut self, dialect: Dialect, builtins: Builtins) {
        let defs = match self.custom_defs.entry(dialect) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(CustomDefs::new(self, builtins));
                return;
            }
        };
        defs.set_builtins(self).to(builtins);
    }

    fn get_custom_defs(&self, dialect: &Dialect) -> CustomDefs {
        self.custom_defs
            .get(dialect)
            .map(|defs| *defs)
            .unwrap_or(CustomDefs::new(self, Builtins::default()))
    }
}

/// A batch of changes to be applied to the database. For now, this consists simply of a map of changed file IDs to
/// their updated contents.
#[derive(Default)]
pub struct Change {
    changed_files: Vec<(FileId, String)>,
}

impl Change {
    pub fn add_file(&mut self, file_id: FileId, contents: String) {
        self.changed_files.push((file_id, contents))
    }
}

/// Provides the main API for querying facts about the source code. This wraps the main `Database` struct.
pub struct Analysis {
    db: Database,
}

impl Analysis {
    pub fn new() -> Self {
        Self {
            db: Default::default(),
        }
    }

    pub fn apply_change(&mut self, change: Change) {
        self.db.apply_file_changes(change.changed_files);
    }

    pub fn snapshot(&self) -> AnalysisSnapshot {
        AnalysisSnapshot {
            db: self.db.snapshot(),
        }
    }

    pub fn set_custom_defs(&mut self, builtins: Builtins) {
        self.db.set_custom_defs(Dialect::Bazel, builtins);
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
}

impl AnalysisSnapshot {
    pub fn completion(&self, pos: FilePosition) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.query(|db| completion::completion(db, pos))
    }

    pub fn diagnostics(&self, file_id: FileId) -> Cancellable<Vec<Diagnostic>> {
        self.query(|db| diagnostics::diagnostics(db, file_id))
    }

    pub fn goto_definition(&self, pos: FilePosition) -> Cancellable<Option<Vec<Location>>> {
        self.query(|db| {
            let res = goto_definition::goto_definition(db, pos);
            res
        })
    }

    pub fn hover(&self, pos: FilePosition) -> Cancellable<Option<Hover>> {
        self.query(|db| hover::hover(db, pos))
    }

    pub fn line_index(&self, file_id: FileId) -> Cancellable<Option<LineIndex>> {
        self.query(|db| line_index::line_index(db, file_id))
    }

    pub fn show_hir(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| show_hir::show_hir(db, file_id))
    }

    pub fn show_syntax_tree(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| show_syntax_tree::show_syntax_tree(db, file_id))
    }

    /// Helper method to handle Salsa cancellations.
    fn query<F, T>(&self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&Database) -> T + std::panic::UnwindSafe,
    {
        starpls_hir::Cancelled::catch(|| f(&self.db))
    }
}

impl std::panic::RefUnwindSafe for AnalysisSnapshot {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub file_id: FileId,
    pub range: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilePosition {
    pub file_id: FileId,
    pub pos: TextSize,
}
