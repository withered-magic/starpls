use dashmap::DashMap;
use indexmap::IndexSet;
use parking_lot::RwLock;
use rustc_hash::FxHasher;
use starpls_bazel::{
    self,
    label::{PartialParse, RepoKind},
    Label, ParseError,
};
use starpls_common::{Dialect, FileId, LoadItemCandidate, LoadItemCandidateKind};
use starpls_ide::FileLoader;
use std::{
    collections::HashMap,
    fs,
    hash::BuildHasherDefault,
    io, mem,
    path::{Path, PathBuf, MAIN_SEPARATOR},
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
    output_base: PathBuf,
    cached_load_results: DashMap<String, PathBuf>,
}

impl DefaultFileLoader {
    pub(crate) fn new(interner: Arc<PathInterner>, output_base: PathBuf) -> Self {
        Self {
            interner,
            output_base,
            cached_load_results: Default::default(),
        }
    }

    fn make_cache_key(&self, repo_kind: &RepoKind, path: &str, from: FileId) -> String {
        format!("{:?}-{:?}-{:?}", repo_kind, path, from.0)
    }

    fn read_cache_result(&self, repo_kind: &RepoKind, path: &str, from: FileId) -> Option<PathBuf> {
        let key = self.make_cache_key(repo_kind, path, from);
        self.cached_load_results
            .get(&key)
            .map(|path_buf| path_buf.clone())
    }

    fn record_cache_result(
        &self,
        repo_kind: &RepoKind,
        path: &str,
        from: FileId,
        resolved_path: PathBuf,
    ) {
        let key = self.make_cache_key(repo_kind, path, from);
        self.cached_load_results.insert(key, resolved_path);
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
    ) -> io::Result<Option<(FileId, Option<String>)>> {
        let path = match dialect {
            Dialect::Standard => {
                // Find the importing file's directory.
                let mut from_path = self.interner.lookup_by_file_id(from);
                assert!(from_path.pop());

                // Resolve the given path relative to the importing file's directory.
                from_path.join(path).canonicalize()?
            }
            Dialect::Bazel => {
                // Parse the load path as a Bazel label.
                let label = match Label::parse(path) {
                    Ok(label) => label,
                    Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.err)),
                };

                // Only .bzl files can be loaded.
                if !label.target().ends_with(".bzl") {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "cannot load a non-bzl file",
                    ));
                }

                let repo_kind = label.kind();

                match self.read_cache_result(&repo_kind, path, from) {
                    Some(path) => path,
                    None => {
                        let (root, package) = match &repo_kind {
                            RepoKind::Apparent => (
                                { self.output_base.join("external").join(label.repo()) },
                                PathBuf::new(),
                            ),
                            RepoKind::Current => {
                                // Find the Bazel workspace root.
                                let from_path = self.interner.lookup_by_file_id(from);
                                match starpls_bazel::resolve_workspace(&from_path)? {
                                    Some(root) => root,
                                    None => {
                                        return Err(io::Error::new(
                                            io::ErrorKind::Other,
                                            "not in a Bazel workspace",
                                        ))
                                    }
                                }
                            }
                            _ => return Ok(None),
                        };

                        // Loading targets using a relative label causes them to be resolved from the closest package to the importing file.
                        let resolved_path = if label.is_relative() {
                            package
                        } else {
                            root.join(label.package())
                        }
                        .join(label.target());

                        self.record_cache_result(&repo_kind, path, from, resolved_path.clone());
                        resolved_path
                    }
                }
            }
        };

        Ok(Some({
            // If we've already interned this file, then simply return the file id.
            if let Some(file_id) = self.interner.lookup_by_path_buf(&path) {
                (file_id, None)
            } else {
                let contents = fs::read_to_string(&path)?;
                let file_id = self.interner.intern_path(path);
                (file_id, Some(contents))
            }
        }))
    }

    fn list_load_candidates(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> io::Result<Option<Vec<LoadItemCandidate>>> {
        match dialect {
            Dialect::Standard => {
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
            Dialect::Bazel => {
                // Determine the loading file's workspace root and package.
                let (root, package) = match starpls_bazel::resolve_workspace(
                    self.interner.lookup_by_file_id(from),
                )? {
                    Some(res) => res,
                    None => return Ok(None),
                };

                let (label, err) = match Label::parse(path) {
                    Ok(label) => (label, None),
                    Err(PartialParse { partial, err }) => (partial, Some(err)),
                };

                if label.kind() != RepoKind::Current
                    || (!label.has_leading_slashes() && !label.is_relative())
                {
                    return Ok(None);
                }

                match err {
                    Some(ParseError::EmptyPackage) => {
                        // An empty package usually indicates that the user is about to
                        // starting typing the package name.
                        read_dir_packages(root).map(Some)
                    }
                    Some(ParseError::EmptyTarget) => {
                        // Same logic as above, but for the target.
                        read_dir_targets(if label.is_relative() {
                            package
                        } else {
                            root.join(label.package())
                        })
                        .map(Some)
                    }
                    Some(_) => {
                        // Don't offer completions for any other parsing errors.
                        Ok(None)
                    }
                    None => {
                        // TODO(withered-magic): Handle targets like in `//foo:bar/baz.bzl`.
                        if label.is_relative() {
                            // If the label is relative, check for target candidates in the current package.
                            read_dir_targets(package).map(Some)
                        } else if !label.target().is_empty() && !label.has_target_shorthand() {
                            // Check for target candidates in the label's package.
                            read_dir_targets(root.join(label.package())).map(Some)
                        } else {
                            // Otherwise, find package candidates.
                            let mut dir = root.join(label.package());
                            if !label.package().ends_with('/') && !dir.pop() {
                                Ok(None)
                            } else {
                                read_dir_packages(dir).map(Some)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn read_dir_packages(path: impl AsRef<Path>) -> io::Result<Vec<LoadItemCandidate>> {
    Ok(fs::read_dir(path)?
        .flat_map(|entry| entry)
        .filter_map(|entry| {
            entry
                .file_type()
                .map(|file_type| (file_type, entry.file_name()))
                .ok()
        })
        .filter_map(|(file_type, file_name)| {
            file_name.to_str().and_then(|file_name| {
                Some(LoadItemCandidate {
                    kind: if file_type.is_dir() {
                        LoadItemCandidateKind::Directory
                    } else {
                        return None;
                    },
                    path: file_name.to_string(),
                })
            })
        })
        .collect())
}

fn read_dir_targets(path: impl AsRef<Path>) -> io::Result<Vec<LoadItemCandidate>> {
    Ok(fs::read_dir(path)?
        .flat_map(|entry| entry)
        .filter_map(|entry| {
            entry
                .file_type()
                .map(|file_type| (file_type, entry.file_name()))
                .ok()
        })
        .filter_map(|(file_type, file_name)| {
            file_name.to_str().and_then(|file_name| {
                Some(LoadItemCandidate {
                    kind: if file_type.is_dir() {
                        LoadItemCandidateKind::Directory
                    } else if file_name.ends_with(".bzl") {
                        LoadItemCandidateKind::File
                    } else {
                        return None;
                    },
                    path: file_name.to_string(),
                })
            })
        })
        .collect())
}
