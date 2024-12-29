use clap::Args;
use log::info;
use lsp_server::Connection;
use lsp_types::CompletionOptions;
use lsp_types::HoverProviderCapability;
use lsp_types::OneOf;
use lsp_types::ServerCapabilities;
use lsp_types::SignatureHelpOptions;
use lsp_types::TextDocumentSyncCapability;
use lsp_types::TextDocumentSyncKind;

use crate::commands::InferenceOptions;
use crate::event_loop;
use crate::get_version;
use crate::make_trigger_characters;

const COMPLETION_TRIGGER_CHARACTERS: &[char] = &['.', '"', '\'', '/', ':', '@'];
const SIGNATURE_HELP_TRIGGER_CHARACTERS: &[char] = &['(', ',', ')'];

#[derive(Args, Default)]
pub(crate) struct ServerCommand {
    /// Path to the Bazel binary.
    #[clap(long = "bazel_path")]
    pub(crate) bazel_path: Option<String>,

    /// Enable completions for labels for targets in the current workspace.
    #[clap(
        long = "experimental_enable_label_completions",
        default_value_t = false
    )]
    pub(crate) enable_label_completions: bool,

    /// After receiving an edit event, the amount of time in milliseconds
    /// the server will wait for additional events before running analysis
    #[clap(long = "analysis_debounce_interval", default_value_t = 250)]
    pub(crate) analysis_debounce_interval: u64,

    #[command(flatten)]
    pub(crate) inference_options: InferenceOptions,
}

impl ServerCommand {
    pub(crate) fn run(self) -> anyhow::Result<()> {
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
                trigger_characters: Some(make_trigger_characters(
                    SIGNATURE_HELP_TRIGGER_CHARACTERS,
                )),
                ..Default::default()
            }),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            ..Default::default()
        })?;
        let initialize_params =
            serde_json::from_value(connection.initialize(server_capabilities)?)?;
        event_loop::process_connection(connection, self, initialize_params)?;

        // Graceful shutdown.
        info!("connection closed, exiting");
        io_threads.join()?;

        Ok(())
    }
}
