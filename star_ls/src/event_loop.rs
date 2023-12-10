use lsp_server::Connection;
use lsp_types::InitializeParams;

pub fn process_connection(
    connection: Connection,
    _initialize_params: InitializeParams,
) -> anyhow::Result<()> {
    eprintln!("server: initializing state and starting event loop");
    Ok(())
}
