use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use tracing::instrument;

use crate::{collector::Collector, reporter::report::Report, scanner::Scanner};

#[maturity(experimental)]
pub struct Inventory {
    scanner: Scanner,
    collector: Collector,
    metrics: MetricCount,
    reporter: Report,
}

impl Inventory {
    #[instrument(level = "trace", name = "Report/new", skip_all)]
    pub fn new() -> Self {
        // Initialising Objects
        let metrics = MetricCount::new();
        let scanner = Scanner::new();
        let collector = Collector::new();
        let reporter = Report::new();

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
    #[maturity(experimental)]
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
