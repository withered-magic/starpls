use std::ops::Range;

use anyhow::format_err;
use line_index::LineIndex;
use line_index::WideEncoding;
use line_index::WideLineCol;
use starpls_common::FileId;
use starpls_ide::LocationLink;

use crate::convert;
use crate::server::ServerSnapshot;

pub(crate) fn text_offset(
    line_index: &LineIndex,
    pos: lsp_types::Position,
) -> anyhow::Result<usize> {
    let line_col = line_index
        .to_utf8(
            WideEncoding::Utf16,
            WideLineCol {
                line: pos.line,
                col: pos.character,
            },
        )
        .ok_or_else(|| format_err!("error converting wide line col to utf-8"))?;
    line_index
        .offset(line_col)
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
            line_index = LineIndex::new(&current_document_contents);
        }
    }

    current_document_contents
}

pub(crate) fn response_from_locations<T, U>(
    snapshot: &ServerSnapshot,
    source_file_id: FileId,
    locations: T,
) -> U
where
    T: Iterator<Item = LocationLink>,
    U: From<Vec<lsp_types::Location>> + From<Vec<lsp_types::LocationLink>>,
{
    let source_line_index = match snapshot.analysis_snapshot.line_index(source_file_id) {
        Ok(Some(source_line_index)) => source_line_index,
        _ => return Vec::<lsp_types::Location>::new().into(),
    };

    // let get_line_index = |file_id| snapshot.analysis_snapshot.line_index(file_id);
    let to_lsp_location = |location: LocationLink| -> Option<lsp_types::Location> {
        let location = match location {
            LocationLink::Local {
                target_range,
                target_file_id,
                ..
            } => {
                let target_line_index = snapshot
                    .analysis_snapshot
                    .line_index(target_file_id)
                    .ok()??;
                let range = convert::lsp_range_from_text_range(target_range, target_line_index);
                lsp_types::Location {
                    uri: lsp_types::Url::from_file_path(
                        snapshot
                            .document_manager
                            .read()
                            .lookup_by_file_id(target_file_id),
                    )
                    .ok()?,
                    range: range?,
                }
            }
            LocationLink::External { target_path, .. } => lsp_types::Location {
                uri: lsp_types::Url::from_file_path(target_path).ok()?,
                range: Default::default(),
            },
        };

        Some(location)
    };

    let to_lsp_location_link = |location: LocationLink| -> Option<lsp_types::LocationLink> {
        let location_link = match location {
            LocationLink::Local {
                origin_selection_range,
                target_range,
                target_file_id,
                ..
            } => {
                let target_line_index = snapshot
                    .analysis_snapshot
                    .line_index(target_file_id)
                    .ok()??;
                let range = convert::lsp_range_from_text_range(target_range, target_line_index);
                lsp_types::LocationLink {
                    origin_selection_range: origin_selection_range.and_then(|range| {
                        convert::lsp_range_from_text_range(range, source_line_index)
                    }),
                    target_range: range?,
                    target_selection_range: range?,
                    target_uri: lsp_types::Url::from_file_path(
                        snapshot
                            .document_manager
                            .read()
                            .lookup_by_file_id(target_file_id),
                    )
                    .ok()?,
                }
            }
            LocationLink::External {
                origin_selection_range,
                target_path,
            } => lsp_types::LocationLink {
                origin_selection_range: origin_selection_range
                    .and_then(|range| convert::lsp_range_from_text_range(range, source_line_index)),
                target_range: Default::default(),
                target_selection_range: Default::default(),
                target_uri: lsp_types::Url::from_file_path(target_path).ok()?,
            },
        };

        Some(location_link)
    };

    if snapshot.config.has_text_document_definition_link_support() {
        locations
            .flat_map(to_lsp_location_link)
            .collect::<Vec<_>>()
            .into()
    } else {
        locations
            .flat_map(to_lsp_location)
            .collect::<Vec<_>>()
            .into()
    }
}
