use crate::{convert, server::Server, utils::apply_document_content_changes};

pub(crate) fn did_open_text_document(
    server: &mut Server,
    params: lsp_types::DidOpenTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    let mut state = server.shared_file_state.0.write();
    state.open_document(
        path,
        params.text_document.version,
        params.text_document.text,
    );
    Ok(())
}

pub(crate) fn did_close_text_document(
    server: &mut Server,
    params: lsp_types::DidCloseTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    server.shared_file_state.0.write().close_document(&path);
    Ok(())
}

pub(crate) fn did_change_text_document(
    server: &mut Server,
    params: lsp_types::DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let mut state = server.shared_file_state.0.write();
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    if let Some(file_id) = state.interner.lookup_by_path_buf(&path) {
        let contents = state
            .get_document(file_id)
            .map(|document| document.contents.clone())
            .expect("lookup contents of non-existent file");
        let contents = apply_document_content_changes(contents, params.content_changes);
        state.modify_document(file_id, contents, Some(params.text_document.version))
    }
    Ok(())
}
