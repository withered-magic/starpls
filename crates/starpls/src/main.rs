use check::run_check;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use log::info;
use lsp_server::Connection;
use lsp_types::CompletionOptions;
use lsp_types::HoverProviderCapability;
use lsp_types::OneOf;
use lsp_types::ServerCapabilities;
use lsp_types::SignatureHelpOptions;
use lsp_types::TextDocumentSyncCapability;
use lsp_types::TextDocumentSyncKind;

mod bazel;
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

/// Starpls is an LSP implementation for Starlark, the configuration language used by Bazel and Buck2.
#[derive(Subcommand)]
enum Commands {
    /// Analyze the specified Starlark files and report errors.
    Check {
        /// Paths to typecheck.
        paths: Vec<String>,
        /// Path to the Bazel output base.
        #[clap(long = "output_base")]
        output_base: Option<String>,
    },
    /// Start the language server.
    Server(ServerArgs),
    /// Print version information and exit.
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

    /// Use code-flow analysis during typechecking.
    #[clap(long = "experimental_use_code_flow_analysis", default_value_t = false)]
    use_code_flow_analysis: bool,

    /// Enable completions for labels for targets in the current workspace.
    #[clap(
        long = "experimental_enable_label_completions",
        default_value_t = false
    )]
    enable_label_completions: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Don't do any log filtering when running the language server.
    if matches!(cli.command, Some(Commands::Server(_)) | None) {
        env_logger::Builder::from_default_env()
            .filter(Some("starpls"), log::LevelFilter::max())
            .init();
    } else {
        env_logger::init();
    }

    match cli.command {
        Some(Commands::Check { paths, output_base }) => run_check(paths, output_base),
        Some(Commands::Server(args)) => run_server(args),
        Some(Commands::Version) => run_version(),
        None => run_server(Default::default()),
    }
}

fn run_server(args: ServerArgs) -> anyhow::Result<()> {
    info!("starpls, v{}", get_version());

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
        references_provider: Some(OneOf::Left(true)),
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
    info!("connection closed, exiting");
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
