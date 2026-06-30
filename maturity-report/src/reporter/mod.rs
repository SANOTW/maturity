//! Reporting and presentation
//!
//! Reporter is responsible for displaying collected metrics to users.
//!
//! Collection and presentation are intentionally separated so that future output formats can be implemented without
//! affecting the collection logic.
//!

use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use tracing::instrument;

/// Displays collected project metrics.
#[maturity]
pub struct Reporter;

impl Reporter {
    #[maturity]
    #[instrument(level = "trace", name = "Reporter/new", skip_all)]
    pub fn new() -> Self {
        Self {}
    }

    /// Prints collected metrics to the terminal.
    #[maturity]
    #[instrument(level = "trace", skip_all)]
    pub fn display(&self, metrics: &MetricCount) {
        println!("Maturity Package Version: {}", env!("CARGO_PKG_VERSION"));
        println!("Inventory counting: Implemented - partially");
        println!("Scoring: Not Implemented");
        println!("Analysis: Not Implemented");
        println!("----------------------------------");
        println!("{}", metrics)
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}
