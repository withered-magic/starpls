use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasherDefault;
use std::mem;
use std::path::Path;
use std::path::PathBuf;
use std::path::MAIN_SEPARATOR;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::bail;
use crossbeam_channel::Sender;
use dashmap::DashMap;
use indexmap::IndexSet;
use parking_lot::RwLock;
use rustc_hash::FxHasher;
use starpls_bazel::client::BazelClient;
use starpls_bazel::label::PartialParse;
use starpls_bazel::label::RepoKind;
use starpls_bazel::APIContext;
use starpls_bazel::Label;
use starpls_bazel::ParseError;
use starpls_bazel::{self};
use starpls_common::Dialect;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_common::LoadItemCandidate;
use starpls_common::LoadItemCandidateKind;
use starpls_common::ResolvedPath;
use starpls_ide::FileLoader;
use starpls_ide::LoadFileResult;

use crate::event_loop::FetchExternalRepoRequest;
use crate::event_loop::Task;

macro_rules! try_opt {
    ($expr:expr) => {
        match { $expr } {
            Some(res) => res,
            None => return Ok(None),
        }
    };
}

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
    pub(crate) info: Option<FileInfo>,
    pub(crate) source: DocumentSource,
}

impl Document {
    fn new(
        contents: String,
        dialect: Dialect,
        info: Option<FileInfo>,
        version: Option<i32>,
    ) -> Self {
        Self {
            contents,
            dialect,
            info,
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
    workspace: PathBuf,
}

impl DocumentManager {
    pub(crate) fn new(path_interner: Arc<PathInterner>, workspace: PathBuf) -> Self {
        Self {
            documents: Default::default(),
            has_closed_or_opened_documents: false,
            changed_file_ids: Default::default(),
            path_interner,
            workspace,
        }
    }

    pub(crate) fn open(&mut self, path: PathBuf, version: i32, contents: String) {
        // Create/update the document with the given contents.
        self.has_closed_or_opened_documents = true;
        let (dialect, info) =
            match dialect_and_api_context_for_workspace_path(&self.workspace, &path) {
                Some((dialect, api_context)) => (
                    dialect,
                    api_context.map(|api_context| FileInfo::Bazel {
                        api_context,
                        is_external: !path.starts_with(&self.workspace),
                    }),
                ),
                None => return,
            };
        let file_id = self.path_interner.intern_path(path);
        self.documents.insert(
            file_id,
            Document::new(contents, dialect, info, Some(version)),
        );
        self.changed_file_ids
            .push((file_id, DocumentChangeKind::Create));
    }

    pub(crate) fn close(&mut self, path: &PathBuf) {
        if let Some(file_id) = self.path_interner.lookup_by_path_buf(path) {
            self.has_closed_or_opened_documents = true;
            if let Some(document) = self.documents.get_mut(&file_id) {
                document.source = DocumentSource::Disk;
            };
        }
    }

    pub(crate) fn modify(&mut self, file_id: FileId, contents: String, version: Option<i32>) {
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

pub(crate) struct DefaultFileLoader {
    bazel_client: Arc<dyn BazelClient>,
    interner: Arc<PathInterner>,
    workspace: PathBuf,
    workspace_name: Option<String>,
    external_output_base: PathBuf,
    cached_load_results: DashMap<String, PathBuf>,
    fetch_repo_sender: Sender<Task>,
    bzlmod_enabled: bool,
    load_prefix: Option<String>,
    extensions: Arc<starpls_common::Extensions>,
}

impl DefaultFileLoader {
    pub(crate) fn new(
        bazel_client: Arc<dyn BazelClient>,
        interner: Arc<PathInterner>,
        workspace: PathBuf,
        workspace_name: Option<String>,
        external_output_base: PathBuf,
        fetch_repo_sender: Sender<Task>,
        bzlmod_enabled: bool,
    ) -> Self {
        Self {
            bazel_client,
            interner,
            workspace,
            workspace_name,
            external_output_base,
            cached_load_results: Default::default(),
            fetch_repo_sender,
            bzlmod_enabled,
            load_prefix: None,
            extensions: Arc::new(starpls_common::Extensions::new()),
        }
    }

    pub(crate) fn with_extensions(mut self, extensions: Arc<starpls_common::Extensions>) -> Self {
        self.extensions = extensions;
        self
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

struct ResolvedLabel {
    resolved_path: PathBuf,
    canonical_repo: Option<String>,
}

impl DefaultFileLoader {
    fn resolve_label(&self, label: &Label, from: FileId) -> anyhow::Result<Option<ResolvedLabel>> {
        let repo_kind = label.kind();
        let mut canonical_repo_res = None;
        let (root, package) = match &repo_kind {
            RepoKind::Apparent if self.bzlmod_enabled => {
                let from_path = self.interner.lookup_by_file_id(from);
                let from_repo = try_opt!(self.repo_for_path(&from_path));
                let canonical_repo = self
                    .bazel_client
                    .resolve_repo_from_mapping(label.repo(), from_repo)?;
                match canonical_repo {
                    Some(canonical_repo) => (
                        if canonical_repo.is_empty() {
                            self.workspace.clone()
                        } else {
                            canonical_repo_res = Some(canonical_repo.clone());
                            self.external_output_base.join(canonical_repo)
                        },
                        PathBuf::new(),
                    ),
                    None => {
                        bail!(
                            "Could not resolve repository \"{}{}\" from current repository mapping",
                            match label.kind() {
                                RepoKind::Canonical => "@@",
                                _ => "@",
                            },
                            label.repo()
                        )
                    }
                }
            }
            RepoKind::Canonical | RepoKind::Apparent => {
                if !label.repo().is_empty() {
                    canonical_repo_res = Some(label.repo().to_string());
                }

                if self.workspace_name.as_deref() == Some(label.repo()) || label.repo().is_empty() {
                    (self.workspace.clone(), PathBuf::new())
                } else {
                    (self.external_output_base.join(label.repo()), PathBuf::new())
                }
            }
            RepoKind::Current => {
                // Find the Bazel workspace root.
                let from_path = self.interner.lookup_by_file_id(from);
                match starpls_bazel::resolve_workspace(from_path)? {
                    Some(root) => root,
                    None => {
                        bail!("not in a Bazel workspace")
                    }
                }
            }
        };

        // Loading targets using a relative label causes them to be resolved from the closest package to the importing file.
        let resolved_path = if label.is_relative() {
            package
        } else {
            root.join(label.package())
        }
        .join(label.target());

        Ok(Some(ResolvedLabel {
            resolved_path,
            canonical_repo: canonical_repo_res,
        }))
    }

    fn maybe_intern_file(
        &self,
        path: PathBuf,
        from: FileId,
        fetch_repo_on_err: Option<String>,
    ) -> anyhow::Result<(FileId, Option<String>)> {
        // If we've already interned this file, then simply return the file id.
        let (file_id, contents) = match self.interner.lookup_by_path_buf(&path) {
            Some(file_id) => (file_id, None),
            None => {
                let contents = match fs::read_to_string(&path) {
                    Ok(contents) => contents,
                    Err(err) => {
                        if let Some(canonical_repo) = fetch_repo_on_err {
                            if !self
                                .external_output_base
                                .join(&canonical_repo)
                                .try_exists()
                                .ok()
                                .unwrap_or_default()
                            {
                                let _ = self.fetch_repo_sender.send(
                                    Task::FetchExternalRepoRequest(FetchExternalRepoRequest {
                                        file_id: from,
                                        repo: canonical_repo,
                                    }),
                                );
                            }
                        }
                        return Err(err.into());
                    }
                };

                (self.interner.intern_path(path), Some(contents))
            }
        };

        Ok((file_id, contents))
    }

    fn repo_for_path<'a>(&'a self, path: &'a Path) -> Option<&str> {
        match path.strip_prefix(&self.external_output_base) {
            Ok(stripped) => stripped
                .components()
                .next()
                .as_ref()
                .and_then(|component| component.as_os_str().to_str()),
            Err(_) => {
                if path.starts_with(&self.workspace) {
                    Some("")
                } else {
                    None
                }
            }
        }
    }
}

/// Create virtual file content from a list of symbols.
/// This generates Starlark code that defines the functions/variables.
fn create_virtual_file_content(symbols: &[starpls_common::Symbol]) -> String {
    let mut content = String::new();
    content.push_str("# Virtual module - generated from extension\n\n");

    for symbol in symbols {
        if !symbol.properties.is_empty() {
            // Object with properties: generate struct
            if !symbol.doc.is_empty() {
                content.push_str(&format!("# {}\n", symbol.doc));
            }
            content.push_str(&format!("{} = struct(\n", symbol.name));
            for (prop_name, prop_symbol) in &symbol.properties {
                if prop_symbol.r#type == "function" && prop_symbol.callable.is_some() {
                    // Generate proper function with signature
                    let callable = prop_symbol.callable.as_ref().unwrap();
                    let params: Vec<String> = callable
                        .params
                        .iter()
                        .map(|p| {
                            if p.default_value.is_empty() {
                                p.name.clone()
                            } else {
                                format!("{} = {}", p.name, p.default_value)
                            }
                        })
                        .collect();
                    content.push_str(&format!(
                        "    {} = lambda {}: None,\n",
                        prop_name,
                        params.join(", ")
                    ));
                } else {
                    content.push_str(&format!("    {} = lambda *a, **k: None,\n", prop_name));
                }
            }
            content.push_str(")\n\n");
        } else {
            match symbol.r#type.as_str() {
                "function" => {
                    if let Some(callable) = &symbol.callable {
                        content.push_str(&format!("def {}(", symbol.name));

                        let params: Vec<String> = callable
                            .params
                            .iter()
                            .map(|p| {
                                if p.default_value.is_empty() {
                                    p.name.clone()
                                } else {
                                    format!("{} = {}", p.name, p.default_value)
                                }
                            })
                            .collect();

                        content.push_str(&params.join(", "));
                        content.push_str("):\n");

                        if !symbol.doc.is_empty() {
                            content.push_str(&format!("    \"\"\"{}\"\"\"\n", symbol.doc));
                        }

                        content.push_str("    pass  # Virtual function\n\n");
                    } else {
                        // Function without signature info
                        content.push_str(&format!("def {}():\n", symbol.name));
                        if !symbol.doc.is_empty() {
                            content.push_str(&format!("    \"\"\"{}\"\"\"\n", symbol.doc));
                        }
                        content.push_str("    pass  # Virtual function\n\n");
                    }
                }
                _ => {
                    // Variables, constants, etc.
                    content.push_str(&format!(
                        "{} = None  # Virtual {}\n",
                        symbol.name, symbol.r#type
                    ));
                    if !symbol.doc.is_empty() {
                        content.push_str(&format!("# {}\n", symbol.doc));
                    }
                    content.push('\n');
                }
            }
        }
    }

    content
}

impl FileLoader for DefaultFileLoader {
    fn resolve_path(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>> {
        if dialect != Dialect::Bazel {
            return Ok(None);
        }

        // Parse the load path as a Bazel label.
        let label = match Label::parse(path) {
            Ok(label) => label,
            Err(err) => return Err(anyhow!("error parsing label: {}", err.err)),
        };

        let resolved_label = try_opt!(self.resolve_label(&label, from)?);
        let res = if fs::metadata(&resolved_label.resolved_path)
            .ok()
            .map(|metadata| metadata.is_file())
            .unwrap_or_default()
        {
            ResolvedPath::Source {
                path: resolved_label.resolved_path,
            }
        } else {
            if label.target().is_empty() {
                return Ok(None);
            }

            let parent = try_opt!(resolved_label.resolved_path.parent());
            let build_file = try_opt!(fs::read_dir(parent)
                .into_iter()
                .flat_map(|entries| entries.into_iter())
                .find_map(|entry| match entry.ok()?.file_name().to_str()? {
                    file_name @ ("BUILD" | "BUILD.bazel") => Some(file_name.to_string()),
                    _ => None,
                }));
            let path = parent.join(build_file);

            // If we've already interned this file, then simply return the file id.
            let (build_file, contents) =
                self.maybe_intern_file(path, from, resolved_label.canonical_repo)?;

            ResolvedPath::BuildTarget {
                build_file,
                target: label.target().to_string(),
                contents,
            }
        };

        Ok(Some(res))
    }

    fn load_file(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<LoadFileResult>> {
        // Check for virtual modules first (only for Standard dialect)
        if dialect == Dialect::Standard {
            let from_path = self.interner.lookup_by_file_id(from);
            if let Some(symbols) = self.extensions.virtual_module(path, &from_path) {
                // Create a virtual file with the module's symbols
                let virtual_content = create_virtual_file_content(symbols);

                // Create a virtual file ID based on the path
                let virtual_path = PathBuf::from(format!("virtual://{}", path));
                let file_id = self.interner.intern_path(virtual_path);

                return Ok(Some(LoadFileResult {
                    file_id,
                    dialect,
                    info: None,
                    contents: Some(virtual_content),
                }));
            }
        }

        let (path, info, canonical_repo) = match dialect {
            Dialect::Standard => {
                // Find the importing file's directory.
                let mut from_path = self.interner.lookup_by_file_id(from);
                assert!(from_path.pop());

                // Apply load_prefix from extensions if configured for this file
                let full_from_path = self.interner.lookup_by_file_id(from);
                let resolved_path =
                    if let Some(prefix) = self.extensions.load_prefix_for_file(&full_from_path) {
                        // Use extension-specific prefix
                        let prefixed_path = if path.is_empty() {
                            format!("{}/", prefix.trim_end_matches('/'))
                        } else {
                            format!("{}/{}", prefix.trim_end_matches('/'), path)
                        };
                        from_path.join(prefixed_path)
                    } else if let Some(ref prefix) = self.load_prefix {
                        // Fall back to global prefix (for backward compatibility)
                        let prefixed_path = if path.is_empty() {
                            format!("{}/", prefix.trim_end_matches('/'))
                        } else {
                            format!("{}/{}", prefix.trim_end_matches('/'), path)
                        };
                        from_path.join(prefixed_path)
                    } else {
                        from_path.join(path)
                    };

                // Resolve the given path relative to the importing file's directory.
                (resolved_path.canonicalize()?, None, None)
            }
            Dialect::Bazel => {
                // Parse the load path as a Bazel label.
                let label = match Label::parse(path) {
                    Ok(label) => label,
                    Err(err) => return Err(anyhow!("error parsing label: {}", err.err)),
                };

                // Only .bzl files can be loaded.
                if !label.target().ends_with(".bzl") {
                    bail!("cannot load a non-bzl file");
                }

                let repo_kind = label.kind();
                let (resolved_path, canonical_repo) = match self
                    .read_cache_result(&repo_kind, path, from)
                {
                    Some(path) => (path, None),
                    None => {
                        let res = try_opt!(self.resolve_label(&label, from)?);
                        self.record_cache_result(&repo_kind, path, from, res.resolved_path.clone());
                        (res.resolved_path, res.canonical_repo)
                    }
                };

                let is_external = !resolved_path.starts_with(&self.workspace);
                (
                    resolved_path,
                    Some(FileInfo::Bazel {
                        api_context: APIContext::Bzl,
                        is_external,
                    }),
                    canonical_repo,
                )
            }
        };

        let (file_id, contents) = self.maybe_intern_file(path, from, canonical_repo)?;
        Ok(Some(LoadFileResult {
            file_id,
            dialect,
            info,
            contents,
        }))
    }

    fn list_load_candidates(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>> {
        let from_path = self.interner.lookup_by_file_id(from);
        match dialect {
            Dialect::Standard => {
                let from_dir = from_path.parent().unwrap();
                let has_trailing_slash = path.ends_with(MAIN_SEPARATOR);
                let mut path = from_dir.join(path);
                if !has_trailing_slash && !path.pop() {
                    return Ok(None);
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
                                    replace_trailing_slash: false,
                                })
                            }
                        }
                    }
                }

                Ok(Some(candidates))
            }
            Dialect::Bazel => {
                // Determine the loading file's workspace root and package.
                let (mut root, package) = try_opt!(starpls_bazel::resolve_workspace(
                    self.interner.lookup_by_file_id(from),
                )?);
                let (label, err) = match Label::parse(path) {
                    Ok(label) => (label, None),
                    Err(PartialParse { partial, err }) => (partial, Some(err)),
                };

                if !label.has_leading_slashes()
                    && !label.is_relative()
                    && err != Some(ParseError::InvalidRepo)
                {
                    return Ok(match label.kind() {
                        RepoKind::Apparent if self.bzlmod_enabled => Some(
                            self.bazel_client
                                .repo_mapping_keys("")?
                                .into_iter()
                                .map(|repo| LoadItemCandidate {
                                    kind: LoadItemCandidateKind::Directory,
                                    path: repo.to_string(),
                                    replace_trailing_slash: false,
                                })
                                .collect(),
                        ),
                        RepoKind::Canonical | RepoKind::Apparent => Some(
                            fs::read_dir(&self.external_output_base)?
                                .filter_map(|entry| {
                                    let entry = entry.ok()?;
                                    entry.file_type().ok()?.is_dir().then(|| LoadItemCandidate {
                                        kind: LoadItemCandidateKind::Directory,
                                        path: entry.file_name().to_string_lossy().to_string(),
                                        replace_trailing_slash: false,
                                    })
                                })
                                .chain(self.workspace_name.as_ref().map(|name| LoadItemCandidate {
                                    kind: LoadItemCandidateKind::Directory,
                                    path: name.clone(),
                                    replace_trailing_slash: false,
                                }))
                                .collect(),
                        ),
                        _ => None,
                    });
                }

                match label.kind() {
                    RepoKind::Apparent | RepoKind::Canonical => {
                        root = if self.bzlmod_enabled {
                            let from_repo = try_opt!(self.repo_for_path(&from_path));
                            let canonical_repo = try_opt!(self
                                .bazel_client
                                .resolve_repo_from_mapping(label.repo(), from_repo)?);
                            if canonical_repo.is_empty() {
                                self.workspace.clone()
                            } else {
                                self.external_output_base.join(canonical_repo)
                            }
                        } else if self.workspace_name.as_deref() == Some(label.repo())
                            || label.repo().is_empty()
                        {
                            self.workspace.clone()
                        } else {
                            self.external_output_base.join(label.repo())
                        };
                    }
                    RepoKind::Current => {}
                }

                match err {
                    Some(ParseError::EmptyPackage) => {
                        // An empty package usually indicates that the user is about to
                        // starting typing the package name.
                        read_dir_packages_and_targets(root, false).map(Some)
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

                    Some(ParseError::InvalidPackageEndingSlash) | None => {
                        // TODO(withered-magic): Handle targets like in `//foo:bar/baz.bzl`.
                        if label.is_relative() {
                            // If the label is relative, check for target candidates in the current package.
                            read_dir_targets(package).map(Some)
                        } else if !label.target().is_empty() && !label.has_target_shorthand() {
                            // Check for target candidates in the label's package.
                            let package_dir = root.join(label.package());
                            let (target_dir, _) =
                                try_opt!(strip_slashes_or_pop_dir(label.target()));
                            read_dir_targets(package_dir.join(target_dir)).map(Some)
                        } else {
                            // Otherwise, find package candidates.
                            let (package_dir, has_trailing_slash) =
                                try_opt!(strip_slashes_or_pop_dir(label.package()));
                            read_dir_packages_and_targets(
                                root.join(package_dir),
                                has_trailing_slash,
                            )
                            .map(Some)
                        }
                    }

                    _ => {
                        // Don't offer completions for any other parsing errors.
                        Ok(None)
                    }
                }
            }
        }
    }

    fn resolve_build_file(&self, file_id: FileId) -> Option<String> {
        let path = self.interner.lookup_by_file_id(file_id);
        let path = path.strip_prefix(&self.workspace).ok()?;
        if matches!(
            &*path.file_name()?.to_string_lossy(),
            "BUILD" | "BUILD.bazel"
        ) {
            Some(path.parent()?.to_string_lossy().to_string())
        } else {
            None
        }
    }
}

fn read_dir_packages_and_targets(
    path: impl AsRef<Path>,
    has_trailing_slash: bool,
) -> anyhow::Result<Vec<LoadItemCandidate>> {
    Ok(fs::read_dir(path)?
        .flatten()
        .filter_map(|entry| {
            entry
                .file_type()
                .map(|file_type| (file_type, entry.file_name()))
                .ok()
        })
        .filter_map(|(file_type, file_name)| {
            file_name.to_str().and_then(|file_name| {
                let (kind, path, replace_trailing_slash) = if file_type.is_dir() {
                    (
                        LoadItemCandidateKind::Directory,
                        file_name.to_string(),
                        false,
                    )
                } else if file_name.ends_with(".bzl") {
                    (
                        LoadItemCandidateKind::File,
                        format!(":{}", file_name),
                        has_trailing_slash,
                    )
                } else {
                    return None;
                };
                Some(LoadItemCandidate {
                    kind,
                    path,
                    replace_trailing_slash,
                })
            })
        })
        .collect())
}

fn read_dir_targets(path: impl AsRef<Path>) -> anyhow::Result<Vec<LoadItemCandidate>> {
    Ok(fs::read_dir(path)?
        .flatten()
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
                    replace_trailing_slash: false,
                })
            })
        })
        .collect())
}

fn strip_slashes_or_pop_dir(input: &str) -> Option<(PathBuf, bool)> {
    Some(if input.ends_with('/') {
        (PathBuf::from(input.trim_end_matches('/')), true)
    } else {
        let mut target_dir = PathBuf::from(input);
        if !target_dir.pop() {
            return None;
        }
        (target_dir, false)
    })
}

pub(crate) fn dialect_and_api_context_for_workspace_path(
    workspace: impl AsRef<Path>,
    path: impl AsRef<Path>,
) -> Option<(Dialect, Option<APIContext>)> {
    let path = path.as_ref();
    let basename = path.file_name().and_then(|name| name.to_str())?;
    Some(match basename {
        "BUILD" | "BUILD.bazel" => (Dialect::Bazel, Some(APIContext::Build)),
        "REPO.bazel" => (Dialect::Bazel, Some(APIContext::Repo)),
        "VENDOR.bazel" => (Dialect::Bazel, Some(APIContext::Vendor)),
        "MODULE.bazel" => (Dialect::Bazel, Some(APIContext::Module)),
        path if path.ends_with(".MODULE.bazel") => (Dialect::Bazel, Some(APIContext::Module)),
        "WORKSPACE" | "WORKSPACE.bazel" | "WORKSPACE.bzlmod" => {
            (Dialect::Bazel, Some(APIContext::Workspace))
        }
        path if path.ends_with(".BUILD.bazel") || path.ends_with(".BUILD") => {
            (Dialect::Bazel, Some(APIContext::Build))
        }
        path if path.ends_with(".cquery") || path.ends_with(".query.bzl") => {
            (Dialect::Bazel, Some(APIContext::Cquery))
        }
        _ => match path.extension().and_then(|ext| ext.to_str()) {
            Some("bzl") => (Dialect::Bazel, Some(APIContext::Bzl)),
            _ => {
                if path == workspace.as_ref().join("tools/build_rules/prelude_bazel") {
                    (Dialect::Bazel, Some(APIContext::Prelude))
                } else {
                    (Dialect::Standard, None)
                }
            }
        },
    })
}
