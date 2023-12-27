use anyhow::anyhow;
use line_index::LineIndex;
use starpls_common::{Diagnostic, Severity};
use starpls_syntax::TextRange;
use std::path::PathBuf;

pub(crate) fn path_buf_from_url(url: &lsp_types::Url) -> anyhow::Result<PathBuf> {
    url.to_file_path()
        .map_err(|_| anyhow!("url is not a file: {}", url))
}

pub fn lsp_diagnostic_from_native(
    diagnostic: Diagnostic,
    line_index: &LineIndex,
) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: lsp_range_from_text_range(diagnostic.range.range, &line_index),
        severity: Some(lsp_severity_from_native(diagnostic.severity)),
        code: None,
        code_description: None,
        source: Some("starpls".to_string()),
        message: diagnostic.message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn lsp_range_from_text_range(text_range: TextRange, line_index: &LineIndex) -> lsp_types::Range {
    let start = line_index.line_col(text_range.start());
    let end = line_index.line_col(text_range.end());
    lsp_types::Range {
        start: lsp_types::Position {
            line: start.line,
            character: start.col,
        },
        end: lsp_types::Position {
            line: end.line,
            character: end.col,
        },
    }
}

fn lsp_severity_from_native(severity: Severity) -> lsp_types::DiagnosticSeverity {
    match severity {
        Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
    }
}
