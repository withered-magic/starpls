use anyhow::format_err;
use line_index::{LineCol, LineIndex};
use std::ops::Range;
use tree_sitter::{InputEdit, Point};

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

pub(crate) enum Edit {
    Incremental(InputEdit),
    Full,
}

pub(crate) fn apply_document_content_changes(
    mut current_document_contents: String,
    content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> (String, Vec<Edit>) {
    let mut line_index = LineIndex::new(&current_document_contents);
    let mut edits = vec![];

    for change in content_changes {
        match change.range {
            Some(pos_range) => {
                if let Ok(range) = text_range(&line_index, pos_range) {
                    current_document_contents.replace_range(range.clone(), &change.text);

                    // Update the line index so we can calculate the new end position.
                    line_index = LineIndex::new(&current_document_contents);
                    let new_end_byte = range.start + change.text.len();
                    let new_end_position =
                        line_index.line_col(new_end_byte.try_into().expect("invalid end byte"));

                    // Construct an `InputEdit` for the purpose of updating our tree-sitter parse tree.
                    edits.push(Edit::Incremental(InputEdit {
                        start_byte: range.start,
                        old_end_byte: range.end,
                        new_end_byte,
                        start_position: Point::new(
                            pos_range.start.line as usize,
                            pos_range.start.character as usize,
                        ),
                        old_end_position: Point::new(
                            pos_range.end.line as usize,
                            pos_range.end.character as usize,
                        ),
                        new_end_position: Point::new(
                            new_end_position.line as usize,
                            new_end_position.col as usize,
                        ),
                    }));
                }
            }
            None => {
                current_document_contents = change.text;
                line_index = LineIndex::new(&current_document_contents);
                edits.push(Edit::Full);
            }
        }
    }
    (current_document_contents, edits)
}
