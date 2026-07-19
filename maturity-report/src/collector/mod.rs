//! Metric collection
//!
//! The collector reads rust source files, builds syntax trees using `syn`, extracts the required metrics, and immediately
//! discards the syntax trees afterwards.
//!
//! Only the information needed for reporting is kept. Syntax trees themselves are intentionally not stored to avoid
//! unnecessary memory usage and ownership complexity.
//!

use std::path::PathBuf;

use maturity_core::{
    annotation::{
        AnnotatedItemInfo, AnnotatedItemInventory, maturity_attributes::MaturityAttributes,
    },
    metrics::{ItemKind, MetricCount, MetricGroup},
};
use maturity_macro::maturity;
use syn::{Attribute, File, ForeignItem, ImplItem, Item, TraitItem};
use tracing::instrument;

use crate::collector::result::MaturityAnnotation;

mod result;

/// Collects metrics from Rust source files.
#[maturity(
    experimental,
    todo = "Refactor, collection of file path, annotations, and other metrics and require values to be stored in much easier format"
)]
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
    #[maturity(
        experimental,
        refactor = "pub fn `collect_with_annotations()` from the same version is a variant of the current un-refactored function"
    )]
    #[instrument(level = "trace", skip_all)]
    // TODO: ErrorHandling: Implementation of CollectorResult<()>
    pub fn collect(&mut self, paths: Vec<PathBuf>, metrics: &mut MetricCount) {
        for path in paths {
            // TODO: ErrorHandling: change the match error to ?;
            let source = match std::fs::read_to_string(&path) {
                Ok(string) => string,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            // TODO: ErrorHandling: change the match error to ?;
            let file = match syn::parse_file(&source) {
                Ok(file) => file,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            self.collect_metrics(&path, file, metrics);
        }
    }

    #[maturity(
        experimental,
        refactor = "pub fn `collect()` from the same version is a variant of the current un-refactored function"
    )]
    #[instrument(level = "trace", skip_all)]
    pub fn collect_with_annotations(
        &mut self,
        paths: Vec<PathBuf>,
        metrics: &mut MetricCount,
        annotations_inventory: &mut AnnotatedItemInventory,
    ) {
        for path in paths {
            let source = match std::fs::read_to_string(&path) {
                Ok(string) => string,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            let file = match syn::parse_file(&source) {
                Ok(file) => file,
                Err(error_value) => panic!("{:#?}", error_value),
            };

            self.collect_metrics_with_annotations(&path, file, metrics, annotations_inventory);
        }
    }

    /// Updates project metrics from a parsed rust syntax tree.
    #[maturity(status = "developing", todo = "Count internal items")]
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics(&mut self, path: &PathBuf, file: File, metrics: &mut MetricCount) {
        self.count(path, file.items, metrics, &mut None);
    }

    #[maturity(status = "developing", todo = "Count internal items")]
    #[instrument(level = "trace", skip_all)]
    fn collect_metrics_with_annotations(
        &mut self,
        path: &PathBuf,
        file: File,
        metrics: &mut MetricCount,
        annotations: &mut AnnotatedItemInventory,
    ) {
        self.count(path, file.items, metrics, &mut Some(annotations));
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}

// helper functions
impl Collector {
    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    fn maturity_annotation(&self, attrs: &[syn::Attribute]) -> MaturityAnnotation {
        let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("maturity")) else {
            return MaturityAnnotation::Missing;
        };
        match attr.parse_args::<MaturityAttributes>() {
            Ok(meta) => MaturityAnnotation::Parsed(meta),
            // TODO: Reinstate the Invalid field and have it properly used
            Err(_) => MaturityAnnotation::Missing, /*MaturityAnnotation::Invalid(error),*/
        }
    }

    #[maturity(
        experimental,
        refactor = "instead of having to pass down the ItemKind, the MetricCount owns what to add."
    )]
    #[instrument(level = "trace", skip_all)]
    fn count(
        &mut self,
        path: &PathBuf,
        items: Vec<Item>,
        metrics: &mut MetricCount,
        annotated_inventory: &mut Option<&mut AnnotatedItemInventory>,
    ) {
        for item in items {
            match item {
                syn::Item::Const(item_const) => {
                    self.process_annotation(
                        path,
                        &item_const.attrs,
                        ItemKind::Const,
                        Some(item_const.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Enum(item_enum) => {
                    self.process_annotation(
                        path,
                        &item_enum.attrs,
                        ItemKind::Enum,
                        Some(item_enum.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::ExternCrate(item_extern_crate) => {
                    self.process_annotation(
                        path,
                        &item_extern_crate.attrs,
                        ItemKind::ExternCrate,
                        Some(item_extern_crate.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                // This is module-wise function count.
                syn::Item::Fn(item_fn) => {
                    self.process_annotation(
                        path,
                        &item_fn.attrs,
                        ItemKind::Function,
                        Some(item_fn.sig.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::ForeignMod(item_foreign_mod) => {
                    self.process_annotation(
                        path,
                        &item_foreign_mod.attrs,
                        ItemKind::ForeignMod,
                        None,
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::ForeignMod(item_foreign_mod.items),
                    );
                }
                syn::Item::Impl(item_impl) => {
                    self.process_annotation(
                        path,
                        &item_impl.attrs,
                        ItemKind::Implementation,
                        None,
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::Implementation(item_impl.items),
                    );
                }
                syn::Item::Macro(item_macro) => {
                    self.process_annotation(
                        path,
                        &item_macro.attrs,
                        ItemKind::Macro,
                        None, // item_macro.ident.unwrap_or(None),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Mod(item_mod) => {
                    // TODO: The NestedTypes will need to be expanded to accommodate Mod as well once figured out how to
                    self.process_annotation(
                        path,
                        &item_mod.attrs,
                        ItemKind::Module,
                        Some(item_mod.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );

                    // TODO: "need to check this block if this block can be refactored much better"
                    let Some((_, items)) = item_mod.content else {
                        continue;
                    };
                    self.count(path, items, metrics, annotated_inventory);
                }
                syn::Item::Static(item_static) => {
                    self.process_annotation(
                        path,
                        &item_static.attrs,
                        ItemKind::Static,
                        Some(item_static.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Struct(item_struct) => {
                    self.process_annotation(
                        path,
                        &item_struct.attrs,
                        ItemKind::Struct,
                        Some(item_struct.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Trait(item_trait) => {
                    self.process_annotation(
                        path,
                        &item_trait.attrs,
                        ItemKind::Trait,
                        Some(item_trait.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::Trait(item_trait.items),
                    );
                }
                syn::Item::TraitAlias(item_trait_alias) => {
                    self.process_annotation(
                        path,
                        &item_trait_alias.attrs,
                        ItemKind::TraitAlias,
                        Some(item_trait_alias.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Type(item_type) => {
                    self.process_annotation(
                        path,
                        &item_type.attrs,
                        ItemKind::Type,
                        Some(item_type.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Union(item_union) => {
                    self.process_annotation(
                        path,
                        &item_union.attrs,
                        ItemKind::Union,
                        Some(item_union.ident.to_string()),
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
                }
                syn::Item::Use(item_use) => {
                    self.process_annotation(
                        path,
                        &item_use.attrs,
                        ItemKind::Use,
                        None,
                        metrics,
                        annotated_inventory,
                        MetricGroup::Item,
                        NestedTypes::None,
                    );
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

    #[maturity(
        experimental,
        refactor = "reduce number of arguments or re look entire data flow and refactor"
    )]
    #[instrument(level = "trace", skip_all)]
    fn process_annotation(
        &self,
        path: &PathBuf,
        attrs: &[Attribute],
        kind: ItemKind,
        item_name: Option<String>,
        metrics: &mut MetricCount,
        inventory: &mut Option<&mut AnnotatedItemInventory>,
        group: MetricGroup,
        nested_type: NestedTypes,
    ) {
        let annotation = self.maturity_annotation(attrs);

        metrics
            .metric_mut(group, kind)
            .increment(annotation.exists());

        if let Some(attributes) = annotation.attributes()
            && let Some(inventory) = inventory.as_deref_mut()
        {
            inventory.store_annotation(AnnotatedItemInfo::new(path, item_name, kind, attributes));
        }

        match nested_type {
            NestedTypes::ForeignMod(foreign_items) => {
                for item in foreign_items {
                    match item {
                        ForeignItem::Fn(foreign_item_fn) => {
                            self.process_annotation(
                                path,
                                &foreign_item_fn.attrs,
                                ItemKind::Function,
                                Some(foreign_item_fn.sig.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::ForeignItem,
                                NestedTypes::None,
                            );
                        }
                        ForeignItem::Static(foreign_item_static) => {
                            self.process_annotation(
                                path,
                                &foreign_item_static.attrs,
                                ItemKind::Static,
                                Some(foreign_item_static.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::ForeignItem,
                                NestedTypes::None,
                            );
                        }
                        ForeignItem::Type(foreign_item_type) => {
                            self.process_annotation(
                                path,
                                &foreign_item_type.attrs,
                                ItemKind::Type,
                                Some(foreign_item_type.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::ForeignItem,
                                NestedTypes::None,
                            );
                        }
                        ForeignItem::Macro(foreign_item_macro) => {
                            self.process_annotation(
                                path,
                                &foreign_item_macro.attrs,
                                ItemKind::Macro,
                                None,
                                metrics,
                                inventory,
                                MetricGroup::ForeignItem,
                                NestedTypes::None,
                            );
                        }
                        // TODO: Need to poke it more
                        // ForeignItem::Verbatim(token_stream) => {
                        //     self.process_annotation( path,
                        //         &token_stream.attrs,
                        //         ItemKind::Verbatim,
                        //         metrics,
                        //         inventory,
                        //         NestedTypes::None,
                        //     );
                        // }
                        _ => {}
                    }
                }
            }
            NestedTypes::Implementation(impl_items) => {
                for item in impl_items {
                    match item {
                        ImplItem::Const(impl_item_const) => {
                            self.process_annotation(
                                path,
                                &impl_item_const.attrs,
                                ItemKind::Const,
                                Some(impl_item_const.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Implementation,
                                NestedTypes::None,
                            );
                        }
                        ImplItem::Fn(impl_item_fn) => {
                            self.process_annotation(
                                path,
                                &impl_item_fn.attrs,
                                ItemKind::Function,
                                Some(impl_item_fn.sig.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Implementation,
                                NestedTypes::None,
                            );
                        }
                        ImplItem::Type(impl_item_type) => {
                            self.process_annotation(
                                path,
                                &impl_item_type.attrs,
                                ItemKind::Type,
                                Some(impl_item_type.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Implementation,
                                NestedTypes::None,
                            );
                        }
                        ImplItem::Macro(impl_item_macro) => {
                            self.process_annotation(
                                path,
                                &impl_item_macro.attrs,
                                ItemKind::Macro,
                                None,
                                metrics,
                                inventory,
                                MetricGroup::Implementation,
                                NestedTypes::None,
                            );
                        }
                        // TODO: Need to poke it more
                        // ImplItem::Verbatim(token_stream) => {
                        //     self.process_annotation( path,
                        //         &foreign_item_fn.attrs,
                        //         ItemKind::Function,
                        //         metrics,
                        //         inventory,
                        //         NestedTypes::None,
                        //     );
                        // },
                        _ => {}
                    }
                }
            }
            NestedTypes::Trait(trait_items) => {
                for item in trait_items {
                    match item {
                        TraitItem::Const(trait_item_const) => {
                            self.process_annotation(
                                path,
                                &trait_item_const.attrs,
                                ItemKind::Const,
                                Some(trait_item_const.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Trait,
                                NestedTypes::None,
                            );
                        }
                        TraitItem::Fn(trait_item_fn) => {
                            self.process_annotation(
                                path,
                                &trait_item_fn.attrs,
                                ItemKind::Function,
                                Some(trait_item_fn.sig.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Trait,
                                NestedTypes::None,
                            );
                        }
                        TraitItem::Type(trait_item_type) => {
                            self.process_annotation(
                                path,
                                &trait_item_type.attrs,
                                ItemKind::Type,
                                Some(trait_item_type.ident.to_string()),
                                metrics,
                                inventory,
                                MetricGroup::Trait,
                                NestedTypes::None,
                            );
                        }
                        TraitItem::Macro(trait_item_macro) => {
                            self.process_annotation(
                                path,
                                &trait_item_macro.attrs,
                                ItemKind::Macro,
                                None,
                                metrics,
                                inventory,
                                MetricGroup::Trait,
                                NestedTypes::None,
                            );
                        }
                        // TODO: Need to poke it more
                        // TraitItem::Verbatim(token_stream) => {
                        //     self.process_annotation( path,
                        //         &foreign_item_fn.attrs,
                        //         ItemKind::Function,
                        //         metrics,
                        //         inventory,
                        //         NestedTypes::None,
                        //     );
                        // }
                        _ => {}
                    }
                }
            }
            NestedTypes::None => {}
        }
    }
}

#[maturity(experimental)]
pub enum NestedTypes {
    ForeignMod(Vec<ForeignItem>),
    Implementation(Vec<ImplItem>),
    Trait(Vec<TraitItem>),
    None,
}
