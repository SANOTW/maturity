use comfy_table::{
    Attribute, Cell, CellAlignment, Color, Row, Table, modifiers::UTF8_ROUND_CORNERS,
    presets::UTF8_FULL,
};
use maturity_core::{
    annotation::{AnnotatedItemInfo, AnnotatedItemInventory},
    metrics::MetricCount,
};
use maturity_macro::maturity;
use owo_colors::OwoColorize;
use tracing::instrument;

use crate::reporter::{FULL_WIDTH, Reporter};

use crate::reporter::LABEL_WIDTH;

// -----------------------------------------------------

/// Displays collected project metrics.
#[maturity(experimental)]
pub struct Annotation;

impl Annotation {
    #[instrument(level = "trace", name = "Annotation/new", skip_all)]
    pub fn new() -> Self {
        Self {}
    }

    /// Prints collected metrics to the terminal.
    #[maturity(experimental, todo = "refactor")]
    #[instrument(level = "trace", skip_all)]
    pub fn display(&self, metrics: &MetricCount, inventory: &AnnotatedItemInventory) {
        let reporter = Reporter::new();

        reporter.display_header();
        reporter.display_capabilities();
        reporter.display_project_info(metrics);
        println!();

        // -------------------------------------------------------------------------

        // Report

        let headers = vec!["Path", "Kind", "Name", "Attributes"];

        let report = [(
            "Annotation",
            self.collect_metrics(inventory.items_iter().collect()),
        )];

        for (label, array) in report {
            let items: Vec<AnnotatedItemInfo> = array;

            if !items.is_empty() {
                let report = self.report_table(label, &headers, /*metrics,*/ inventory, items);
                println!("{report}");
            }
        }
    }
}

// helper functions
impl Annotation {
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics(&self, iter: Vec<&AnnotatedItemInfo>) -> Vec<AnnotatedItemInfo> {
        iter.into_iter()
            .map(|item_info| item_info.clone())
            .collect()
    }

    #[maturity(experimental, refactor = "some")]
    fn report_table(
        &self,
        label: &str,
        headers: &Vec<&str>,
        // metrics: &MetricCount,
        inventory: &AnnotatedItemInventory,
        inventory_items: Vec<AnnotatedItemInfo>,
    ) -> Table {
        Reporter.label(label);

        let mut items = Table::new();

        items
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
            .set_width(FULL_WIDTH);

        let mut header_row = Row::new();

        for header in headers {
            header_row.add_cell(
                Cell::new(header)
                    .fg(Color::Cyan)
                    .add_attribute(Attribute::Bold),
            );
        }

        self.display_project_info(/*metrics,*/ inventory);

        items.set_header(header_row);

        // ---
        for annotated_item_info in inventory_items {
            items.add_row([
                Cell::new(annotated_item_info.path.to_str().unwrap()),
                Cell::new(annotated_item_info.kind),
                Cell::new(annotated_item_info.name.unwrap_or("N/A".to_string()))
                    .set_alignment(CellAlignment::Left),
                Cell::new(annotated_item_info.attributes).set_alignment(CellAlignment::Left),
            ]);
        }

        items
    }

    #[instrument(level = "trace", skip_all)]
    pub fn display_project_info(
        &self,
        // metrics: &MetricCount,
        inventory_items: &AnnotatedItemInventory,
    ) {
        let hash = inventory_items.annotation_summary_header();

        for (item, value) in hash {
            self.item(item, value.to_string());
        }
    }

    #[instrument(level = "trace", skip_all)]
    fn item(&self, label: String, value: String) {
        println!("{:<LABEL_WIDTH$} {}", label.bold(), value);
    }
}

impl Default for Annotation {
    fn default() -> Self {
        Self::new()
    }
}
