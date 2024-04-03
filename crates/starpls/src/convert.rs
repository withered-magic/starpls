use anyhow::anyhow;
use line_index::{LineIndex, WideEncoding, WideLineCol};
use starpls_common::{Diagnostic, FileId, Severity};
use starpls_syntax::{TextRange, TextSize};
use std::path::PathBuf;

use crate::server::ServerSnapshot;

pub(crate) fn path_buf_from_url(url: &lsp_types::Url) -> anyhow::Result<PathBuf> {
    url.to_file_path()
        .map_err(|_| anyhow!("url is not a file: {}", url))
}

pub fn lsp_diagnostic_from_native(
    diagnostic: Diagnostic,
    line_index: &LineIndex,
) -> Option<lsp_types::Diagnostic> {
    Some(lsp_types::Diagnostic {
        range: lsp_range_from_text_range(diagnostic.range.range, &line_index)?,
        severity: Some(lsp_severity_from_native(diagnostic.severity)),
        code: None,
        code_description: None,
        source: Some("starpls".to_string()),
        message: diagnostic.message,
        related_information: None,
        tags: None,
        data: None,
    })
}

pub(crate) fn lsp_range_from_text_range(
    text_range: TextRange,
    line_index: &LineIndex,
) -> Option<lsp_types::Range> {
    let start = line_index.to_wide(WideEncoding::Utf16, line_index.line_col(text_range.start()))?;
    let end = line_index.to_wide(WideEncoding::Utf16, line_index.line_col(text_range.end()))?;
    Some(lsp_types::Range {
        start: lsp_types::Position {
            line: start.line,
            character: start.col,
        },
        end: lsp_types::Position {
            line: end.line,
            character: end.col,
        },
    })
}

fn wide_line_col_from_lsp_position(pos: lsp_types::Position) -> WideLineCol {
    WideLineCol {
        line: pos.line,
        col: pos.character,
    }
}

pub(crate) fn text_size_from_lsp_position(
    snapshot: &ServerSnapshot,
    file_id: FileId,
    pos: lsp_types::Position,
) -> anyhow::Result<Option<TextSize>> {
    let line_index = match snapshot.analysis_snapshot.line_index(file_id)? {
        Some(line_index) => line_index,
        None => return Ok(None),
    };
    let line_col =
        match line_index.to_utf8(WideEncoding::Utf16, wide_line_col_from_lsp_position(pos)) {
            Some(line_col) => line_col,
            None => return Ok(None),
        };
    Ok(line_index.offset(line_col))
}

fn lsp_severity_from_native(severity: Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
        Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
    }
}
