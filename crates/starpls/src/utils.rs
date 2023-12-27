use anyhow::format_err;
use line_index::{LineCol, LineIndex};
use std::ops::Range;

pub(crate) fn text_offset(
    line_index: &LineIndex,
    pos: lsp_types::Position,
) -> anyhow::Result<usize> {
    line_index
        .offset(LineCol {
            line: pos.line,
            col: pos.character,
        })
        .map(|offset| offset.into())
        .ok_or_else(|| format_err!("invalid offset"))
}

pub(crate) fn text_range(
    line_index: &LineIndex,
    range: lsp_types::Range,
) -> anyhow::Result<Range<usize>> {
    let start = text_offset(line_index, range.start)?;
    let end = text_offset(line_index, range.end)?;
    Ok(start..end)
}

pub(crate) fn apply_document_content_changes(
    mut current_document_contents: String,
    content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> String {
    let mut line_index = LineIndex::new(&current_document_contents);

    for change in content_changes {
        let Some(pos_range) = change.range else {
            continue;
        };
        if let Ok(range) = text_range(&line_index, pos_range) {
            current_document_contents.replace_range(range.clone(), &change.text);

            // Update the line index so we can calculate the new end position.
            line_index = LineIndex::new(&current_document_contents);
            let new_end_byte = range.start + change.text.len();
            let new_end_position =
                line_index.line_col(new_end_byte.try_into().expect("invalid end byte"));
        }
    }

    current_document_contents
}
