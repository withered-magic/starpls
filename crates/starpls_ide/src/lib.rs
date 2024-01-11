use crate::{handlers::*, hover::Hover};
use completions::CompletionItem;
use dashmap::{mapref::entry::Entry, DashMap};
use salsa::ParallelDatabase;
use starpls_common::{Db, Diagnostic, File, FileId};
use starpls_hir::{intern_builtins, TyCtxt, TyCtxtSnapshot};
use starpls_syntax::{LineIndex, TextRange, TextSize};
use std::sync::Arc;

mod handlers;
mod util;

pub mod completions;
pub mod hover;

pub type Cancellable<T> = Result<T, salsa::Cancelled>;

#[derive(Default)]
#[salsa::db(starpls_common::Jar, starpls_hir::Jar)]
pub(crate) struct Database {
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            storage: self.storage.snapshot(),
            files: self.files.clone(),
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
    tcx: TyCtxt,
}

impl Analysis {
    pub fn new() -> Self {
        Self {
            db: Default::default(),
            tcx: TyCtxt::new_with_builtins(intern_builtins()),
        }
    }

    pub fn apply_change(&mut self, change: Change) {
        self.tcx.cancel();
        for (path, contents) in change.changed_files {
            self.db.set_file_contents(path, contents);
        }
    }

    pub fn snapshot(&self) -> AnalysisSnapshot {
        AnalysisSnapshot {
            db: self.db.snapshot(),
            tcx: self.tcx.snapshot(),
        }
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
    tcx: TyCtxtSnapshot,
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
        self.query_with_tcx(|db, tcx| hover::hover(db, tcx, pos))
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
        salsa::Cancelled::catch(|| f(&self.db))
    }

    fn query_with_tcx<F, T>(&self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&Database, &TyCtxtSnapshot) -> T + std::panic::UnwindSafe,
    {
        salsa::Cancelled::catch(|| f(&self.db, &self.tcx))
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
