use anyhow::Ok;
use starpls_ide::{FilePosition, Location};

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
) -> anyhow::Result<lsp_types::GotoDefinitionResponse> {
    let res = || Ok(Vec::<lsp_types::Location>::new().into());
    let path = path_buf_from_url(&params.text_document_position_params.text_document.uri)?;

    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return res(),
    };

    let pos = match convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )? {
        Some(pos) => pos,
        None => return res(),
    };

    let line_index = match snapshot.analysis_snapshot.line_index(file_id)? {
        Some(line_index) => line_index,
        None => return res(),
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

    Ok(snapshot
        .analysis_snapshot
        .goto_definition(FilePosition { file_id, pos })?
        .unwrap_or_else(|| Vec::new())
        .into_iter()
        .flat_map(to_lsp_location)
        .collect::<Vec<_>>()
        .into())
}

pub(crate) fn completion(
    snapshot: &ServerSnapshot,
    params: lsp_types::CompletionParams,
) -> anyhow::Result<lsp_types::CompletionResponse> {
    let res = || Ok(Vec::<lsp_types::CompletionItem>::new().into());
    let path = path_buf_from_url(&params.text_document_position.text_document.uri)?;

    let file_id = match snapshot.document_manager.read().lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return res(),
    };

    let pos = match convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position.position,
    )? {
        Some(pos) => pos,
        None => return res(),
    };

    Ok(snapshot
        .analysis_snapshot
        .completion(FilePosition { file_id, pos })?
        .unwrap_or_else(|| Vec::new())
        .into_iter()
        .map(|item| lsp_types::CompletionItem {
            label: item.label,
            kind: Some(lsp_types::CompletionItemKind::VARIABLE),
            ..Default::default()
        })
        .collect::<Vec<_>>()
        .into())
}
