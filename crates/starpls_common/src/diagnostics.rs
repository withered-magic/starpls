use starpls_syntax::TextRange;

use crate::FileId;

/// An IDE diagnostic. This is the common data structure used to report errors to the user.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub message: String,
    pub severity: Severity,
    pub range: FileRange,
    pub tags: Option<DiagnosticTag>,
}

#[derive(Clone, Debug)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

/// A severity level for diagnostic messages.
#[derive(Clone, Debug)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub enum DiagnosticTag {
    Unnecessary,
    Deprecated,
}

#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);
