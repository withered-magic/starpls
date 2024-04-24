use std::sync::Arc;

use dashmap::{mapref::entry::Entry, DashMap};
use starpls_bazel::{APIContext, Builtins};
use starpls_common::{File, FileId, LoadItemCandidate, ResolvedPath};
use starpls_test_util::{make_test_builtins, FixtureType};

use crate::{def::ExprId, BuiltinDefs, Db, Dialect, GlobalCtxt, LoadItemId, ParamId, Ty};

#[derive(Default)]
#[salsa::db(starpls_common::Jar, crate::Jar)]
pub(crate) struct TestDatabase {
    builtin_defs: Arc<DashMap<Dialect, BuiltinDefs>>,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    pub(crate) gcx: Arc<GlobalCtxt>,
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

    fn resolve_load_stmt(&self, _file: File, _load_stmt: crate::def::LoadStmt) -> Option<File> {
        None
    }

    fn resolve_call_expr_active_param(
        &self,
        _file: File,
        _expr: ExprId,
        _active_arg: usize,
    ) -> Option<usize> {
        None
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

#[allow(unused)]
#[derive(Default)]
pub(crate) struct TestDatabaseBuilder {
    functions: Vec<String>,
    types: Vec<FixtureType>,
}

#[allow(unused)]
impl TestDatabaseBuilder {
    pub fn add_function(&mut self, name: impl Into<String>) {
        self.functions.push(name.into());
    }

    pub fn add_type(&mut self, ty: FixtureType) {
        self.types.push(ty);
    }

    pub fn build(self) -> TestDatabase {
        let mut db = TestDatabase::default();
        db.set_builtin_defs(
            Dialect::Bazel,
            make_test_builtins(self.functions, self.types),
            Builtins::default(),
        );
        db
    }
}
