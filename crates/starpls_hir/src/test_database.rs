use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use rustc_hash::FxHashMap;
use starpls_bazel::APIContext;
use starpls_bazel::Builtins;
use starpls_common::File;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::LoadItemCandidate;
use starpls_common::ResolvedPath;
use starpls_syntax::TextRange;
use starpls_syntax::TextSize;
use starpls_test_util::make_test_builtins;
use starpls_test_util::FixtureFile;
use starpls_test_util::FixtureType;

use crate::BuiltinDefs;
use crate::Db;
use crate::Dialect;
use crate::GlobalContext;
use crate::InferenceOptions;

#[derive(Default)]
#[salsa::db(starpls_common::Jar, crate::Jar)]
pub(crate) struct TestDatabase {
    builtin_defs: Arc<DashMap<Dialect, BuiltinDefs>>,
    dialect_registry: starpls_common::DialectRegistry,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    prelude_file: Option<FileId>,
    all_workspace_targets: Arc<Vec<String>>,
    pub(crate) gcx: Arc<GlobalContext>,
}

impl salsa::Database for TestDatabase {}

impl starpls_common::Db for TestDatabase {
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
        _path: &str,
        _dialect: Dialect,
        _from: FileId,
    ) -> anyhow::Result<Option<File>> {
        Ok(Some(File::new(
            self,
            FileId(0),
            Dialect::Standard,
            None,
            String::new(),
        )))
    }

    fn get_file(&self, file_id: FileId) -> Option<File> {
        self.files.get(&file_id).map(|file| *file)
    }

    fn list_load_candidates(
        &self,
        _path: &str,
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

    fn resolve_build_file(&self, _file_id: FileId) -> Option<String> {
        None
    }
}

impl crate::Db for TestDatabase {
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

    fn gcx(&self) -> &GlobalContext {
        &self.gcx
    }

    fn set_all_workspace_targets(&mut self, targets: Vec<String>) {
        self.all_workspace_targets = Arc::new(targets)
    }

    fn get_all_workspace_targets(&self) -> Arc<Vec<String>> {
        Arc::clone(&self.all_workspace_targets)
    }

    fn get_dialect_registry(&self) -> &starpls_common::DialectRegistry {
        &self.dialect_registry
    }

    fn get_dialect_registry_mut(&mut self) -> &mut starpls_common::DialectRegistry {
        &mut self.dialect_registry
    }

    fn register_dialect(&mut self, dialect: starpls_common::ExtensibleDialect) {
        self.dialect_registry.register(dialect);
    }

    fn get_builtin_defs_by_id(&self, dialect_id: &starpls_common::DialectId, api_context: Option<starpls_bazel::APIContext>) -> BuiltinDefs {
        if let Some(provider) = self.dialect_registry.builtin_provider(dialect_id) {
            let builtins = provider.load_builtins(api_context).unwrap_or_default();
            let rules = provider.load_rules(api_context).unwrap_or_default();
            BuiltinDefs::new(self, builtins, rules)
        } else {
            BuiltinDefs::new(self, Builtins::default(), Builtins::default())
        }
    }
}

#[allow(unused)]
#[derive(Default)]
pub(crate) struct TestDatabaseBuilder {
    options: InferenceOptions,
    functions: Vec<String>,
    globals: Vec<(String, String)>,
    types: Vec<FixtureType>,
}

#[allow(unused)]
impl TestDatabaseBuilder {
    pub fn add_function(&mut self, name: impl Into<String>) {
        self.functions.push(name.into());
    }

    pub fn add_global(&mut self, name: impl Into<String>, ty: impl Into<String>) {
        self.globals.push((name.into(), ty.into()));
    }

    pub fn add_type(&mut self, ty: FixtureType) {
        self.types.push(ty);
    }

    pub fn set_inference_options(&mut self, options: InferenceOptions) {
        self.options = options;
    }

    pub fn build(self) -> TestDatabase {
        let mut db = TestDatabase {
            gcx: Arc::new(GlobalContext::new(self.options)),
            ..Default::default()
        };
        db.set_builtin_defs(
            Dialect::Bazel,
            make_test_builtins(self.functions, self.globals, self.types),
            Builtins::default(),
        );
        db
    }
}

pub struct Fixture {
    pub path_to_file_id: FxHashMap<PathBuf, FileId>,
    pub selected_ranges: Vec<(FileId, TextRange)>,
    pub cursor_pos: Option<(FileId, TextSize)>,
    next_file_id: u32,
}

impl Fixture {
    pub fn new(db: &mut dyn Db) -> Self {
        let fixture = Self {
            path_to_file_id: Default::default(),
            selected_ranges: Default::default(),
            cursor_pos: None,
            next_file_id: 0,
        };

        // Add builtins here as needed for tests.
        // TODO(withered-magic): Make this a little bit nicer.
        let functions = vec!["provider", "rule", "struct"];
        let globals = vec![("attr", "attr")];
        let types = vec![FixtureType::new("attr", vec![], vec!["int", "string"])];
        db.set_builtin_defs(
            Dialect::Bazel,
            make_test_builtins(functions, globals, types),
            Builtins::default(),
        );

        fixture
    }

    /// Provides a convenient way to quickly construct a fixture from a single file, as is commonly
    /// needed by tests.
    pub fn from_single_file(db: &mut dyn Db, contents: &str) -> (Self, FileId) {
        let mut fixture = Self::new(db);
        let file_id = fixture.add_file(db, "main.bzl", contents);
        (fixture, file_id)
    }

    pub fn add_file(&mut self, db: &mut dyn Db, path: impl AsRef<Path>, contents: &str) -> FileId {
        self.add_file_with_options(
            db,
            path,
            contents,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Bzl,
                is_external: false,
            }),
        )
    }

    pub fn add_prelude_file(&mut self, db: &mut dyn Db, contents: &str) -> FileId {
        let file_id = self.add_file_with_options(
            db,
            "tools/build_rules/prelude_bazel",
            contents,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Prelude,
                is_external: false,
            }),
        );
        db.set_bazel_prelude_file(file_id);
        file_id
    }

    pub fn add_file_with_options(
        &mut self,
        db: &mut dyn Db,
        path: impl AsRef<Path>,
        contents: &str,
        dialect: Dialect,
        info: Option<FileInfo>,
    ) -> FileId {
        let fixture = FixtureFile::parse(contents);
        let file_id = FileId(self.next_file_id);
        self.next_file_id += 1;
        self.path_to_file_id
            .insert(path.as_ref().to_path_buf(), file_id);
        db.create_file(file_id, dialect, info, fixture.contents);

        if let Some(cursor_pos) = fixture.cursor_pos {
            if self.cursor_pos.is_some() {
                panic!("cannot have more than one cursor_pos");
            }
            self.cursor_pos = Some((file_id, cursor_pos));
        }
        self.selected_ranges.extend(
            fixture
                .selected_ranges
                .into_iter()
                .map(|range| (file_id, range)),
        );

        file_id
    }
}
