use crate::{convert, server::Server, utils::apply_document_content_changes};

pub(crate) fn did_open_text_document(
    server: &mut Server,
    params: lsp_types::DidOpenTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    server.document_manager.write().open(
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
    server.document_manager.write().close(&path);
    Ok(())
}

pub(crate) fn did_change_text_document(
    server: &mut Server,
    params: lsp_types::DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let mut document_manager = server.document_manager.write();
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    if let Some(file_id) = document_manager.lookup_by_path_buf(&path) {
        let contents = document_manager
            .get(file_id)
            .map(|document| document.contents.clone())
            .expect("lookup contents of non-existent file");
        let contents = apply_document_content_changes(contents, params.content_changes);
        document_manager.modify(file_id, contents, Some(params.text_document.version))
    }
    Ok(())
}

pub(crate) fn did_save_text_document(
    server: &mut Server,
    params: lsp_types::DidSaveTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    if server
        .document_manager
        .read()
        .lookup_by_path_buf(&path)
        .is_some()
    {
        if let Some("MODULE.bazel" | "WORKSPACE" | "WORKSPACE.bazel") =
            path.file_name().and_then(|file_name| file_name.to_str())
        {
            server.bazel_client.clear_repo_mappings();
            server.fetched_repos.clear();
        }
    }
    Ok(())
}
