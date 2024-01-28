use anyhow::Ok;
use starpls_ide::{completions::CompletionItemKind, FilePosition, Location};

use crate::{
    convert::{self, path_buf_from_url},
    extensions::{ShowHirParams, ShowSyntaxTreeParams},
    server::ServerSnapshot,
};

pub(crate) fn show_hir(snapshot: &ServerSnapshot, params: ShowHirParams) -> anyhow::Result<String> {
    let document_manager = snapshot.document_manager.read();
    let path = path_buf_from_url(&params.text_document.uri)?;
    let file_id = match document_manager.lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok("".to_string()),
    };
    let rendered_hir = snapshot.analysis_snapshot.show_hir(file_id)?;
    Ok(rendered_hir.unwrap_or_else(|| "".to_string()))
}

pub(crate) fn show_syntax_tree(
    snapshot: &ServerSnapshot,
    params: ShowSyntaxTreeParams,
) -> anyhow::Result<String> {
    let path = path_buf_from_url(&params.text_document.uri)?;
    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok("".to_string()),
    };
    let rendered_syntax_tree = snapshot.analysis_snapshot.show_syntax_tree(file_id)?;
    Ok(rendered_syntax_tree.unwrap_or_else(|| "".to_string()))
}

pub(crate) fn goto_definition(
    snapshot: &ServerSnapshot,
    params: lsp_types::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp_types::GotoDefinitionResponse>> {
    let path = path_buf_from_url(&params.text_document_position_params.text_document.uri)?;

    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok(None),
    };

    let pos = match convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )? {
        Some(pos) => pos,
        None => return Ok(None),
    };

    let line_index = match snapshot.analysis_snapshot.line_index(file_id)? {
        Some(line_index) => line_index,
        None => return Ok(None),
    };

    let to_lsp_location = |location: Location| -> Option<lsp_types::Location> {
        let range = convert::lsp_range_from_text_range(location.range, &line_index);
        Some(lsp_types::Location {
            uri: lsp_types::Url::from_file_path(
                snapshot
                    .document_manager
                    .read()
                    .lookup_by_file_id(location.file_id),
            )
            .ok()?,
            range,
        })
    };

    Ok(Some(
        snapshot
            .analysis_snapshot
            .goto_definition(FilePosition { file_id, pos })?
            .unwrap_or_else(|| Vec::new())
            .into_iter()
            .flat_map(to_lsp_location)
            .collect::<Vec<_>>()
            .into(),
    ))
}

pub(crate) fn completion(
    snapshot: &ServerSnapshot,
    params: lsp_types::CompletionParams,
) -> anyhow::Result<Option<lsp_types::CompletionResponse>> {
    let path = path_buf_from_url(&params.text_document_position.text_document.uri)?;

    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok(None),
    };

    let pos = match convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position.position,
    )? {
        Some(pos) => pos,
        None => return Ok(None),
    };

    Ok(Some(
        snapshot
            .analysis_snapshot
            .completion(FilePosition { file_id, pos })?
            .unwrap_or_else(|| Vec::new())
            .into_iter()
            .map(|item| lsp_types::CompletionItem {
                label: item.label,
                kind: Some(match item.kind {
                    CompletionItemKind::Function => lsp_types::CompletionItemKind::FUNCTION,
                    CompletionItemKind::Variable => lsp_types::CompletionItemKind::VARIABLE,
                    CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
                    // TODO(withered-magic): Only choosing `INTERFACE` because it looks cooler in VSCode :D
                    CompletionItemKind::Class => lsp_types::CompletionItemKind::CLASS,
                }),
                ..Default::default()
            })
            .collect::<Vec<_>>()
            .into(),
    ))
}

pub(crate) fn hover(
    snapshot: &ServerSnapshot,
    params: lsp_types::HoverParams,
) -> anyhow::Result<Option<lsp_types::Hover>> {
    let path = path_buf_from_url(&params.text_document_position_params.text_document.uri)?;

    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok(None),
    };

    let pos = match convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )? {
        Some(pos) => pos,
        None => return Ok(None),
    };

    Ok(snapshot
        .analysis_snapshot
        .hover(FilePosition { file_id, pos })?
        .map(|hover| lsp_types::Hover {
            contents: lsp_types::HoverContents::Markup(lsp_types::MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: hover.contents.value,
            }),
            range: None,
        }))
}
