use anyhow::Ok;
use starpls_ide::CompletionItemKind;
use starpls_ide::CompletionMode::InsertText;
use starpls_ide::CompletionMode::TextEdit;
use starpls_ide::Edit;
use starpls_ide::FilePosition;

use crate::convert::path_buf_from_url;
use crate::convert::{self};
use crate::extensions::ShowHirParams;
use crate::extensions::ShowSyntaxTreeParams;
use crate::server::ServerSnapshot;
use crate::utils::response_from_locations;

macro_rules! try_opt {
    ($expr:expr) => {
        match { $expr } {
            Some(res) => res,
            None => return Ok(None),
        }
    };
}

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
    goto_definition_impl(
        snapshot,
        params,
        snapshot.config.args.goto_definition_skip_re_exports,
    )
}

pub(crate) fn goto_declaration(
    snapshot: &ServerSnapshot,
    params: lsp_types::GotoDefinitionParams,
) -> anyhow::Result<Option<lsp_types::GotoDefinitionResponse>> {
    goto_definition_impl(snapshot, params, true)
}

fn goto_definition_impl(
    snapshot: &ServerSnapshot,
    params: lsp_types::GotoDefinitionParams,
    skip_re_exports: bool,
) -> anyhow::Result<Option<lsp_types::GotoDefinitionResponse>> {
    let path = path_buf_from_url(&params.text_document_position_params.text_document.uri)?;
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let pos = try_opt!(convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )?);
    let resp = response_from_locations(
        snapshot,
        file_id,
        snapshot
            .analysis_snapshot
            .goto_definition(FilePosition { file_id, pos }, skip_re_exports)?
            .unwrap_or_else(Vec::new)
            .into_iter(),
    );
    Ok(Some(resp))
}

pub(crate) fn find_references(
    snapshot: &ServerSnapshot,
    params: lsp_types::ReferenceParams,
) -> anyhow::Result<Option<Vec<lsp_types::Location>>> {
    let path = path_buf_from_url(&params.text_document_position.text_document.uri)?;
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let line_index = try_opt!(snapshot.analysis_snapshot.line_index(file_id)?);
    let pos = try_opt!(convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position.position,
    )?);
    let resp = snapshot
        .analysis_snapshot
        .find_references(FilePosition { file_id, pos })?
        .unwrap_or_else(Vec::new)
        .into_iter()
        .filter_map(|location| {
            Some(lsp_types::Location {
                range: convert::lsp_range_from_text_range(location.range, line_index)?,
                uri: lsp_types::Url::from_file_path(
                    snapshot
                        .document_manager
                        .read()
                        .lookup_by_file_id(location.file_id),
                )
                .ok()?,
            })
        });

    Ok(Some(resp.collect()))
}

pub(crate) fn completion(
    snapshot: &ServerSnapshot,
    params: lsp_types::CompletionParams,
) -> anyhow::Result<Option<lsp_types::CompletionResponse>> {
    let path = path_buf_from_url(&params.text_document_position.text_document.uri)?;
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let line_index = try_opt!(snapshot.analysis_snapshot.line_index(file_id)?);
    let pos = try_opt!(convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position.position,
    )?);

    Ok(Some(
        snapshot
            .analysis_snapshot
            .completions(
                FilePosition { file_id, pos },
                params.context.and_then(|cx| cx.trigger_character),
            )?
            .unwrap_or_else(Vec::new)
            .into_iter()
            .flat_map(|item| {
                let sort_text = Some(item.sort_text());
                let (insert_text, text_edit) = match item.mode {
                    Some(mode) => match mode {
                        InsertText(text) => (Some(text), None),
                        TextEdit(edit) => (
                            None,
                            Some(match edit {
                                Edit::TextEdit(edit) => {
                                    lsp_types::CompletionTextEdit::Edit(lsp_types::TextEdit {
                                        range: convert::lsp_range_from_text_range(
                                            edit.range, line_index,
                                        )?,
                                        new_text: edit.new_text,
                                    })
                                }
                                Edit::InsertReplaceEdit(edit)
                                    if snapshot.config.has_insert_replace_support() =>
                                {
                                    lsp_types::CompletionTextEdit::InsertAndReplace(
                                        lsp_types::InsertReplaceEdit {
                                            new_text: edit.new_text,
                                            insert: convert::lsp_range_from_text_range(
                                                edit.insert,
                                                line_index,
                                            )?,
                                            replace: convert::lsp_range_from_text_range(
                                                edit.replace,
                                                line_index,
                                            )?,
                                        },
                                    )
                                }
                                _ => return None,
                            }),
                        ),
                    },
                    None => (None, None),
                };

                Some(lsp_types::CompletionItem {
                    label: item.label,
                    kind: Some(match item.kind {
                        CompletionItemKind::Function => lsp_types::CompletionItemKind::FUNCTION,
                        CompletionItemKind::Field => lsp_types::CompletionItemKind::FIELD,
                        CompletionItemKind::Variable => lsp_types::CompletionItemKind::VARIABLE,
                        CompletionItemKind::Class => lsp_types::CompletionItemKind::CLASS,
                        CompletionItemKind::Module => lsp_types::CompletionItemKind::MODULE,
                        CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
                        CompletionItemKind::File => lsp_types::CompletionItemKind::FILE,
                        CompletionItemKind::Folder => lsp_types::CompletionItemKind::FOLDER,
                        CompletionItemKind::Constant => lsp_types::CompletionItemKind::CONSTANT,
                    }),
                    sort_text,
                    insert_text,
                    text_edit,
                    filter_text: item.filter_text,
                    ..Default::default()
                })
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
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let pos = try_opt!(convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )?);
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

pub(crate) fn signature_help(
    snapshot: &ServerSnapshot,
    params: lsp_types::SignatureHelpParams,
) -> anyhow::Result<Option<lsp_types::SignatureHelp>> {
    let path = path_buf_from_url(&params.text_document_position_params.text_document.uri)?;
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let pos = try_opt!(convert::text_size_from_lsp_position(
        snapshot,
        file_id,
        params.text_document_position_params.position,
    )?);
    Ok(snapshot
        .analysis_snapshot
        .signature_help(FilePosition { file_id, pos })?
        .map(|help| lsp_types::SignatureHelp {
            signatures: help
                .signatures
                .into_iter()
                .map(|sig| lsp_types::SignatureInformation {
                    label: sig.label,
                    documentation: sig.documentation.map(to_markup_doc),
                    parameters: sig.parameters.map(|params| {
                        params
                            .into_iter()
                            .map(|param| lsp_types::ParameterInformation {
                                label: lsp_types::ParameterLabel::Simple(param.label),
                                documentation: param.documentation.map(to_markup_doc),
                            })
                            .collect()
                    }),
                    active_parameter: sig.active_parameter.map(|i| i as u32),
                })
                .collect(),
            active_signature: None,
            active_parameter: None,
        }))
}

pub(crate) fn document_symbols(
    snapshot: &ServerSnapshot,
    params: lsp_types::DocumentSymbolParams,
) -> anyhow::Result<Option<lsp_types::DocumentSymbolResponse>> {
    let path = path_buf_from_url(&params.text_document.uri)?;
    let file_id = try_opt!(snapshot.document_manager.read().lookup_by_path_buf(&path));
    let line_index = try_opt!(snapshot.analysis_snapshot.line_index(file_id)?);
    Ok(snapshot
        .analysis_snapshot
        .document_symbols(file_id)?
        .map(|symbols| {
            symbols
                .into_iter()
                .filter_map(|symbol| convert::lsp_document_symbol_from_native(symbol, line_index))
                .collect::<Vec<_>>()
                .into()
        }))
}

fn to_markup_doc(doc: String) -> lsp_types::Documentation {
    lsp_types::Documentation::MarkupContent(lsp_types::MarkupContent {
        kind: lsp_types::MarkupKind::Markdown,
        value: doc,
    })
}
