use std::{fmt::Debug, path::PathBuf};

use starpls_bazel::APIContext;
use starpls_syntax::{
    line_index as syntax_line_index, parse_module, LineIndex, Module, ParseTree, SyntaxNode,
};

pub use crate::diagnostics::{Diagnostic, Diagnostics, FileRange, Severity};

mod diagnostics;
mod util;

#[salsa::jar(db = Db)]
pub struct Jar(
    Diagnostics,
    File,
    LineIndexResult,
    Parse,
    parse,
    line_index_query,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dialect {
    Standard,
    Bazel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadItemCandidateKind {
    Directory,
    File,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadItemCandidate {
    pub kind: LoadItemCandidateKind,
    pub path: String,
}

/// A Key corresponding to an interned file path. Use these instead of `Path`s to refer to files.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u32);

pub enum ResolvedPath {
    Source {
        path: PathBuf,
    },
    BuildTarget {
        build_file: FileId,
        target: String,
        contents: Option<String>,
    },
}

/// The base Salsa database. Supports file-related operations, like getting/setting file contents.
pub trait Db: salsa::DbWithJar<Jar> {
    /// Creates a `File` in the database. This will overwrite the currently active
    /// `File` for the given `FileId`, if it exists.
    fn create_file(
        &mut self,
        file_id: FileId,
        dialect: Dialect,
        api_context: Option<APIContext>,
        contents: String,
    ) -> File;

    /// Sets the contents the `File` identified by the given `FileId`. Has no affect
    /// if the file doesn't exist.
    fn update_file(&mut self, file_id: FileId, contents: String);

    /// Loads a file from the filesystem.
    fn load_file(&self, path: &str, dialect: Dialect, from: FileId)
        -> anyhow::Result<Option<File>>;

    /// Returns the `File` identified by the given `FileId`.
    fn get_file(&self, file_id: FileId) -> Option<File>;

    fn list_load_candidates(
        &self,
        path: &str,
        from: FileId,
    ) -> anyhow::Result<Option<Vec<LoadItemCandidate>>>;

    fn resolve_path(
        &self,
        path: &str,
        dialect: Dialect,
        from: FileId,
    ) -> anyhow::Result<Option<ResolvedPath>>;
}

#[salsa::input]
pub struct File {
    pub id: FileId,
    pub dialect: Dialect,
    pub api_context: Option<APIContext>,
    #[return_ref]
    pub contents: String,
}

#[salsa::tracked]
pub struct Parse {
    pub file: File,
    module: ParseTree<Module>,
}

impl Parse {
    pub fn tree(&self, db: &dyn Db) -> Module {
        self.module(db).tree()
    }

    pub fn syntax(&self, db: &dyn Db) -> SyntaxNode {
        self.module(db).syntax()
    }
}

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: File) -> Parse {
    let parse = parse_module(&file.contents(db), &mut |err| {
        Diagnostics::push(
            db,
            Diagnostic {
                message: err.message,
                range: FileRange {
                    file_id: file.id(db),
                    range: err.range,
                },
                severity: Severity::Error,
            },
        )
    });
    Parse::new(db, file, parse)
}

#[salsa::tracked]
struct LineIndexResult {
    #[return_ref]
    pub inner: LineIndex,
}

#[salsa::tracked]
fn line_index_query(db: &dyn Db, file: File) -> LineIndexResult {
    let line_index = syntax_line_index(&file.contents(db));
    LineIndexResult::new(db, line_index)
}

pub fn line_index(db: &dyn Db, file: File) -> &LineIndex {
    line_index_query(db, file).inner(db)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InFile<T: Clone + Debug> {
    pub file: File,
    pub value: T,
}
