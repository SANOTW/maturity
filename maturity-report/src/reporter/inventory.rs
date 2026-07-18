use maturity_core::metrics::{ItemKind, ItemMetric, MetricCount};
use maturity_macro::maturity;
use tracing::instrument;

use crate::reporter::{ReportType, Reporter};

// -----------------------------------------------------

/// Displays collected project metrics.
#[maturity(experimental)]
pub struct Inventory;

impl Inventory {
    #[instrument(level = "trace", name = "Inventory/new", skip_all)]
    pub fn new() -> Self {
        Self {}
    }

    /// Prints collected metrics to the terminal.
    #[maturity(experimental, todo = "refactor")]
    #[instrument(level = "trace", skip_all)]
    pub fn display(&self, metrics: &MetricCount) {
        let reporter = Reporter::new();

        reporter.display_header();
        reporter.display_capabilities();
        reporter.display_project_info(metrics);
        println!();

        // -------------------------------------------------------------------------

        // Report

        reporter.label("Report");

        let headers = vec!["Item", "Total Count", "Maturity Count"];

        let report = [
            (
                "Items",
                self.collect_metrics(metrics.items_iter().collect()),
            ),
            (
                "Foreign Items",
                self.collect_metrics(metrics.foreign_items_iter().collect()),
            ),
            (
                "Implementation Items",
                self.collect_metrics(metrics.impl_items_iter().collect()),
            ),
            (
                "Trait Items",
                self.collect_metrics(metrics.trait_items_iter().collect()),
            ),
        ];

        for (label, array) in report {
            let items: Vec<(ItemKind, ItemMetric)> = array;

            if !items.is_empty() {
                let report = reporter.report_table(label, &headers, ReportType::Inventory(items));
                println!("{report}");
            }
        }
    }
}

// Helper functions
impl Inventory {
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics(&self, iter: Vec<(&ItemKind, &ItemMetric)>) -> Vec<(ItemKind, ItemMetric)> {
        iter.into_iter()
            .map(|(kind, metric)| (*kind, metric.clone()))
            .collect()
    }
}

// helper functions

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}
