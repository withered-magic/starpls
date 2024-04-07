use clap::{Parser, Subcommand};
use lsp_server::Connection;
use lsp_types::{
    CompletionOptions, HoverProviderCapability, OneOf, ServerCapabilities, SignatureHelpOptions,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

mod convert;
mod debouncer;
mod diagnostics;
mod dispatcher;
mod document;
mod event_loop;
mod extensions;
mod handlers;
mod server;
mod task_pool;
mod utils;

const COMPLETION_TRIGGER_CHARACTERS: &[char] = &['.', '"', '\'', '/', ':'];
const SIGNATURE_HELP_TRIGGER_CHARACTERS: &[char] = &['(', ',', ')'];

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check,
    Server,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Check) => Ok(()),
        Some(Commands::Server) | None => run_server(),
    }
}

fn run_server() -> anyhow::Result<()> {
    eprintln!("server: starpls, v0.1.8");

    // Create the transport over stdio.
    let (connection, io_threads) = Connection::stdio();

    // Initialize the connection with server capabilities. For now, this consists
    // only of `TextDocumentSyncKind.Full`.
    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(make_trigger_characters(COMPLETION_TRIGGER_CHARACTERS)),
            ..Default::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(make_trigger_characters(SIGNATURE_HELP_TRIGGER_CHARACTERS)),
            ..Default::default()
        }),
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

fn make_trigger_characters(chars: &[char]) -> Vec<String> {
    chars.iter().map(|c| c.to_string()).collect()
}
