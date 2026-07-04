# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased\]

- n/a

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
- Standalone CLI executable
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
