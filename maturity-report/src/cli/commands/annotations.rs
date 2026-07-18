use maturity_core::{annotation::AnnotatedItemInventory, metrics::MetricCount};
use maturity_macro::maturity;
use tracing::instrument;

use crate::{
    collector::Collector, reporter::annotation::Annotation as ReporterAnnotation, scanner::Scanner,
};

#[maturity(experimental)]
pub struct Annotations {
    scanner: Scanner,
    collector: Collector,
    metrics: MetricCount,
    annotations: AnnotatedItemInventory,
    reporter: ReporterAnnotation,
}

impl Annotations {
    #[instrument(level = "trace", name = "Annotations/new", skip_all)]
    pub fn new() -> Self {
        let scanner = Scanner::new();
        let collector = Collector::new();
        let metrics = MetricCount::new();
        let annotations = AnnotatedItemInventory::new();
        let reporter = ReporterAnnotation::new();

        Self {
            scanner,
            collector,
            metrics,
            annotations,
            reporter,
        }
    }

    #[maturity(pending)]
    #[instrument(level = "trace", skip_all)]
    pub fn run(&mut self) {
        // Scanning project
        // Metric Count
        let paths = self.scanner.scan(&mut self.metrics);
        // Collecting the paths, counting and annotation storing
        self.collector
            .collect_with_annotations(paths, &mut self.metrics, &mut self.annotations);
        // Displaying output
        self.reporter.display(&self.metrics, &self.annotations);
    }
}
