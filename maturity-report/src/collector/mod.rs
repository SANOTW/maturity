//! Metric collection
//!
//! The collector reads rust source files, builds syntax trees using `syn`, extracts the required metrics, and immediately
//! discards the syntax trees afterwards.
//!
//! Only the information needed for reporting is kept. Syntax trees themselves are intentionally not stored to avoid
//! unnecessary memory usage and ownership complexity.
//!

use std::path::PathBuf;

use maturity_core::metrics::MetricCount;
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
    #[maturity]
    fn collect_metrics(&mut self, file: File, metrics: &mut MetricCount) {
        for item in file.items {
            match item {
                // syn::Item::Const(item_const) => todo!(),
                syn::Item::Enum(item_enum) => {
                    {
                        metrics.increment_enums_count();

                        // TODO: Extract this into a helper function.
                        for attribute in item_enum.attrs {
                            for segment in &attribute.meta.path().segments {
                                if "maturity".to_string() == segment.ident.to_string() {
                                    metrics.increment_maturity_enums_count();
                                }
                            }
                        }
                    };
                }
                // syn::Item::ExternCrate(item_extern_crate) => todo!(),
                // This is module-wise function count.
                syn::Item::Fn(item_fn) => {
                    metrics.increment_functions_count();
                    for attribute in item_fn.attrs {
                        for segment in &attribute.meta.path().segments {
                            if "maturity".to_string() == segment.ident.to_string() {
                                metrics.increment_maturity_functions_count();
                            }
                        }
                    }
                }
                // syn::Item::ForeignMod(item_foreign_mod) => todo!(),
                // syn::Item::Impl(item_impl) => todo!(),
                // syn::Item::Macro(item_macro) => todo!(),
                // syn::Item::Mod(item_mod) => todo!(),
                // syn::Item::Static(item_static) => todo!(),
                syn::Item::Struct(item_struct) => {
                    metrics.increment_structs_count();

                    for attribute in item_struct.attrs {
                        for segment in &attribute.meta.path().segments {
                            if "maturity".to_string() == segment.ident.to_string() {
                                metrics.increment_maturity_structs_count();
                            }
                        }
                    }
                }
                syn::Item::Trait(item_trait) => {
                    metrics.increment_traits_count();

                    for attribute in item_trait.attrs {
                        for segment in &attribute.meta.path().segments {
                            if "maturity".to_string() == segment.ident.to_string() {
                                metrics.increment_maturity_traits_count();
                            }
                        }
                    }
                }
                // syn::Item::TraitAlias(item_trait_alias) => todo!(),
                // syn::Item::Type(item_type) => todo!(),
                // syn::Item::Union(item_union) => todo!(),
                // syn::Item::Use(item_use) => todo!(),
                // syn::Item::Verbatim(token_stream) => todo!(),
                _ => {}
            }
        }
    }
}
