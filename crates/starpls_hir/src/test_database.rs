use std::sync::Arc;

use dashmap::{mapref::entry::Entry, DashMap};
use starpls_bazel::Builtins;
use starpls_common::{File, FileId, FileInfo, LoadItemCandidate, ResolvedPath};
use starpls_test_util::{make_test_builtins, FixtureType};

use crate::{BuiltinDefs, Db, Dialect, GlobalContext, InferenceOptions};

#[derive(Default)]
#[salsa::db(starpls_common::Jar, crate::Jar)]
pub(crate) struct TestDatabase {
    builtin_defs: Arc<DashMap<Dialect, BuiltinDefs>>,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    prelude_file: Option<FileId>,
    pub(crate) gcx: Arc<GlobalContext>,
}

impl TestDatabase {
    #[allow(dead_code)]
    pub(crate) fn infer_all_exprs(&self, file: File) {
        self.gcx.with_tcx(self, |tcx| tcx.infer_all_exprs(file));
    }

    #[allow(dead_code)]
    pub(crate) fn infer_all_params(&self, file: File) {
        self.gcx.with_tcx(self, |tcx| tcx.infer_all_params(file));
    }
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
