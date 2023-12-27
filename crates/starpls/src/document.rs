use indexmap::IndexSet;
use rustc_hash::FxHasher;
use starpls_common::FileId;
use std::{collections::HashMap, hash::BuildHasherDefault, mem, path::PathBuf};

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
    documents: HashMap<FileId, Document>,
    has_closed_or_opened_documents: bool,
    changed_file_ids: Vec<FileId>,
    path_interner: PathInterner,
}

impl DocumentManager {
    pub(crate) fn open(&mut self, path: PathBuf, version: i32, contents: String) {
        self.has_closed_or_opened_documents = true;

        // Create/update the document with the given contents.
        let file_id = self.path_interner.intern_path(path);
        self.documents
            .insert(file_id, Document::new(contents, Some(version)));
        self.changed_file_ids.push(file_id);
    }

    pub(crate) fn close(&mut self, path: &PathBuf) {
        self.has_closed_or_opened_documents = true;

        let file_id = self.path_interner.intern_path(path.clone());
        if let Some(document) = self.documents.get_mut(&file_id) {
            document.source = DocumentSource::Disk;
        };
    }

    pub(crate) fn modify(&mut self, path: PathBuf, contents: String, version: Option<i32>) {
        let file_id = self.path_interner.intern_path(path);
        if let Some(document) = self.documents.get_mut(&file_id) {
            document.contents = contents;
            document.source = version.into();
            self.changed_file_ids.push(file_id);
        };
    }

    pub(crate) fn take_changes(&mut self) -> (bool, Vec<FileId>) {
        let changed_documents = mem::take(&mut self.changed_file_ids);
        let has_opened_or_closed_documents = self.has_closed_or_opened_documents;
        self.has_closed_or_opened_documents = false;
        (has_opened_or_closed_documents, changed_documents)
    }

    pub(crate) fn contents(&self, file_id: FileId) -> Option<&str> {
        self.get(file_id).map(|document| document.contents.as_str())
    }

    pub(crate) fn get(&self, file_id: FileId) -> Option<&Document> {
        self.documents.get(&file_id)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&FileId, &Document)> {
        self.documents.iter()
    }

    pub(crate) fn lookup_by_file_id(&self, file_id: FileId) -> &PathBuf {
        self.path_interner.lookup_by_file_id(file_id)
    }

    pub(crate) fn lookup_by_path_buf(&self, path: &PathBuf) -> Option<FileId> {
        self.path_interner.lookup_by_path_buf(path)
    }
}

#[derive(Default)]
pub struct PathInterner {
    map: IndexSet<PathBuf, BuildHasherDefault<FxHasher>>,
}

impl PathInterner {
    pub(crate) fn intern_path(&mut self, path: PathBuf) -> FileId {
        let index = self.map.insert_full(path).0;
        FileId(index as u32)
    }

    pub(crate) fn lookup_by_path_buf(&self, path: &PathBuf) -> Option<FileId> {
        self.map
            .get_index_of(path)
            .map(|index| FileId(index as u32))
    }

    pub(crate) fn lookup_by_file_id(&self, file_id: FileId) -> &PathBuf {
        self.map
            .get_index(file_id.0 as usize)
            .expect("unknown file_id")
    }
}
