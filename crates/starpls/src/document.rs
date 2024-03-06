use indexmap::IndexSet;
use parking_lot::RwLock;
use rustc_hash::FxHasher;
use starpls_bazel::{self, label::RepoKind, Label};
use starpls_common::{Dialect, FileId, LoadItemCandidate, LoadItemCandidateKind};
use starpls_ide::FileLoader;
use std::{
    collections::HashMap,
    fs,
    hash::BuildHasherDefault,
    io, mem,
    path::{PathBuf, MAIN_SEPARATOR},
    sync::Arc,
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
    pub(crate) dialect: Dialect,
    pub(crate) source: DocumentSource,
}

impl Document {
    fn new(contents: String, dialect: Dialect, version: Option<i32>) -> Self {
        Self {
            contents,
            dialect,
            source: version.into(),
        }
    }
}

pub(crate) enum DocumentChangeKind {
    Create,
    Update,
}

/// A collection of documents.
pub(crate) struct DocumentManager {
    documents: HashMap<FileId, Document>,
    has_closed_or_opened_documents: bool,
    changed_file_ids: Vec<(FileId, DocumentChangeKind)>,
    path_interner: Arc<PathInterner>,
}

impl DocumentManager {
    pub(crate) fn new(path_interner: Arc<PathInterner>) -> Self {
        Self {
            documents: Default::default(),
            has_closed_or_opened_documents: false,
            changed_file_ids: Default::default(),
            path_interner,
        }
    }

    pub(crate) fn open(&mut self, path: PathBuf, version: i32, contents: String) {
        self.has_closed_or_opened_documents = true;

        // Create/update the document with the given contents.
        let basename = match path.file_name().and_then(|name| name.to_str()) {
            Some(basename) => basename,
            None => return,
        };

        let dialect = match basename {
            "BUILD" | "BUILD.bazel" | "MODULE.bazel" | "REPO.bazel" | "WORKSPACE"
            | "WORKSPACE.bazel" => Dialect::Bazel,
            _ => match path.extension().and_then(|ext| ext.to_str()) {
                Some("sky" | "star") => Dialect::Standard,
                Some("bzl") => Dialect::Bazel,
                _ => return,
            },
        };

        let file_id = self.path_interner.intern_path(path);
        self.documents
            .insert(file_id, Document::new(contents, dialect, Some(version)));
        self.changed_file_ids
            .push((file_id, DocumentChangeKind::Create));
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
            self.changed_file_ids
                .push((file_id, DocumentChangeKind::Update));
        };
    }

    pub(crate) fn take_changes(&mut self) -> (bool, Vec<(FileId, DocumentChangeKind)>) {
        let changed_documents = mem::take(&mut self.changed_file_ids);
        let has_opened_or_closed_documents = self.has_closed_or_opened_documents;
        self.has_closed_or_opened_documents = false;
        (has_opened_or_closed_documents, changed_documents)
    }

    pub(crate) fn get(&self, file_id: FileId) -> Option<&Document> {
        self.documents.get(&file_id)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&FileId, &Document)> {
        self.documents.iter()
    }

    pub(crate) fn lookup_by_file_id(&self, file_id: FileId) -> PathBuf {
        self.path_interner.lookup_by_file_id(file_id)
    }

    pub(crate) fn lookup_by_path_buf(&self, path: &PathBuf) -> Option<FileId> {
        self.path_interner.lookup_by_path_buf(path)
    }
}

#[derive(Default, Debug)]
pub(crate) struct PathInterner {
    map: RwLock<IndexSet<PathBuf, BuildHasherDefault<FxHasher>>>,
}

impl PathInterner {
    pub(crate) fn intern_path(&self, path: PathBuf) -> FileId {
        let index = self.map.write().insert_full(path).0;
        FileId(index as u32)
    }

    pub(crate) fn lookup_by_path_buf(&self, path: &PathBuf) -> Option<FileId> {
        self.map
            .read()
            .get_index_of(path)
            .map(|index| FileId(index as u32))
    }

    pub(crate) fn lookup_by_file_id(&self, file_id: FileId) -> PathBuf {
        self.map
            .read()
            .get_index(file_id.0 as usize)
            .expect("unknown file_id")
            .clone()
    }
}

#[derive(Debug)]
pub(crate) struct DefaultFileLoader {
    interner: Arc<PathInterner>,
}

impl DefaultFileLoader {
    pub(crate) fn new(interner: Arc<PathInterner>) -> Self {
        Self { interner }
    }
}

impl DefaultFileLoader {
    fn dirname(&self, file_id: FileId) -> PathBuf {
        // Find the importing file's directory.
        let mut from_path = self.interner.lookup_by_file_id(file_id);
        assert!(from_path.pop());
        from_path
    }
}

impl FileLoader for DefaultFileLoader {
    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> io::Result<(FileId, Option<String>)> {
        let path = match dialect {
            Dialect::Standard => {
                // Find the importing file's directory.
                let mut from_path = self.interner.lookup_by_file_id(from);
                assert!(from_path.pop());

                // Resolve the given path relative to the importing file's directory.
                from_path.join(path).canonicalize()?
            }
            Dialect::Bazel => {
                // Find the Bazel workspace root.
                let from_path = self.interner.lookup_by_file_id(from);
                let (root, package) = match starpls_bazel::resolve_workspace(&from_path)? {
                    Some(root) => root,
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "not in a Bazel workspace",
                        ))
                    }
                };

                // Parse the load path as a Bazel label.
                let label = match Label::parse(path) {
                    Ok(label) => label,
                    Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.err)),
                };

                // Handle labels with apparent or canonical repos.
                if label.kind() != RepoKind::Current {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "apparent and canonical labels not yet supported",
                    ));
                }

                // Only .bzl files can be loaded.
                if !label.target().ends_with(".bzl") {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "cannot load a non BZL file",
                    ));
                }

                // Loading targets using a relative label causes them to be resolved from the closest package to the importing file.
                if label.is_relative() {
                    package
                } else {
                    root.join(label.package())
                }
                .join(label.target())
            }
        };

        // If we've already interned this file, then simply return the file id.
        if let Some(file_id) = self.interner.lookup_by_path_buf(&path) {
            return Ok((file_id, None));
        }

        let contents = fs::read_to_string(&path)?;
        let file_id = self.interner.intern_path(path);
        Ok((file_id, Some(contents)))
    }

    fn list_load_candidates(
        &self,
        path: &str,
        from: FileId,
    ) -> io::Result<Option<Vec<LoadItemCandidate>>> {
        let from_dir = self.dirname(from);
        let has_trailing_slash = path.ends_with(MAIN_SEPARATOR);
        let mut path = from_dir.join(path);

        if !has_trailing_slash {
            if !path.pop() {
                return Ok(None);
            }
        }

        let path = path.canonicalize()?;
        let mut candidates = vec![];
        let readdir = fs::read_dir(path)?;

        for entry in readdir {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".star") || name.ends_with(".sky") {
                        candidates.push(LoadItemCandidate {
                            kind: LoadItemCandidateKind::File,
                            path: name.to_string(),
                        })
                    }
                }
            }
        }

        Ok(Some(candidates))
    }
}
