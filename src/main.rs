use anyhow::{Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
enum Commands {
    /// Print a string
    Echo {
        /// String to print
        #[arg(value_name = "STRING")]
        str: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Echo { str }) => {
            rush::commands::echo(str, std::io::stdout());
        }
        None => {}
    }

    Ok(())
}