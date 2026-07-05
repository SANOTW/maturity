//! Reporting and presentation
//!
//! Reporter is responsible for displaying collected metrics to users.
//!
//! Collection and presentation are intentionally separated so that future output formats can be implemented without
//! affecting the collection logic.
//!

use comfy_table::{
    Attribute, Cell, CellAlignment, Color, ContentArrangement, Table,
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
};
use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use owo_colors::OwoColorize;
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
        // TODO: Transfer it into the Reporter struct as field
        // To ensure the entire report uses a consistent width
        const FULL_WIDTH: u16 = 80;
        const LABEL_WIDTH: usize = 20;

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

        // Version
        println!(
            "{:<LABEL_WIDTH$} {}",
            "Inventory counting".bold().cyan(),
            "Implemented (partial)".yellow()
        );

        println!(
            "{:<LABEL_WIDTH$} {}",
            "Scoring".bold().cyan(),
            "Not Implemented".red()
        );
        println!(
            "{:<LABEL_WIDTH$} {}",
            "Analysis".bold().cyan(),
            "Not Implemented".red()
        );
        println!();

        // Project info

        println!("{}", "Project".bold().cyan());
        println!("{}", "-".repeat(FULL_WIDTH as usize));

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
        println!();

        // Report

        let mut items = Table::new();

        items
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth)
            .set_width(FULL_WIDTH);

        items.set_header([
            Cell::new("Item")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("Total Count")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
            Cell::new("Maturity Count")
                .fg(Color::Cyan)
                .add_attribute(Attribute::Bold),
        ]);

        for (kind, metric) in metrics.items_iter() {
            items.add_row([
                Cell::new(kind.to_string()),
                Cell::new(metric.total().to_string()).set_alignment(CellAlignment::Right),
                Cell::new(metric.maturity().to_string()).set_alignment(CellAlignment::Right),
            ]);
        }

        println!("{items}");
    }
}

impl Default for Reporter {
    fn default() -> Self {
        Self::new()
    }
}
