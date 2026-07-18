//! Command-line interfaces for `maturity`
//!
//! This module contains the shared CLI implementation used by both `cargo-maturity` and `maturity-cli`
//!

use clap::Parser;
use maturity_macro::maturity;

mod commands;

use crate::cli::commands::{
    Commands, annotations::Annotations, inventory::Inventory, report::Report,
};

#[maturity]
#[derive(Parser)]
#[command(version, about, long_about= None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[maturity]
// TODO: Implementation off the cli parsing to be given to the cargo-maturity and the maturity-report.
// TODO: Implementation of MaturityResult<()>
pub fn run(args: Vec<String>) {
    // let _ = initialise_logging();

    let cli = Cli::parse_from(args);

    match &cli.command.unwrap_or(Commands::Report) {
        Commands::Report => {
            Report::new().run();
        }
        Commands::Inventory => {
            Inventory::new().run();
        }
        Commands::Annotations => {
            Annotations::new().run();
        }
    }
}
