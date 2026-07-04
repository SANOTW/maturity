# cargo-maturity

Standalone command-line interface for the `maturity` ecosystem.
Provides the `maturity` executable without relying on cargo plugin support.

It exists to support environments where cargo plugin may not be appropriate, such as:
- CI pipelines
- automation scripts
- editor integrations
- external tooling
- custom workflows

## `maturity` ecosystem?

The `maturity` ecosystem is of 5 crates where 2 crates are interfaces and 3 crates are functionalities:

### functionality crates:
- maturity-core
- maturity-macro
- maturity-report

### interface crates:
- cargo-maturity
- maturity-cli
