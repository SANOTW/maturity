//! Available CLI commands.
//!

use clap::Subcommand;

use maturity_macro::maturity;

pub mod annotations;
pub mod inventory;
pub mod report;

#[maturity]
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(name = "report", about = "Generate a maturity report for the project")]
    Report,
    #[command(
        name = "inventory",
        about = "Collect and display the project's Rust inventory"
    )]
    Inventory,
    #[command(
        name = "annotations",
        about = "Collect and display the annotated Rust construct's attributes with their values"
    )]
    Annotations,
}
