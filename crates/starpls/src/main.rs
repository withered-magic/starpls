use lsp_server::Connection;
use lsp_types::{OneOf, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};

mod convert;
mod diagnostics;
mod document;
mod event_loop;
mod extensions;
mod handlers;
mod server;
mod task_pool;
mod utils;

fn main() -> anyhow::Result<()> {
    eprintln!("server: starpls, v0.1.0");

    // Create the transport over stdio.
    let (connection, io_threads) = Connection::stdio();

    // Initialize the connection with server capabilities. For now, this consists
    // only of `TextDocumentSyncKind.Full`.
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        completion_provider: Some(Default::default()),
        definition_provider: Some(OneOf::Left(true)),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        ..Default::default()
    })?;
    let initialize_params = serde_json::from_value(connection.initialize(server_capabilities)?)?;

    event_loop::process_connection(connection, initialize_params)?;

    // Graceful shutdown.
    eprintln!("server: connection closed, exiting");
    io_threads.join()?;

    Ok(())
}
