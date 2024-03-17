use crate::{completion::CompletionItem, hover::Hover};
use dashmap::{mapref::entry::Entry, DashMap};
use salsa::ParallelDatabase;
use signature_help::SignatureHelp;
use starpls_bazel::Builtins;
use starpls_common::{Db, Diagnostic, Dialect, File, FileId, LoadItemCandidate};
use starpls_hir::{BuiltinDefs, Db as _, ExprId, GlobalCtxt, LoadStmt, ParamId, Ty};
use starpls_syntax::{LineIndex, TextRange, TextSize};
use std::fmt::Debug;
use std::io;
use std::sync::Arc;

pub use starpls_hir::Cancelled;

mod handlers;
mod util;

pub mod completion;
pub mod hover;
pub mod signature_help;

pub type Cancellable<T> = Result<T, Cancelled>;

#[salsa::db(starpls_common::Jar, starpls_hir::Jar)]
pub(crate) struct Database {
    builtin_defs: Arc<DashMap<Dialect, BuiltinDefs>>,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    loader: Arc<dyn FileLoader>,
    gcx: Arc<GlobalCtxt>,
}

impl Database {
    fn apply_file_changes(&mut self, changes: Vec<(FileId, FileChange)>) {
        let gcx = self.gcx.clone();
        let _guard = gcx.cancel();
        for (file_id, change) in changes {
            match change {
                FileChange::Create { dialect, contents } => {
                    self.create_file(file_id, dialect, contents);
                }
                FileChange::Update { contents } => {
                    self.update_file(file_id, contents);
                }
            }
        }
    }
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            builtin_defs: self.builtin_defs.clone(),
            files: self.files.clone(),
            gcx: self.gcx.clone(),
            loader: self.loader.clone(),
            storage: self.storage.snapshot(),
        })
    }
}

impl starpls_common::Db for Database {
    fn create_file(&mut self, file_id: FileId, dialect: Dialect, contents: String) -> File {
        let file = File::new(self, file_id, dialect, contents);
        self.files.insert(file_id, file);
        file
    }

    fn update_file(&mut self, file_id: FileId, contents: String) {
        if let Some(file) = self.files.get(&file_id).map(|file_id| *file_id) {
            file.set_contents(self).to(contents);
        }
    }

    fn load_file(&self, path: &str, dialect: Dialect, from: FileId) -> io::Result<Option<File>> {
        let (file_id, contents) = match self.loader.load_file(path, dialect, from)? {
            Some(res) => res,
            None => return Ok(None),
        };
        Ok(Some(match self.files.entry(file_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(File::new(
                self,
                file_id,
                dialect,
                contents.unwrap_or_default(),
            )),
        }))
    }

    fn get_file(&self, file_id: FileId) -> Option<File> {
        self.files.get(&file_id).map(|file| *file)
    }

    fn list_load_candidates(
        &self,
        path: &str,
        from: FileId,
    ) -> io::Result<Option<Vec<LoadItemCandidate>>> {
        let dialect = match self.get_file(from) {
            Some(file) => file.dialect(self),
            None => return Ok(None),
        };
        self.loader.list_load_candidates(path, dialect, from)
    }
}

impl starpls_hir::Db for Database {
    fn infer_expr(&self, file: File, expr: ExprId) -> Ty {
        self.gcx.with_tcx(self, |tcx| tcx.infer_expr(file, expr))
    }

    fn infer_param(&self, file: File, param: ParamId) -> Ty {
        self.gcx.with_tcx(self, |tcx| tcx.infer_param(file, param))
    }

    fn resolve_load_stmt(&self, file: File, load_stmt: LoadStmt) -> Option<File> {
        self.gcx
            .with_tcx(self, |tcx| tcx.resolve_load_stmt(file, load_stmt))
    }

    fn resolve_call_expr_active_param(
        &self,
        file: File,
        expr: ExprId,
        active_arg: usize,
    ) -> Option<usize> {
        self.gcx.with_tcx(self, |tcx| {
            tcx.resolve_call_expr_active_param(file, expr, active_arg)
        })
    }

    fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins) {
        let defs = match self.builtin_defs.entry(dialect) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(BuiltinDefs::new(self, builtins));
                return;
            }
        };
        defs.set_builtins(self).to(builtins);
    }

    fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs {
        self.builtin_defs
            .get(dialect)
            .map(|defs| *defs)
            .unwrap_or(BuiltinDefs::new(self, Builtins::default()))
    }
}

#[derive(Debug)]
enum FileChange {
    Create { dialect: Dialect, contents: String },
    Update { contents: String },
}

/// A batch of changes to be applied to the database. For now, this consists simply of a map of changed file IDs to
/// their updated contents.
#[derive(Debug, Default)]
pub struct Change {
    changed_files: Vec<(FileId, FileChange)>,
}

impl Change {
    pub fn create_file(&mut self, file_id: FileId, dialect: Dialect, contents: String) {
        self.changed_files
            .push((file_id, FileChange::Create { dialect, contents }))
    }

    pub fn update_file(&mut self, file_id: FileId, contents: String) {
        self.changed_files
            .push((file_id, FileChange::Update { contents }))
    }
}

/// Provides the main API for querying facts about the source code. This wraps the main `Database` struct.
pub struct Analysis {
    db: Database,
}

impl Analysis {
    pub fn new(loader: Arc<dyn FileLoader>) -> Self {
        Self {
            db: Database {
                builtin_defs: Default::default(),
                files: Default::default(),
                gcx: Default::default(),
                storage: Default::default(),
                loader,
            },
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

    pub fn set_builtin_defs(&mut self, builtins: Builtins) {
        self.db.set_builtin_defs(Dialect::Bazel, builtins);
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
}

impl AnalysisSnapshot {
    pub fn completion(
        &self,
        pos: FilePosition,
        trigger_character: Option<String>,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.query(|db| handlers::completion::completion(db, pos, trigger_character))
    }

    pub fn diagnostics(&self, file_id: FileId) -> Cancellable<Vec<Diagnostic>> {
        self.query(|db| handlers::diagnostics::diagnostics(db, file_id))
    }

    pub fn goto_definition(&self, pos: FilePosition) -> Cancellable<Option<Vec<Location>>> {
        self.query(|db| {
            let res = handlers::goto_definition::goto_definition(db, pos);
            res
        })
    }

    pub fn hover(&self, pos: FilePosition) -> Cancellable<Option<Hover>> {
        self.query(|db| handlers::hover::hover(db, pos))
    }

    pub fn line_index<'a>(&'a self, file_id: FileId) -> Cancellable<Option<&'a LineIndex>> {
        self.query(move |db| handlers::line_index::line_index(db, file_id))
    }

    pub fn show_hir(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| handlers::show_hir::show_hir(db, file_id))
    }

    pub fn show_syntax_tree(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| handlers::show_syntax_tree::show_syntax_tree(db, file_id))
    }

    pub fn signature_help(&self, pos: FilePosition) -> Cancellable<Option<SignatureHelp>> {
        self.query(|db| handlers::signature_help::signature_help(db, pos))
    }

    /// Helper method to handle Salsa cancellations.
    fn query<'a, F, T>(&'a self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&'a Database) -> T + std::panic::UnwindSafe,
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

pub trait FileLoader: Debug + Send + Sync + 'static {
    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> io::Result<Option<(FileId, Option<String>)>>;

    fn list_load_candidates(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> io::Result<Option<Vec<LoadItemCandidate>>>;
}
