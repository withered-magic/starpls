use starpls_syntax::TextRange;

use crate::FileId;

/// An IDE diagnostic. This is the common data structure used to report errors to the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub message: String,
    pub severity: Severity,
    pub range: FileRange,
    pub tags: Option<Vec<DiagnosticTag>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

/// A severity level for diagnostic messages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticTag {
    Unnecessary,
    Deprecated,
}

#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);
