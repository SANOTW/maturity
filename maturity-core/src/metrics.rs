use core::fmt;

use maturity_macro::maturity;
use tracing::instrument;

/// Aggregated metrics collected from a Rust project.
///
/// Stores inventory information which into the fields.
///
/// This struct essentially the foundation for the entire system. However as time goes this will change if need be.
#[maturity]
pub struct MetricCount {
    project_name: String,

    total_files_count: u64,
    rust_files_count: u64,

    structs_count: u64,
    maturity_structs_count: u64,
    enums_count: u64,
    maturity_enums_count: u64,
    traits_count: u64,
    maturity_traits_count: u64,
    functions_count: u64,
    maturity_functions_count: u64,

    total: u64,
}

impl MetricCount {
    /// Creates an empty metric collection.
    #[maturity]
    #[instrument(level = "trace", name = "Metrics/new", skip_all)]
    pub fn new() -> Self {
        Self {
            project_name: String::new(),
            total_files_count: 0,
            rust_files_count: 0,
            structs_count: 0,
            maturity_structs_count: 0,
            enums_count: 0,
            maturity_enums_count: 0,
            traits_count: 0,
            maturity_traits_count: 0,
            functions_count: 0,
            maturity_functions_count: 0,
            total: 0,
        }
    }

    #[maturity]
    #[instrument(level = "trace", skip_all)]
    pub fn from(&mut self, metric: MetricCount) -> MetricCount {
        Self {
            project_name: metric.project_name().to_string(),
            total_files_count: metric.total_files_count(),
            rust_files_count: metric.rust_files_count(),
            structs_count: metric.structs_count(),
            maturity_structs_count: metric.maturity_structs_count(),
            enums_count: metric.enums_count(),
            maturity_enums_count: metric.maturity_enums_count(),
            traits_count: metric.traits_count(),
            maturity_traits_count: metric.maturity_traits_count(),
            functions_count: metric.functions_count(),
            maturity_functions_count: metric.maturity_functions_count(),
            total: metric.total(),
        }
    }
}

impl Default for MetricCount {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MetricCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project: {}", self.project_name)?;
        writeln!(f, "Total Files Count: {}", self.total_files_count)?;
        writeln!(f, "Rust Files Count: {}", self.rust_files_count)?;
        let _ = writeln!(f);
        let _ = writeln!(f, "Items");
        let _ = writeln!(f, "------");
        let _ = writeln!(f);
        writeln!(f, "Structs Count: {}", self.structs_count)?;
        writeln!(f, "Enums Count: {}", self.enums_count)?;
        writeln!(f, "Traits Count: {}", self.traits_count)?;
        writeln!(f, "Functions Count: {}", self.functions_count)?;

        let _ = writeln!(f);
        let _ = writeln!(f, "Maturity Items");
        let _ = writeln!(f, "------");
        let _ = writeln!(f);
        writeln!(f, "Maturity Structs Count: {}", self.maturity_structs_count)?;
        writeln!(f, "Maturity Enums Count: {}", self.maturity_enums_count)?;
        writeln!(f, "Maturity Traits Count: {}", self.maturity_traits_count)?;
        writeln!(
            f,
            "Maturity Functions Count: {}",
            self.maturity_functions_count
        )
    }
}

/// Getters and Setters
///
/// Used to maintain a stable interface between crates.
///
/// This may eventually be replaced by generated code once the helper crate is implemented. Not replaced but this entire
/// impl Block will get nuked.
impl MetricCount {
    #[instrument(level = "trace", skip_all)]
    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_project_name(&mut self, project_name: String) {
        self.project_name = project_name
    }

    #[instrument(level = "trace", skip_all)]
    pub fn total_files_count(&self) -> u64 {
        self.total_files_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_total_files_count(&mut self, total_files_count: u64) {
        self.total_files_count = total_files_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_total_files_count(&mut self) {
        self.total_files_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn rust_files_count(&self) -> u64 {
        self.rust_files_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_rust_files_count(&mut self, rust_files_count: u64) {
        self.rust_files_count = rust_files_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_rust_files_count(&mut self) {
        self.rust_files_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn structs_count(&self) -> u64 {
        self.structs_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn maturity_structs_count(&self) -> u64 {
        self.maturity_structs_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_maturity_structs_count(&mut self, maturity_structs_count: u64) {
        self.maturity_structs_count = maturity_structs_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_structs_count(&mut self, structs_count: u64) {
        self.structs_count = structs_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_structs_count(&mut self) {
        self.structs_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_maturity_structs_count(&mut self) {
        self.maturity_structs_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn enums_count(&self) -> u64 {
        self.enums_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn maturity_enums_count(&self) -> u64 {
        self.maturity_enums_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_maturity_enums_count(&mut self, maturity_enums_count: u64) {
        self.maturity_enums_count = maturity_enums_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_enums_count(&mut self, enums_count: u64) {
        self.enums_count = enums_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_enums_count(&mut self) {
        self.enums_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_maturity_enums_count(&mut self) {
        self.maturity_enums_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn traits_count(&self) -> u64 {
        self.traits_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn maturity_traits_count(&self) -> u64 {
        self.maturity_traits_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_maturity_traits_count(&mut self, maturity_traits_count: u64) {
        self.maturity_traits_count = maturity_traits_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_traits_count(&mut self, traits_count: u64) {
        self.traits_count = traits_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_traits_count(&mut self) {
        self.traits_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_maturity_traits_count(&mut self) {
        self.maturity_traits_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn functions_count(&self) -> u64 {
        self.functions_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn maturity_functions_count(&self) -> u64 {
        self.maturity_functions_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_functions_count(&mut self, functions_count: u64) {
        self.functions_count = functions_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_maturity_functions_count(&mut self, maturity_functions_count: u64) {
        self.maturity_functions_count = maturity_functions_count
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_functions_count(&mut self) {
        self.functions_count += 1
    }

    #[instrument(level = "trace", skip_all)]
    pub fn increment_maturity_functions_count(&mut self) {
        self.maturity_functions_count += 1
    }
}

impl MetricCount {
    pub fn total(&self) -> u64 {
        self.total
    }
}
