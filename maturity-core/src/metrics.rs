use core::fmt;
use std::collections::BTreeMap;

use maturity_macro::maturity;
use tracing::instrument;

// TODO: Should this file be modularised... is it possible to refactor this file further... without destroying meaning
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
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[default]
    Verbatim,
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Const => write!(f, "Const"),
            ItemKind::Enum => write!(f, "Enum"),
            ItemKind::ExternCrate => write!(f, "External Crate"),
            ItemKind::Function => write!(f, "Function"),
            ItemKind::ForeignMod => write!(f, "Foreign Module"),
            ItemKind::Implementation => write!(f, "Implementation"),
            ItemKind::Macro => write!(f, "Macro"),
            ItemKind::Module => write!(f, "Module"),
            ItemKind::Static => write!(f, "Static"),
            ItemKind::Struct => write!(f, "Struct"),
            ItemKind::Trait => write!(f, "Trait"),
            ItemKind::TraitAlias => write!(f, "Trait Aliase"),
            ItemKind::Type => write!(f, "Type"),
            ItemKind::Union => write!(f, "Union"),
            ItemKind::Use => write!(f, "Use"),
            ItemKind::Verbatim => write!(f, "Verbatim"),
        }
    }
}

// ----------------------------------------------

#[maturity(experimental)]
pub enum MetricGroup {
    Item,
    ForeignItem,
    Implementation,
    Trait,
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
/// use maturity_core::metrics::MetricGroup;
///
/// let mut metric = MetricCount::new();
///
/// metric.file_mut().increment(true);
/// metric.metric_mut(MetricGroup::Item, ItemKind::Function).increment(true);
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

    // ---

    // mutable
    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn metric_mut(&mut self, group: MetricGroup, kind: ItemKind) -> &mut ItemMetric {
        match group {
            MetricGroup::Item => self.items.entry(kind).or_default(),
            MetricGroup::ForeignItem => self.foreign_items.entry(kind).or_default(),
            MetricGroup::Implementation => self.impl_items.entry(kind).or_default(),
            MetricGroup::Trait => self.trait_items.entry(kind).or_default(),
        }
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn items_maturity_total(&self) -> u64 {
        self.maturity_total(self.collect_metrics(self.items_iter().collect()))
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn foreign_items_maturity_total(&self) -> u64 {
        self.maturity_total(self.collect_metrics(self.foreign_items_iter().collect()))
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn impl_items_maturity_total(&self) -> u64 {
        self.maturity_total(self.collect_metrics(self.impl_items_iter().collect()))
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn trait_items_maturity_total(&self) -> u64 {
        self.maturity_total(self.collect_metrics(self.trait_items_iter().collect()))
    }

    #[maturity(experimental, unused)]
    #[instrument(level = "trace", skip_all)]
    pub fn grand_maturity_total(&self) -> u64 {
        self.items_maturity_total()
            + self.foreign_items_maturity_total()
            + self.impl_items_maturity_total()
            + self.trait_items_maturity_total()
    }
}

#[maturity(todo = "Check if this can be removed.")]
impl MetricCount {
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics(&self, iter: Vec<(&ItemKind, &ItemMetric)>) -> Vec<(ItemKind, ItemMetric)> {
        iter.into_iter()
            .map(|(kind, metric)| (*kind, metric.clone()))
            .collect()
    }

    #[instrument(level = "trace", skip_all)]
    fn maturity_total(&self, iter: Vec<(ItemKind, ItemMetric)>) -> u64 {
        iter.iter().map(|(_, metric)| metric.maturity).sum()
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

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn foreign_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.foreign_items.get(&kind)
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn impl_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.impl_items.get(&kind)
    }

    // ---

    #[instrument(level = "trace", skip_all)]
    pub fn trait_items(&self, kind: ItemKind) -> Option<&ItemMetric> {
        self.trait_items.get(&kind)
    }
}
