use check::run_check;
use clap::{Args, Parser, Subcommand};
use lsp_server::Connection;
use lsp_types::{
    CompletionOptions, HoverProviderCapability, OneOf, ServerCapabilities, SignatureHelpOptions,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

mod check;
mod config;
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

const COMPLETION_TRIGGER_CHARACTERS: &[char] = &['.', '"', '\'', '/', ':', '@'];
const SIGNATURE_HELP_TRIGGER_CHARACTERS: &[char] = &['(', ',', ')'];

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Check {
        /// Paths to typecheck.
        paths: Vec<String>,
        /// Path to the Bazel output base.
        #[clap(long = "output_base")]
        output_base: Option<String>,
    },
    Server(ServerArgs),
    Version,
}

#[derive(Args, Default)]
pub(crate) struct ServerArgs {
    /// Path to the Bazel binary.
    #[clap(long = "bazel_path")]
    bazel_path: Option<String>,
    /// Infer attributes on a rule implementation function's context parameter.
    #[clap(long = "experimental_infer_ctx_attributes", default_value_t = false)]
    infer_ctx_attributes: bool,
    #[clap(long = "experimental_use_code_flow_analysis", default_value_t = false)]
    use_code_flow_analysis: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Check { paths, output_base }) => run_check(paths, output_base),
        Some(Commands::Server(args)) => run_server(args),
        Some(Commands::Version) => run_version(),
        None => run_server(Default::default()),
    }
}

fn run_server(args: ServerArgs) -> anyhow::Result<()> {
    eprintln!("server: starpls, v{}", get_version());

    // Create the transport over stdio.
    let (connection, io_threads) = Connection::stdio();

    // Initialize the connection with server capabilities. For now, this consists
    // only of `TextDocumentSyncKind.Full`.
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(make_trigger_characters(COMPLETION_TRIGGER_CHARACTERS)),
            ..Default::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
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
    event_loop::process_connection(connection, args, initialize_params)?;

    // Graceful shutdown.
    eprintln!("server: connection closed, exiting");
    io_threads.join()?;

    Ok(())
}

fn run_version() -> anyhow::Result<()> {
    println!("starpls version: v{}", get_version());
    Ok(())
}

fn make_trigger_characters(chars: &[char]) -> Vec<String> {
    chars.iter().map(|c| c.to_string()).collect()
}

fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
