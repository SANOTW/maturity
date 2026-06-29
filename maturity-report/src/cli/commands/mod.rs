//! Available CLI commands.
//!

use clap::Subcommand;

use maturity_macro::maturity;

pub mod report;

#[maturity]
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(
        name = "report",
        about = "Collect and report Rust code maturity metrics."
    )]
    Report,
}
