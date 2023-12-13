use crate::utils::Edit;
use rustc_hash::FxHashMap;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    mem,
    path::PathBuf,
    rc::Rc,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum DocumentSource {
    Editor(i32),
    Disk,
}

impl From<Option<i32>> for DocumentSource {
    fn from(value: Option<i32>) -> Self {
        value.map(Self::Editor).unwrap_or(Self::Disk)
    }
}

/// Represents an active text document. Text documents may either be sourced from disk
/// or from the editor.
pub(crate) struct Document {
    pub(crate) contents: String,
    pub(crate) source: DocumentSource,
}

impl Document {
    fn new(contents: String, version: Option<i32>) -> Self {
        Self {
            contents,
            source: version.into(),
        }
    }
}

/// A collection of documents.
#[derive(Default)]
pub(crate) struct DocumentManager {
    documents: HashMap<u32, Document>,
    has_closed_or_opened_documents: bool,
    changed_documents: Vec<(u32, Edit)>,
    path_interner: PathInterner,
}

impl DocumentManager {
    pub(crate) fn open(&mut self, path: PathBuf, version: i32, contents: String) {
        self.has_closed_or_opened_documents = true;

        // Create/update the document with the given contents.
        let file_id = self.path_interner.intern_path(path);
        self.documents
            .insert(file_id, Document::new(contents, Some(version)));
        self.changed_documents.push((file_id, Edit::Full));
    }

    pub(crate) fn close(&mut self, path: &PathBuf) {
        self.has_closed_or_opened_documents = true;

        let file_id = self.path_interner.intern_path(path.clone());
        if let Some(document) = self.documents.get_mut(&file_id) {
            document.source = DocumentSource::Disk;
        };
    }

    pub(crate) fn modify(
        &mut self,
        path: PathBuf,
        contents: String,
        version: Option<i32>,
        edits: Edit,
    ) {
        let file_id = self.path_interner.intern_path(path);
        if let Some(document) = self.documents.get_mut(&file_id) {
            document.contents = contents;
            document.source = version.into();
            self.changed_documents.push((file_id, edits));
        };
    }

    pub(crate) fn take_changes(&mut self) -> (bool, Vec<(u32, Edit)>) {
        let changed_documents = mem::take(&mut self.changed_documents);
        let has_opened_or_closed_documents = self.has_closed_or_opened_documents;
        self.has_closed_or_opened_documents = false;
        (has_opened_or_closed_documents, changed_documents)
    }

    pub(crate) fn contents(&self, file_id: u32) -> Option<&str> {
        self.get(file_id).map(|document| document.contents.as_str())
    }

    pub(crate) fn get(&self, file_id: u32) -> Option<&Document> {
        self.documents.get(&file_id)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&u32, &Document)> {
        self.documents.iter()
    }

    pub(crate) fn lookup_path(&self, file_id: u32) -> Option<Rc<PathBuf>> {
        self.path_interner.lookup_path(file_id)
    }

    pub(crate) fn lookup_file_id(&self, path: &PathBuf) -> Option<u32> {
        self.path_interner.lookup_file_id(path)
    }
}

#[derive(Default)]
pub struct PathInterner {
    seq: Cell<u32>,
    key_map: RefCell<FxHashMap<PathBuf, u32>>,
    value_map: RefCell<FxHashMap<u32, Rc<PathBuf>>>,
}

impl PathInterner {
    pub(crate) fn intern_path(&self, path: PathBuf) -> u32 {
        let mut key_map = self.key_map.borrow_mut();

        // If this path has already been intered, return the corresponding file ID.
        if let Some(file_id) = key_map.get(&path) {
            return *file_id;
        }

        // Otherwise, intern the path and return the newly allocated identifier.
        let file_id = self.seq.replace(self.seq.get() + 1);
        key_map.insert(path.clone(), file_id);
        self.value_map.borrow_mut().insert(file_id, Rc::new(path));
        file_id
    }

    pub(crate) fn lookup_path(&self, file_id: u32) -> Option<Rc<PathBuf>> {
        self.value_map.borrow().get(&file_id).cloned()
    }

    pub(crate) fn lookup_file_id(&self, path: &PathBuf) -> Option<u32> {
        self.key_map.borrow().get(path).copied()
    }
}
