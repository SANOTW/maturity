use core::fmt;
use std::collections::BTreeMap;

use maturity_macro::maturity;
use tracing::instrument;

/// Metrics tracking files related values.
///
/// ## Usage
///
/// ```rust
/// use maturity_core::metrics::FileMetric;
///
/// let mut metrics = FileMetric::default();
///
/// metrics.increment(false);
/// metrics.increment(true);
///
/// assert_eq!(metrics.total(), 2);
/// assert_eq!(metrics.rust(), 1);
/// ```
#[maturity(experimental)]
#[derive(Debug, Default, Clone)]
pub struct FileMetric {
    /// Total count of general files
    total: u64,
    /// Total count of rust files
    rust: u64,
}

impl FileMetric {
    #[instrument(level = "trace", skip_all)]
    pub fn increment(&mut self, is_rust_file: bool) {
        self.total += 1;

        if is_rust_file {
            self.rust += 1
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn total(&self) -> u64 {
        self.total
    }

    #[instrument(level = "trace", skip_all)]
    pub fn rust(&self) -> u64 {
        self.rust
    }
}

// ----------------------------------------------

/// Metrics tracking the values for the construct items.
/// ///
/// ## Usage
///
/// ```rust
/// use maturity_core::metrics::ItemMetric;
///
/// let mut metrics = ItemMetric::default();
///
/// metrics.increment(false);
/// metrics.increment(true);
///
/// assert_eq!(metrics.total(), 2);
/// assert_eq!(metrics.maturity(), 1);
/// ```
#[maturity(experimental)]
#[derive(Debug, Default, Clone)]
pub struct ItemMetric {
    /// Total count of the item
    total: u64,
    /// Total count of the item with `#[maturity]` annotation on it
    maturity: u64,
}

// -------------------
// Getters and setters
// -------------------
impl ItemMetric {
    #[instrument(level = "trace", skip_all)]
    pub fn increment(&mut self, has_maturity: bool) {
        self.total += 1;

        if has_maturity {
            self.maturity += 1;
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn total(&self) -> u64 {
        self.total
    }

    #[instrument(level = "trace", skip_all)]
    pub fn maturity(&self) -> u64 {
        self.maturity
    }
}

// ----------------------------------------------

/// ItemKinds matching the `syn` crate's `Item` enum as close as possible
#[maturity(experimental)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemKind {
    Const,
    Enum,
    ExternCrate,
    Function,
    ForeignMod,
    Implementation,
    Macro,
    Module,
    Static,
    Struct,
    Trait,
    TraitAlias,
    Type,
    Union,
    Use,
    Verbatim,
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Const => write!(f, "Consts"),
            ItemKind::Enum => write!(f, "Enums"),
            ItemKind::ExternCrate => write!(f, "External Crates"),
            ItemKind::Function => write!(f, "Functions"),
            ItemKind::ForeignMod => write!(f, "Foreign Modules"),
            ItemKind::Implementation => write!(f, "Implementations"),
            ItemKind::Macro => write!(f, "Macros"),
            ItemKind::Module => write!(f, "Modules"),
            ItemKind::Static => write!(f, "Statics"),
            ItemKind::Struct => write!(f, "Structs"),
            ItemKind::Trait => write!(f, "Traits"),
            ItemKind::TraitAlias => write!(f, "Trait Aliases"),
            ItemKind::Type => write!(f, "Types"),
            ItemKind::Union => write!(f, "Unions"),
            ItemKind::Use => write!(f, "Uses"),
            ItemKind::Verbatim => write!(f, "Verbatim"),
        }
    }
}

// ----------------------------------------------

/// Aggregated metrics collected from a Rust project.
///
/// Stores inventory information which are gathered into the fields.
///
/// This struct essentially the foundation for the entire system. However as time goes this will change if need be.
///
/// Item metrics are stored in a `BTreeMap` to guarantee deterministic iteration order when generating reports and terminal
/// output.
///
/// ## Usage
///
/// ```rust
/// use maturity_core::metrics::MetricCount;
/// use maturity_core::metrics::ItemKind;
///
/// let mut metric = MetricCount::new();
///
/// metric.file_mut().increment(true);
/// metric.items_mut(ItemKind::Function).increment(true);
///
/// assert_eq!(metric.file().rust(), 1);
/// assert_eq!(metric.items(ItemKind::Function).unwrap().maturity(), 1);
/// ```
#[maturity(experimental)]
#[derive(Default, Debug, Clone)]
pub struct MetricCount {
    project_name: String,

    file: FileMetric,

    /// Stores metrics for rust constructs
    ///
    /// A `BTreeMap` is used instead of a `HashMap` to guarantee deterministic ordering during iteration and report
    /// generation.
    items: BTreeMap<ItemKind, ItemMetric>,
    foreign_items: BTreeMap<ItemKind, ItemMetric>,
    impl_items: BTreeMap<ItemKind, ItemMetric>,
    trait_items: BTreeMap<ItemKind, ItemMetric>,
}

impl MetricCount {
    /// Creates an empty metric collection.
    #[instrument(level = "trace", name = "Metrics/new", skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an iterator over all collected item metrics
    ///
    /// Iteration order is deterministic and follows the ordering defined by the `ItemKind`, as the internal storage uses a
    /// `BTreeMap`
    #[instrument(level = "trace", skip_all)]
    pub fn items_iter(&self) -> impl Iterator<Item = (&ItemKind, &ItemMetric)> {
        self.items.iter()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn foreign_items_iter(&self) -> impl Iterator<Item = (&ItemKind, &ItemMetric)> {
        self.foreign_items.iter()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn impl_items_iter(&self) -> impl Iterator<Item = (&ItemKind, &ItemMetric)> {
        self.impl_items.iter()
    }

    #[instrument(level = "trace", skip_all)]
    pub fn trait_items_iter(&self) -> impl Iterator<Item = (&ItemKind, &ItemMetric)> {
        self.trait_items.iter()
    }
}

/// Getters and Setters
///
/// Used to maintain a stable interface between crates.
#[maturity(
    experimental,
    todo = "Need to check if it can be refactored in better form"
)]
impl MetricCount {
    #[instrument(level = "trace", skip_all)]
    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    #[instrument(level = "trace", skip_all)]
    pub fn set_project_name(&mut self, name: String) {
        self.project_name = name
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn file(&self) -> &FileMetric {
        &self.file
    }

    #[instrument(level = "trace", skip_all)]
    pub fn file_mut(&mut self) -> &mut FileMetric {
        &mut self.file
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.items.get(&kind)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn items_mut(&mut self, kind: ItemKind) -> &mut ItemMetric {
        self.items.entry(kind).or_default()
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn foreign_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.foreign_items.get(&kind)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn foreign_items_mut(&mut self, kind: ItemKind) -> &mut ItemMetric {
        self.foreign_items.entry(kind).or_default()
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn impl_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.impl_items.get(&kind)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn impl_items_mut(&mut self, kind: ItemKind) -> &mut ItemMetric {
        self.impl_items.entry(kind).or_default()
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn trait_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.trait_items.get(&kind)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn trait_items_mut(&mut self, kind: ItemKind) -> &mut ItemMetric {
        self.trait_items.entry(kind).or_default()
    }
}
