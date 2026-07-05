//! Metric collection
//!
//! The collector reads rust source files, builds syntax trees using `syn`, extracts the required metrics, and immediately
//! discards the syntax trees afterwards.
//!
//! Only the information needed for reporting is kept. Syntax trees themselves are intentionally not stored to avoid
//! unnecessary memory usage and ownership complexity.
//!

use std::path::PathBuf;

use maturity_core::metrics::{ItemKind, MetricCount};
use maturity_macro::maturity;
use syn::File;
use tracing::instrument;

/// Collects metrics from Rust source files.
#[maturity]
pub struct Collector;

impl Collector {
    #[maturity]
    #[instrument(level = "trace", name = "Collector/name" skip_all)]
    pub fn new() -> Self {
        Self
    }

    /// Reads, parses, and collects metrics from rust source files.
    ///
    /// Each file follows the same pipeline:
    ///
    /// Path -> source code -> syntax tree -> metrics
    ///
    /// Once done the syntax tree is dropped
    #[maturity]
    #[instrument(level = "trace", skip_all)]
    // TODO: ErrorHandling: Implementation of CollectorResult<()>
    pub fn collect(&mut self, paths: Vec<PathBuf>, metrics: &mut MetricCount) {
        for path in paths {
            // TODO: ErrorHandling: change the match error to ?;
            let source = match std::fs::read_to_string(path) {
                Ok(string) => string,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            // TODO: ErrorHandling: change the match error to ?;
            let file = match syn::parse_file(&source) {
                Ok(file) => file,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            self.collect_metrics(file, metrics);
        }
    }

    /// Updates project metrics from a parsed rust syntax tree.
    #[maturity(status = "developing", todo = "Count internal items")]
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics(&mut self, file: File, metrics: &mut MetricCount) {
        for item in file.items {
            match item {
                syn::Item::Const(item_const) => {
                    metrics
                        .items_mut(ItemKind::Const)
                        .increment(self.has_maturity(&item_const.attrs));
                }
                syn::Item::Enum(item_enum) => {
                    metrics
                        .items_mut(ItemKind::Enum)
                        .increment(self.has_maturity(&item_enum.attrs));
                }
                syn::Item::ExternCrate(item_extern_crate) => {
                    metrics
                        .items_mut(ItemKind::ExternCrate)
                        .increment(self.has_maturity(&item_extern_crate.attrs));
                }
                // This is module-wise function count.
                syn::Item::Fn(item_fn) => {
                    metrics
                        .items_mut(ItemKind::Function)
                        .increment(self.has_maturity(&item_fn.attrs));
                }
                syn::Item::ForeignMod(item_foreign_mod) => {
                    metrics
                        .items_mut(ItemKind::ForeignMod)
                        .increment(self.has_maturity(&item_foreign_mod.attrs));
                }
                syn::Item::Impl(item_impl) => {
                    // TODO: Internal functions counting
                    metrics
                        .items_mut(ItemKind::Implementation)
                        .increment(self.has_maturity(&item_impl.attrs));
                }
                syn::Item::Macro(item_macro) => {
                    metrics
                        .items_mut(ItemKind::Macro)
                        .increment(self.has_maturity(&item_macro.attrs));
                }
                syn::Item::Mod(item_mod) => {
                    metrics
                        .items_mut(ItemKind::Module)
                        .increment(self.has_maturity(&item_mod.attrs));
                }
                syn::Item::Static(item_static) => {
                    metrics
                        .items_mut(ItemKind::Static)
                        .increment(self.has_maturity(&item_static.attrs));
                }
                syn::Item::Struct(item_struct) => {
                    metrics
                        .items_mut(ItemKind::Struct)
                        .increment(self.has_maturity(&item_struct.attrs));
                }
                syn::Item::Trait(item_trait) => {
                    metrics
                        .items_mut(ItemKind::Trait)
                        .increment(self.has_maturity(&item_trait.attrs));
                }
                syn::Item::TraitAlias(item_trait_alias) => {
                    metrics
                        .items_mut(ItemKind::TraitAlias)
                        .increment(self.has_maturity(&item_trait_alias.attrs));
                }
                syn::Item::Type(item_type) => {
                    metrics
                        .items_mut(ItemKind::Type)
                        .increment(self.has_maturity(&item_type.attrs));
                }
                syn::Item::Union(item_union) => {
                    metrics
                        .items_mut(ItemKind::Union)
                        .increment(self.has_maturity(&item_union.attrs));
                }
                syn::Item::Use(item_use) => {
                    metrics
                        .items_mut(ItemKind::Use)
                        .increment(self.has_maturity(&item_use.attrs));
                }
                // TODO: Need to poke it more
                // syn::Item::Verbatim(token_stream) => {
                //     metrics
                //         .items_mut(ItemKind::Verbatim)
                //         .increment(self.has_maturity());
                // }
                _ => {}
            }
        }
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}

// helper functions
impl Collector {
    #[maturity]
    #[instrument(level = "trace", skip_all)]
    fn has_maturity(&self, attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| attr.path().is_ident("maturity"))
    }
}
