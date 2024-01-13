use dashmap::{mapref::entry::Entry, DashMap};
use starpls_common::{File, FileId};
use std::sync::Arc;

use crate::{Ty, TyKind};

#[derive(Default)]
#[salsa::db(starpls_common::Jar, crate::Jar)]
pub(crate) struct TestDatabase {
    storage: salsa::Storage<Self>,
    files: Arc<DashMap<FileId, File>>,
}

impl salsa::Database for TestDatabase {}

impl starpls_common::Db for TestDatabase {
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

impl crate::Db for TestDatabase {
    fn infer_expr(&self, _file: File, _expr: crate::def::ExprId) -> Ty {
        TyKind::Any.intern()
    }
}
