use dashmap::{mapref::entry::Entry, DashMap};
use salsa::ParallelDatabase;
use starpls_common::{Db, Diagnostic, File, FileId};
use std::sync::Arc;

mod diagnostics;
mod view_syntax_tree;

pub type Cancellable<T> = Result<T, salsa::Cancelled>;

#[derive(Default)]
#[salsa::db(starpls_common::Jar)]
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
#[derive(Default)]
pub struct Analysis {
    db: Database,
}

impl Analysis {
    pub fn apply_change(&mut self, change: Change) {
        for (path, contents) in change.changed_files {
            self.db.set_file_contents(path, contents);
        }
    }

    pub fn snapshot(&self) -> AnalysisSnapshot {
        AnalysisSnapshot {
            db: self.db.snapshot(),
        }
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
}

impl AnalysisSnapshot {
    pub fn diagnostics(&self, file_id: FileId) -> Cancellable<Vec<Diagnostic>> {
        self.query(|db| diagnostics::diagnostics(db, file_id))
    }

    pub fn view_syntax_tree(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| view_syntax_tree::view_syntax_tree(db, file_id))
    }

    /// Helper method to handle Salsa cancellations.
    fn query<F, T>(&self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&Database) -> T + std::panic::UnwindSafe,
    {
        salsa::Cancelled::catch(|| f(&self.db))
    }
}

impl std::panic::RefUnwindSafe for AnalysisSnapshot {}
