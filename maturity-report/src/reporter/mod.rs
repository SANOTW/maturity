//! Reporting and presentation
//!
//! Reporter is responsible for displaying collected metrics to users.
//!
//! Collection and presentation are intentionally separated so that future output formats can be implemented without
//! affecting the collection logic.
//!

use comfy_table::{
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Row, Table,
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
};
use maturity_core::metrics::{ItemKind, ItemMetric, MetricCount};
use maturity_macro::maturity;
use owo_colors::OwoColorize;
use tracing::instrument;

pub mod annotation;
pub mod inventory;
pub mod report;

// TODO: Transfer it into the Inventory struct as field
// To ensure the entire report uses a consistent width
pub const FULL_WIDTH: u16 = 80;
pub const LABEL_WIDTH: usize = 21;

#[maturity]
pub struct Reporter;

impl Reporter {
    #[instrument(level = "trace", name = "Reporter/new", skip_all)]
    pub fn new() -> Self {
        Self
    }

    #[instrument(level = "trace", skip_all)]
    pub fn display_header(&self) {
        // Header
        let mut header = Table::new();

        header
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_width(FULL_WIDTH);

        header
            .add_row([
                Cell::new(format!("Maturity Report v{}", env!("CARGO_PKG_VERSION")))
                    .fg(Color::DarkCyan)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
            ])
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        println!("{header}");
    }

    #[instrument(level = "trace", skip_all)]
    pub fn display_capabilities(&self) {
        // Version
        println!(
            "{:<LABEL_WIDTH$} {}",
            "Inventory Collection".bold().cyan(),
            "Implemented".green()
        );

        println!(
            "{:<LABEL_WIDTH$} {}",
            "Annotation Collection".bold().cyan(),
            "In Progress".yellow()
        );

        println!(
            "{:<LABEL_WIDTH$} {}",
            "Scoring".bold().cyan(),
            "Planned".red()
        );

        println!(
            "{:<LABEL_WIDTH$} {}",
            "Analysis".bold().cyan(),
            "Planned".red()
        );
        println!();
    }

    #[instrument(level = "trace", skip_all)]
    pub fn display_project_info(&self, metrics: &MetricCount) {
        self.label("Project");

        println!("{:<LABEL_WIDTH$} {}", "Name".bold(), metrics.project_name());

        println!(
            "{:<LABEL_WIDTH$} {}",
            "File".bold(),
            &metrics.file().total()
        );
        println!(
            "{:<LABEL_WIDTH$} {}",
            "Rust Files".bold(),
            &metrics.file().rust()
        );
    }
}

impl Reporter {
    #[maturity(experimental, todo = "turn label into String? need to think over it")]
    fn label(&self, label: &str) {
        println!("{}", label.bold().cyan());
        println!("{}", "-".repeat(FULL_WIDTH as usize));
    }

    #[maturity(experimental)]
    fn report_table(&self, label: &str, headers: &Vec<&str>, report_type: ReportType) -> Table {
        self.label(label);

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

        items.set_header(header_row);

        // ---

        match report_type {
            ReportType::Inventory(inventory_items) => {
                for (kind, metric) in inventory_items {
                    items.add_row([
                        Cell::new(kind.to_string()),
                        Cell::new(metric.total().to_string()).set_alignment(CellAlignment::Right),
                        Cell::new(metric.maturity().to_string())
                            .set_alignment(CellAlignment::Right),
                    ]);
                }
            }
        }

        items
    }
}

#[maturity(experimental, todo = "move this enum block to proper location")]
pub enum ReportType {
    Inventory(Vec<(ItemKind, ItemMetric)>),
}
