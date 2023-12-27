use crate::{convert, server::Server, utils::apply_document_content_changes};

pub(crate) fn did_open_text_document(
    server: &mut Server,
    params: lsp_types::DidOpenTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    server.document_manager.open(
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
    server.document_manager.close(&path);
    Ok(())
}

pub(crate) fn did_change_text_document(
    server: &mut Server,
    params: lsp_types::DidChangeTextDocumentParams,
) -> anyhow::Result<()> {
    let path = convert::path_buf_from_url(&params.text_document.uri)?;
    // let file_id = server.document_manager.lookup_file_id(&path);
    // if let Some(file_id) = file_id {
    //     let contents = server
    //         .document_manager
    //         .contents(file_id)
    //         .expect("lookup contents of non-existent file");
    //     let (contents, edits) =
    //         apply_document_content_changes(contents.to_string(), params.content_changes);
    //     server
    //         .document_manager
    //         .modify(path, contents, None, Edit::Incremental(edits));
    // };

    Ok(())
}
