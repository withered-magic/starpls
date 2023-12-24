use starpls_syntax::TextRange;

use crate::FileId;

/// An IDE diagnostic. This is the common data structure used to report errors to the user.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub message: String,
    pub range: FileRange,
}

#[derive(Clone, Debug)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

#[salsa::accumulator]
pub struct Diagnostics(Diagnostic);
