# maturity-report

This crate contains the core implementation behind the `maturity` tooling.

Its responsibilities lies in:

- project scanning
- file discovery
- rust syntax parsing
- metric collection
- report generation
- command execution

## `maturity` ecosystem?

The `maturity` ecosystem is of 5 crates where 2 crates are interfaces and 3 crates are functionalities:

### functionality crates:
- maturity-core
- maturity-macro
- maturity-report

### interface crates:
- cargo-maturity
- maturity-cli
