//! Report command implementation.
//!
//! The report command coordinates the complete reporting pipeline:
//!
//! Scanner -> Collector -> Reporter
//!
//! Each component has a single responsibility and operates independently from the others.
//!

use maturity_macro::maturity;
use tracing::instrument;

use crate::cli::commands::inventory::Inventory;

#[maturity(experimental)]
pub struct Report;

impl Report {
    #[instrument(level = "trace", name = "Report/new", skip_all)]
    pub fn new() -> Self {
        Self
    }

    /// Executes the complete reporting pipeline.
    ///
    /// execution flow:
    /// Scan the project -> collecting paths and counting -> displaying output
    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn run(&mut self) {
        Inventory::new().run();
    }
}
