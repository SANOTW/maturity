use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use tracing::instrument;

use crate::reporter::inventory::Inventory;

#[maturity(experimental)]
pub struct Report;

impl Report {
    #[instrument(level = "trace", name = "Report/new", skip_all)]
    pub fn new() -> Self {
        Self
    }

    /// Prints collected metrics to the terminal.
    #[maturity(experimental, todo = "refactor")]
    #[instrument(level = "trace", skip_all)]
    pub fn display(&self, metrics: &MetricCount) {
        Inventory::new().display(metrics);
    }
}
