# maturity-core

This crate contains common data structures used across the workspace.

Keeping these definitions in a dedicated crate ensures consistency between the crates within the workspace.

## `maturity` ecosystem?

The `maturity` ecosystem is of 5 crates where 2 crates are interfaces and 3 crates are functionalities:

### functionality crates:
- maturity-core
- maturity-macro
- maturity-report

### interface crates:
- cargo-maturity
- maturity-cli
