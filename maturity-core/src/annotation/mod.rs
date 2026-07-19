use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use maturity_macro::maturity;
use tracing::instrument;

use crate::{annotation::maturity_attributes::MaturityAttributes, metrics::ItemKind};

pub mod maturity_attributes;

// --------------------------

#[maturity(experimental, undocumented)]
#[derive(Debug, Default)]
pub struct ItemPath {
    pub crate_name: String,
    pub modules: Vec<String>,
    pub item: Option<String>,
}

// --------------------------

#[maturity(
    pending,
    note = "This is to be used in `AnnotatedItemInfo`'s `file` field replacement. Unless something changes."
)]
pub struct SourceLocation {
    // file: PathBuf,
    // line: usize,
}

// --------------------------

#[maturity(experimental, todo = "introduce, path and file", undocumented)]
#[derive(Default, Clone, Debug)]
pub struct AnnotatedItemInfo {
    pub path: PathBuf,
    pub name: Option<String>,
    pub kind: ItemKind,
    // pub path: ItemPath,
    // pub file: PathBuf,
    pub attributes: MaturityAttributes,
}

impl AnnotatedItemInfo {
    #[instrument(level = "trace", skip_all)]
    pub fn new(
        path: &Path,
        name: Option<String>,
        kind: ItemKind,
        attributes: &MaturityAttributes,
    ) -> Self {
        Self {
            path: path.to_path_buf(),
            name,
            kind,
            attributes: attributes.clone(),
        }
    }
}

impl fmt::Display for AnnotatedItemInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;

        if let Some(name) = &self.name {
            write!(f, " {}", name)?;
        }

        write!(f, " {}", self.attributes)
    }
}

// --------------------------

#[maturity(experimental)]
#[derive(Default)]
pub struct AnnotatedItemInventory {
    items: Vec<AnnotatedItemInfo>,
}

impl AnnotatedItemInventory {
    #[instrument(level = "trace", name = "AnnotatedItemInventory/new", skip_all)]
    pub fn new() -> Self {
        Self::default()
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn store_annotation(&mut self, info: AnnotatedItemInfo) {
        self.items.push(info);
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn items_iter(&self) -> impl Iterator<Item = &AnnotatedItemInfo> {
        self.items.iter()
    }

    #[maturity(experimental, unused)]
    #[instrument(level = "trace", skip_all)]
    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    pub fn annotation_summary_header(&self) -> HashMap<String, i32> {
        let mut output = HashMap::new();

        for annotated_item_info in self.items_iter() {
            if annotated_item_info.attributes.state.is_some() {
                self.increment("state", &mut output);
            }
            if annotated_item_info.attributes.refactor.is_some() {
                self.increment("refactor", &mut output);
            }

            if annotated_item_info.attributes.todo.is_some() {
                self.increment("todo", &mut output);
            }
            for _ in &annotated_item_info.attributes.unknown {
                self.increment("unknown", &mut output);
            }
        }

        output
    }
}

impl AnnotatedItemInventory {
    #[maturity(experimental)]
    #[instrument(level = "trace", skip_all)]
    fn increment(&self, attribute: impl Into<String>, output: &mut HashMap<String, i32>) {
        *output.entry(attribute.into()).or_insert(0) += 1
    }
}
