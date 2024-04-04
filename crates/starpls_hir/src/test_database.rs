use crate::{def::ExprId, BuiltinDefs, Db, Dialect, GlobalCtxt, LoadItemId, ParamId, Ty};
use dashmap::{mapref::entry::Entry, DashMap};
use starpls_bazel::Builtins;
use starpls_common::{File, FileId, LoadItemCandidate};
use starpls_test_util::builtins_with_catch_all_functions;
use std::{io, sync::Arc};

#[derive(Default)]
#[salsa::db(starpls_common::Jar, crate::Jar)]
pub(crate) struct TestDatabase {
    builtin_defs: Arc<DashMap<Dialect, BuiltinDefs>>,
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
    pub(crate) gcx: Arc<GlobalCtxt>,
}

impl TestDatabase {
    #[allow(unused)]
    pub(crate) fn with_catch_all_functions(names: &[&str]) -> Self {
        let mut db = Self::default();
        db.set_builtin_defs(
            Dialect::Bazel,
            builtins_with_catch_all_functions(names),
            Builtins::default(),
        );
        db
    }

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

    fn load_file(&self, _path: &str, _dialect: Dialect, _from: FileId) -> io::Result<Option<File>> {
        Ok(Some(File::new(
            self,
            FileId(0),
            Dialect::Standard,
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
    ) -> io::Result<Option<Vec<LoadItemCandidate>>> {
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
