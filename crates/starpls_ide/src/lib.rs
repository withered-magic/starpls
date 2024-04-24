use dashmap::{mapref::entry::Entry, DashMap};
use rustc_hash::FxHashMap;
use salsa::ParallelDatabase;
use starpls_bazel::{APIContext, Builtins};
use starpls_common::{Db, Diagnostic, Dialect, File, FileId, LoadItemCandidate, ResolvedPath};
use starpls_hir::{BuiltinDefs, Db as _, ExprId, GlobalCtxt, LoadItemId, LoadStmt, ParamId, Ty};
use starpls_syntax::{LineIndex, TextRange, TextSize};
use starpls_test_util::make_test_builtins;
use std::{fmt::Debug, panic, path::PathBuf, sync::Arc};

pub use crate::{
    completions::{CompletionItem, CompletionItemKind, CompletionMode},
    document_symbols::{DocumentSymbol, SymbolKind, SymbolTag},
    hover::{Hover, Markup},
    signature_help::{ParameterInfo, SignatureHelp, SignatureInfo},
};
pub use starpls_hir::Cancelled;

mod completions;
mod diagnostics;
mod document_symbols;
mod goto_definition;
mod hover;
mod line_index;
mod show_hir;
mod show_syntax_tree;
mod signature_help;
mod util;

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
                FileChange::Create {
                    dialect,
                    api_context,
                    contents,
                } => {
                    self.create_file(file_id, dialect, api_context, contents);
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
    fn create_file(
        &mut self,
        file_id: FileId,
        dialect: Dialect,
        api_context: Option<APIContext>,
        contents: String,
    ) -> File {
        let file = File::new(self, file_id, dialect, api_context, contents);
        self.files.insert(file_id, file);
        file
    }

    fn update_file(&mut self, file_id: FileId, contents: String) {
        if let Some(file) = self.files.get(&file_id).map(|file_id| *file_id) {
            file.set_contents(self).to(contents);
        }
    }

    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<File>> {
        let (file_id, dialect, api_context, contents) =
            match self.loader.load_file(path, dialect, from)? {
                Some(res) => res,
                None => return Ok(None),
            };
        Ok(Some(match self.files.entry(file_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(File::new(
                self,
                file_id,
                dialect,
                api_context,
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
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>> {
        let dialect = match self.get_file(from) {
            Some(file) => file.dialect(self),
            None => return Ok(None),
        };
        self.loader.list_load_candidates(path, dialect, from)
    }

    fn resolve_path(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>> {
        let mut resolved_path = match self.loader.resolve_path(path, dialect, from)? {
            Some(resolved_path) => resolved_path,
            None => return Ok(None),
        };

        if let ResolvedPath::BuildTarget {
            build_file,
            ref mut contents,
            ..
        } = resolved_path
        {
            match self.files.entry(build_file) {
                Entry::Vacant(entry) => {
                    entry.insert(File::new(
                        self,
                        build_file,
                        Dialect::Bazel,
                        Some(APIContext::Bzl),
                        contents.take().unwrap_or_default(),
                    ));
                }
                _ => {}
            };
        }

        Ok(Some(resolved_path))
    }
}

impl starpls_hir::Db for Database {
    fn infer_expr(&self, file: File, expr: ExprId) -> Ty {
        self.gcx.with_tcx(self, |tcx| tcx.infer_expr(file, expr))
    }

    fn infer_param(&self, file: File, param: ParamId) -> Ty {
        self.gcx.with_tcx(self, |tcx| tcx.infer_param(file, param))
    }

    fn infer_load_item(&self, file: File, load_item: LoadItemId) -> Ty {
        self.gcx
            .with_tcx(self, |tcx| tcx.infer_load_item(file, load_item))
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

    fn set_builtin_defs(&mut self, dialect: Dialect, builtins: Builtins, rules: Builtins) {
        let defs = match self.builtin_defs.entry(dialect) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(BuiltinDefs::new(self, builtins, rules));
                return;
            }
        };
        defs.set_builtins(self).to(builtins);
    }

    fn get_builtin_defs(&self, dialect: &Dialect) -> BuiltinDefs {
        self.builtin_defs
            .get(dialect)
            .map(|defs| *defs)
            .unwrap_or(BuiltinDefs::new(
                self,
                Builtins::default(),
                Builtins::default(),
            ))
    }
}

#[derive(Debug)]
enum FileChange {
    Create {
        dialect: Dialect,
        api_context: Option<APIContext>,
        contents: String,
    },
    Update {
        contents: String,
    },
}

/// A batch of changes to be applied to the database. For now, this consists simply of a map of changed file IDs to
/// their updated contents.
#[derive(Debug, Default)]
pub struct Change {
    changed_files: Vec<(FileId, FileChange)>,
}

impl Change {
    pub fn create_file(
        &mut self,
        file_id: FileId,
        dialect: Dialect,
        api_context: Option<APIContext>,
        contents: String,
    ) {
        self.changed_files.push((
            file_id,
            FileChange::Create {
                dialect,
                api_context,
                contents,
            },
        ))
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

    pub fn set_builtin_defs(&mut self, builtins: Builtins, rules: Builtins) {
        self.db.set_builtin_defs(Dialect::Bazel, builtins, rules);
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
}

impl AnalysisSnapshot {
    pub fn from_single_file(
        contents: &str,
        dialect: Dialect,
        api_context: Option<APIContext>,
    ) -> (Self, FileId) {
        let mut file_set = FxHashMap::default();
        let file_id = FileId(0);
        file_set.insert("main.star".to_string(), (file_id, contents.to_string()));
        let mut change = Change::default();
        change.create_file(file_id, dialect, api_context, contents.to_string());
        let mut analysis = Analysis::new(Arc::new(SimpleFileLoader::from_file_set(file_set)));
        analysis.db.set_builtin_defs(
            Dialect::Bazel,
            make_test_builtins(vec!["provider".to_string(), "struct".to_string()], vec![]),
            Builtins::default(),
        );
        analysis.apply_change(change);
        (analysis.snapshot(), file_id)
    }

    pub fn completion(
        &self,
        pos: FilePosition,
        trigger_character: Option<String>,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.query(|db| completions::completions(db, pos, trigger_character))
    }

    pub fn diagnostics(&self, file_id: FileId) -> Cancellable<Vec<Diagnostic>> {
        self.query(|db| diagnostics::diagnostics(db, file_id))
    }

    pub fn document_symbols(&self, file_id: FileId) -> Cancellable<Option<Vec<DocumentSymbol>>> {
        self.query(|db| document_symbols::document_symbols(db, file_id))
    }

    pub fn goto_definition(&self, pos: FilePosition) -> Cancellable<Option<Vec<LocationLink>>> {
        self.query(|db| {
            let res = goto_definition::goto_definition(db, pos);
            res
        })
    }

    pub fn hover(&self, pos: FilePosition) -> Cancellable<Option<Hover>> {
        self.query(|db| hover::hover(db, pos))
    }

    pub fn line_index<'a>(&'a self, file_id: FileId) -> Cancellable<Option<&'a LineIndex>> {
        self.query(move |db| line_index::line_index(db, file_id))
    }

    pub fn show_hir(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| show_hir::show_hir(db, file_id))
    }

    pub fn show_syntax_tree(&self, file_id: FileId) -> Cancellable<Option<String>> {
        self.query(|db| show_syntax_tree::show_syntax_tree(db, file_id))
    }

    pub fn signature_help(&self, pos: FilePosition) -> Cancellable<Option<SignatureHelp>> {
        self.query(|db| signature_help::signature_help(db, pos))
    }

    /// Helper method to handle Salsa cancellations.
    fn query<'a, F, T>(&'a self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&'a Database) -> T + panic::UnwindSafe,
    {
        starpls_hir::Cancelled::catch(|| f(&self.db))
    }
}

impl panic::RefUnwindSafe for AnalysisSnapshot {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocationLink {
    Local {
        origin_selection_range: Option<TextRange>,
        target_range: TextRange,
        target_selection_range: TextRange,
        target_file_id: FileId,
    },
    External {
        origin_selection_range: Option<TextRange>,
        target_path: PathBuf,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilePosition {
    pub file_id: FileId,
    pub pos: TextSize,
}

/// A trait for loading a path and listing its exported symbols.
pub trait FileLoader: Send + Sync + 'static {
    fn resolve_path(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>>;

    /// Open the Starlark file corresponding to the given `path` and of the given `Dialect`.
    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<(FileId, Dialect, Option<APIContext>, Option<String>)>>;

    /// Returns a list of Starlark modules that can be loaded from the given `path`.
    fn list_load_candidates(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>>;
}

/// [`FileLoader`] that looks up files by path from a hash map.
pub(crate) struct SimpleFileLoader {
    file_set: FxHashMap<String, (FileId, String)>,
}

impl SimpleFileLoader {
    /// Creates a [`SimpleFileLoader`] from a static set of files.
    pub(crate) fn from_file_set(file_set: FxHashMap<String, (FileId, String)>) -> Self {
        Self { file_set }
    }
}

impl FileLoader for SimpleFileLoader {
    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<(FileId, Dialect, Option<APIContext>, Option<String>)>> {
        Ok(self
            .file_set
            .get(path)
            .map(|(file_id, contents)| (*file_id, dialect, None, Some(contents.clone()))))
    }

    fn list_load_candidates(
        &self,
        _path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>> {
        Ok(None)
    }

    fn resolve_path(
        &self,
        _path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>> {
        Ok(None)
    }
}
