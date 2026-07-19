<!-- markdownlint-disable MD024 -->
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[0.2.0\] - 2026-07-20

### Added

- Inventory command for displaying project inventories
- Annotation command for displaying collected maturity annotations
- Annotation collection and reporting
- Support for unknown attributes
- Add Inventory and Annotations commands
- Foreign, implementation and trait item inventories

### Changed

- Refactored metric storage around grouped metrics
- `ItemKind` display names are now singular
- Simplified metric iteration APIs

### Documentation

- Fixed markdownlint in `CONTRIBUTING.md` and `README.md`
- Updated `ROADMAP.md`
- Updated workspace and crate `README.md`s
- Updated doc-tests

## \[0.1.3\] - 2026-07-06

### Documentation

- Fixed README image urls for `cargo-maturity` and `maturity-cli` on crates.io

## \[0.1.2\] - 2026-07-06

### Added

- Introduced `FileMetric`
  - `increment` increments `total` by one and, when the provided boolean is `true`, increments `rust` as well
  - `total` - returns the field value
  - `rust` - returns the field value
  
- Introduced `ItemMetric`
  - `increment` - increments `total` by one and, when the provided boolean is `true`, increments `maturity` as well
  - `total` - returns the field value
  - `maturity` - returns the field value
  
- Introduced `ItemKind`
  - `fmt::Display` implementation

- `MetricCount`
  - `items_iter` - returns an iterator over all item metrics

### Changed

- Refactored `MetricCount` to use domain-specific metric types
- Replaced `u64` file counters with `FileMetric`
- Replaced individual item fields with `items: BTreeMap<ItemKind, ItemMetric>`
- Simplified metric access through `item` and `item_mut`
- Nuked getter/setter boilerplate to the smaller, minimal version throughout the metric system.
- Updated metric collection to use the new domain-oriented metric system
- Updated file scanning to use the new `FileMetric` API
- Upgraded terminal Output by utilising `owo-colors` and `comfy-table`

### Documentation

- Updated workspace and crate READMEs
- Added repository links, MSRV, and license information to the per-crate READMEs

### Removed

- Removed individual item count fields from `MetricCount`
- `fmt::Display` from `MetricCount`

## \[0.1.1\] - 2026-07-04

### Added

- Repository metadata
- Homepage metadata
- Added per-crate READMEs

## \[0.1.0\] - 2026-06-30

Inventory system

### Added

- This `CHANGELOG` file.
- Cargo plugin (`cargo maturity`)
- Stand-alone CLI executable
- Metrics reporting system
- Detection and counting of `#[maturity]` annotation on supported items.
- Inventory counting for:
  - Total files
  - Rust files
  - Structs
  - Enums
  - Traits
  - Functions
- Initial maturity annotation detection
- Recursive project scanning using the `ignore` crate
- Rust syntax tree parsing using `syn`
- Separation between file discovery and metric collection systems.
- Logging infrastructure for development and profiling
- Workspace split into:
  - cargo-maturity
  - maturity-cli
  - maturity-core
  - maturity-macro
  - maturity-report

## \[0.0.1\]

Changelog was not maintained for this release.
Some of the changes from here are integrated into 0.1.0 release.

It consisted mainly of initialisation of the workspace.
