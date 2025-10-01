use clap::Parser;
use clap::Subcommand;
use commands::check::CheckCommand;
use commands::server::ServerCommand;

mod bazel;
mod commands;
mod config;
mod convert;
mod debouncer;
mod diagnostics;
mod dispatcher;
mod document;
mod event_loop;
mod extensions;
mod handlers;
mod plugin;
mod server;
mod task_pool;
mod utils;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Starpls is an LSP implementation for Starlark, the configuration language used by Bazel and Buck2.
#[derive(Subcommand)]
enum Commands {
    /// Analyze the specified Starlark files and report errors.
    Check(CheckCommand),

    /// Start the language server.
    Server(ServerCommand),

    /// Print version information and exit.
    Version,
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
        Some(Commands::Check(cmd)) => cmd.run(),
        Some(Commands::Server(cmd)) => cmd.run(),
        Some(Commands::Version) => run_version(),
        None => ServerCommand::default().run(),
    }
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
