use std::path::PathBuf;

use anyhow::anyhow;
use line_index::{LineIndex, WideEncoding, WideLineCol};
use starpls_common::{Diagnostic, FileId, Severity};
use starpls_ide::{DocumentSymbol, SymbolKind, SymbolTag};
use starpls_syntax::{TextRange, TextSize};

use crate::server::ServerSnapshot;

pub(crate) fn path_buf_from_url(url: &lsp_types::Url) -> anyhow::Result<PathBuf> {
    url.to_file_path()
        .map_err(|_| anyhow!("url is not a file: {}", url))
}

pub(crate) fn lsp_diagnostic_from_native(
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

#[allow(deprecated)]
pub(crate) fn lsp_document_symbol_from_native(
    DocumentSymbol {
        name,
        detail,
        kind,
        tags,
        range,
        selection_range,
        children,
    }: DocumentSymbol,
    line_index: &LineIndex,
) -> Option<lsp_types::DocumentSymbol> {
    Some(lsp_types::DocumentSymbol {
        name,
        detail,
        kind: match kind {
            SymbolKind::File => lsp_types::SymbolKind::FILE,
            SymbolKind::Module => lsp_types::SymbolKind::MODULE,
            SymbolKind::Namespace => lsp_types::SymbolKind::NAMESPACE,
            SymbolKind::Package => lsp_types::SymbolKind::PACKAGE,
            SymbolKind::Class => lsp_types::SymbolKind::CLASS,
            SymbolKind::Method => lsp_types::SymbolKind::METHOD,
            SymbolKind::Property => lsp_types::SymbolKind::PROPERTY,
            SymbolKind::Field => lsp_types::SymbolKind::FIELD,
            SymbolKind::Constructor => lsp_types::SymbolKind::CONSTRUCTOR,
            SymbolKind::Enum => lsp_types::SymbolKind::ENUM,
            SymbolKind::Interface => lsp_types::SymbolKind::INTERFACE,
            SymbolKind::Function => lsp_types::SymbolKind::FUNCTION,
            SymbolKind::Variable => lsp_types::SymbolKind::VARIABLE,
            SymbolKind::Constant => lsp_types::SymbolKind::CONSTANT,
            SymbolKind::String => lsp_types::SymbolKind::STRING,
            SymbolKind::Number => lsp_types::SymbolKind::NUMBER,
            SymbolKind::Boolean => lsp_types::SymbolKind::BOOLEAN,
            SymbolKind::Array => lsp_types::SymbolKind::ARRAY,
            SymbolKind::Object => lsp_types::SymbolKind::OBJECT,
            SymbolKind::Key => lsp_types::SymbolKind::KEY,
            SymbolKind::Null => lsp_types::SymbolKind::NULL,
            SymbolKind::EnumMember => lsp_types::SymbolKind::ENUM_MEMBER,
            SymbolKind::Struct => lsp_types::SymbolKind::STRUCT,
            SymbolKind::Event => lsp_types::SymbolKind::EVENT,
            SymbolKind::Operator => lsp_types::SymbolKind::OPERATOR,
            SymbolKind::TypeParameter => lsp_types::SymbolKind::TYPE_PARAMETER,
        },
        tags: tags.map(|tags| {
            tags.into_iter()
                .map(|tag| match tag {
                    SymbolTag::Deprecated => lsp_types::SymbolTag::DEPRECATED,
                })
                .collect()
        }),
        range: lsp_range_from_text_range(range, line_index)?,
        selection_range: lsp_range_from_text_range(selection_range, line_index)?,
        children: children.map(|children| {
            children
                .into_iter()
                .filter_map(|child| lsp_document_symbol_from_native(child, line_index))
                .collect()
        }),
        deprecated: None,
    })
}
