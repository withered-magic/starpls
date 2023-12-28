use crate::{convert::path_buf_from_url, extensions::ShowSyntaxTreeParams, server::ServerSnapshot};

pub(crate) fn show_syntax_tree(
    snapshot: &ServerSnapshot,
    params: ShowSyntaxTreeParams,
) -> anyhow::Result<String> {
    eprintln!("enter show syntax tree");
    let document_manager = snapshot.document_manager.read();
    let path = path_buf_from_url(&params.text_document.uri)?;
    let file_id = match document_manager.lookup_by_path_buf(&path) {
        Some(file_id) => file_id,
        None => return Ok("".to_string()),
    };
    let rendered_syntax_tree = snapshot.analysis_snapshot.show_syntax_tree(file_id)?;
    eprintln!("return show syntax tree");
    Ok(rendered_syntax_tree.unwrap_or_else(|| "".to_string()))
}
