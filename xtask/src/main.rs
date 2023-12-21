use clap::{Parser, Subcommand};

mod update_parser_test_data;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    UpdateParserTestData { filters: Vec<String> },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::UpdateParserTestData { filters }) => update_parser_test_data::run(filters),
        None => Ok(()),
    }
}
