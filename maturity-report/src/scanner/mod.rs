//! Project scanning
//!
//! The scanner is responsible for discovering rust source files while ensuring that the ignore rules, files that are listed
//! in files such as '.gitignore' and '.ignore' are ignored / skipped.
//!
//! Its responsibility ends at file discovery and metric bookkeeping. The only metric count that are done at the time is the
//! total files it read as well as the rust files count. Parsing and syntax-tree processing belong to the
//! collector layer.
//!

use std::path::PathBuf;

use ignore::{DirEntry, Walk};
use maturity_core::metrics::MetricCount;
use maturity_macro::maturity;
use tracing::instrument;

#[maturity]
pub struct Scanner {
    // Where the scanning of the directories will start
    starting_directory: PathBuf,
}

impl Scanner {
    /// Creates a scanner rooted at the current working directory (`.`)
    ///
    /// The starting directory can later be replaced with a user-provided path once custom scanning locations are supported.
    #[maturity]
    #[instrument(level = "trace", name = "Scanner/new", skip_all)]
    pub fn new() -> Self {
        Self {
            starting_directory: PathBuf::from("."),
        }
    }

    #[maturity]
    #[instrument(level = "trace", skip_all)]
    /// Scans the configured directory and returns all discovered rust files.
    ///
    /// Only `.rs` files are returned.
    ///
    /// Files ignored by `.gitignore` and `.ignore` are automatically skipped.
    pub fn scan(&mut self, metrics: &mut MetricCount) -> Vec<PathBuf> {
        // Storing package name
        // TODO: This has a fail format. will need to later read cargo.toml to know the package name also the workspace.
        metrics.set_project_name(
            std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );
        // Storing all the found / required file paths into this vec
        let mut rust_file_paths = Vec::new();

        for result in Walk::new(&self.starting_directory) {
            match result {
                Ok(entry) => {
                    metrics.increment_total_files_count();
                    // ensuring that only rust files are read and none else.
                    if !self.is_rust_file(&entry) {
                        continue;
                    }
                    metrics.increment_rust_files_count();

                    rust_file_paths.push(entry.path().to_path_buf());
                }
                Err(err) => println!("ERROR: {}", err),
            }
        }

        rust_file_paths
    }
}

// Helper functions
impl Scanner {
    /// Returning true if the entry ends with the `rs` extension and if the entry is a file
    #[maturity]
    #[instrument(level = "trace", skip_all)]
    fn is_rust_file(&self, entry: &DirEntry) -> bool {
        matches!(
            entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str()),
            Some("rs")
        ) && entry
            .file_type()
            .map(|file_type| file_type.is_file())
            .unwrap_or(false)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
