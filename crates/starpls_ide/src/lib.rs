use std::fmt::Debug;
use std::panic;
use std::path::PathBuf;
use std::sync::Arc;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use salsa::ParallelDatabase;
use starpls_bazel::APIContext;
use starpls_bazel::Builtins;
use starpls_common::Db;
use starpls_common::Diagnostic;
use starpls_common::Dialect;
use starpls_common::File;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::LoadItemCandidate;
use starpls_common::ResolvedPath;
use starpls_hir::BuiltinDefs;
pub use starpls_hir::Cancelled;
use starpls_hir::Db as _;
#[cfg(test)]
use starpls_hir::Fixture;
use starpls_hir::GlobalContext;
pub use starpls_hir::InferenceOptions;
use starpls_syntax::LineIndex;
use starpls_syntax::TextRange;
use starpls_syntax::TextSize;

pub use crate::completions::CompletionItem;
pub use crate::completions::CompletionItemKind;
pub use crate::completions::CompletionMode;
pub use crate::completions::Edit;
pub use crate::completions::InsertReplaceEdit;
pub use crate::completions::TextEdit;
pub use crate::document_symbols::DocumentSymbol;
pub use crate::document_symbols::SymbolKind;
pub use crate::document_symbols::SymbolTag;
pub use crate::hover::Hover;
pub use crate::hover::Markup;
pub use crate::signature_help::ParameterInfo;
pub use crate::signature_help::SignatureHelp;
pub use crate::signature_help::SignatureInfo;

mod completions;
mod diagnostics;
mod document_symbols;
mod find_references;
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
    gcx: Arc<GlobalContext>,
    prelude_file: Option<FileId>,
    all_workspace_targets: Arc<Vec<String>>,
}

impl Database {
    fn apply_file_changes(&mut self, changes: Vec<(FileId, FileChange)>) {
        let gcx = self.gcx.clone();
        let _guard = gcx.cancel();
        for (file_id, change) in changes {
            match change {
                FileChange::Create {
                    dialect,
                    info,
                    contents,
                } => {
                    self.create_file(file_id, dialect, info, contents);
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
            prelude_file: self.prelude_file,
            all_workspace_targets: self.all_workspace_targets.clone(),
        })
    }
}

impl starpls_common::Db for Database {
    fn create_file(
        &mut self,
        file_id: FileId,
        dialect: Dialect,
        info: Option<FileInfo>,
        contents: String,
    ) -> File {
        let file = File::new(self, file_id, dialect, info, contents);
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
        let res = match self.loader.load_file(path, dialect, from)? {
            Some(res) => res,
            None => return Ok(None),
        };
        Ok(Some(match self.files.entry(res.file_id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => *entry.insert(File::new(
                self,
                res.file_id,
                dialect,
                res.info,
                res.contents.unwrap_or_default(),
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
            if let Entry::Vacant(entry) = self.files.entry(build_file) {
                entry.insert(File::new(
                    self,
                    build_file,
                    Dialect::Bazel,
                    Some(FileInfo::Bazel {
                        api_context: APIContext::Build,
                        is_external: false,
                    }),
                    contents.take().unwrap_or_default(),
                ));
            }
        }

        Ok(Some(resolved_path))
    }

    fn resolve_build_file(&self, file_id: FileId) -> Option<String> {
        self.loader.resolve_build_file(file_id)
    }
}

impl starpls_hir::Db for Database {
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

    fn set_bazel_prelude_file(&mut self, file_id: FileId) {
        self.prelude_file = Some(file_id)
    }

    fn get_bazel_prelude_file(&self) -> Option<FileId> {
        self.prelude_file
    }

    fn set_all_workspace_targets(&mut self, targets: Vec<String>) {
        self.all_workspace_targets = Arc::new(targets)
    }

    fn get_all_workspace_targets(&self) -> Arc<Vec<String>> {
        Arc::clone(&self.all_workspace_targets)
    }

    fn gcx(&self) -> &GlobalContext {
        &self.gcx
    }
}

#[derive(Debug)]
enum FileChange {
    Create {
        dialect: Dialect,
        info: Option<FileInfo>,
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
        info: Option<FileInfo>,
        contents: String,
    ) {
        self.changed_files.push((
            file_id,
            FileChange::Create {
                dialect,
                info,
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
    pub fn new(loader: Arc<dyn FileLoader>, options: InferenceOptions) -> Self {
        Self {
            db: Database {
                builtin_defs: Default::default(),
                files: Default::default(),
                gcx: Arc::new(GlobalContext::new(options)),
                storage: Default::default(),
                loader,
                prelude_file: None,
                all_workspace_targets: Arc::default(),
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

    pub fn set_bazel_prelude_file(&mut self, file_id: FileId) {
        self.db.set_bazel_prelude_file(file_id);
    }

    pub fn set_all_workspace_targets(&mut self, targets: Vec<String>) {
        self.db.set_all_workspace_targets(targets);
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> (Analysis, Arc<SimpleFileLoader>) {
        let loader = Arc::new(SimpleFileLoader::default());
        let analysis = Analysis::new(loader.clone(), Default::default());
        (analysis, loader)
    }

    #[cfg(test)]
    pub(crate) fn from_single_file_fixture(fixture: &str) -> (Analysis, Fixture) {
        let (mut analysis, loader) = Self::new_for_test();
        let (fixture, _) = Fixture::from_single_file(&mut analysis.db, fixture);
        loader.add_files_from_fixture(&analysis.db, &fixture);
        (analysis, fixture)
    }
}

pub struct AnalysisSnapshot {
    db: salsa::Snapshot<Database>,
}

impl AnalysisSnapshot {
    pub fn completions(
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

    pub fn find_references(&self, pos: FilePosition) -> Cancellable<Option<Vec<Location>>> {
        self.query(|db| find_references::find_references(db, pos))
    }

    pub fn goto_definition(
        &self,
        pos: FilePosition,
        skip_re_exports: bool,
    ) -> Cancellable<Option<Vec<LocationLink>>> {
        self.query(|db| goto_definition::goto_definition(db, pos, skip_re_exports))
    }

    pub fn hover(&self, pos: FilePosition) -> Cancellable<Option<Hover>> {
        self.query(|db| hover::hover(db, pos))
    }

    pub fn line_index(&self, file_id: FileId) -> Cancellable<Option<&LineIndex>> {
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
pub struct Location {
    pub file_id: FileId,
    pub range: TextRange,
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadFileResult {
    pub file_id: FileId,
    pub dialect: Dialect,
    pub info: Option<FileInfo>,
    pub contents: Option<String>,
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
    ) -> anyhow::Result<Option<LoadFileResult>>;

    /// Returns a list of Starlark modules that can be loaded from the given `path`.
    fn list_load_candidates(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>>;

    /// If the specified file is a BUILD file, returns its package.
    fn resolve_build_file(&self, file_id: FileId) -> Option<String>;
}

/// Simple implementation of [`FileLoader`] backed by a HashMap.
/// Mainly used for tests.
#[derive(Default)]
pub(crate) struct SimpleFileLoader(DashMap<String, LoadFileResult>);

impl SimpleFileLoader {
    #[cfg(test)]
    pub(crate) fn add_files_from_fixture(&self, db: &dyn Db, fixture: &Fixture) {
        for (path, file_id) in &fixture.path_to_file_id {
            let file = db.get_file(*file_id).unwrap();
            self.0.insert(
                path.to_string_lossy().to_string(),
                LoadFileResult {
                    file_id: *file_id,
                    dialect: file.dialect(db),
                    info: file.info(db),
                    contents: Some(file.contents(db).clone()),
                },
            );
        }
    }
}

impl FileLoader for SimpleFileLoader {
    fn resolve_path(
        &self,
        _path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>> {
        Ok(None)
    }

    fn load_file(
        &self,
        path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<LoadFileResult>> {
        Ok(self.0.get(path).map(|res| res.clone()))
    }

    fn list_load_candidates(
        &self,
        _path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>> {
        Ok(None)
    }

    fn resolve_build_file(&self, _file_id: FileId) -> Option<String> {
        None
    }
}
