//! Report command implementation.
//!
//! The report command coordinates the complete reporting pipeline:
//!
//! Scanner -> Collector -> Reporter
//!
//! Each component has a single responsibility and operates independently from the others.
//!

use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use tracing::instrument;

use crate::{collector::Collector, reporter::Reporter, scanner::Scanner};

#[maturity]
pub struct Report {
    metrics: MetricCount,
    scanner: Scanner,
    collector: Collector,
    reporter: Reporter,
}

impl Report {
    #[maturity]
    #[instrument(level = "trace", name = "Report/new", skip_all)]
    pub fn new() -> Self {
        // Initialising Objects
        let metrics = MetricCount::new();
        let scanner = Scanner::new();
        let collector = Collector::new();
        let reporter = Reporter::new();

        Self {
            metrics,
            scanner,
            collector,
            reporter,
        }
    }

    /// Executes the complete reporting pipeline.
    ///
    /// execution flow:
    /// Scan the project -> collecting paths and counting -> displaying output
    #[maturity]
    #[instrument(level = "trace", skip_all)]
    pub fn run(&mut self) {
        // Scanning the project
        let paths = self.scanner.scan(&mut self.metrics);
        // Collecting the paths and counting
        self.collector.collect(paths, &mut self.metrics);
        // Displaying output
        self.reporter.display(&self.metrics);
    }
}
