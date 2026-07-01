# cargo-maturity

Standalone command-line interface for the `maturity` ecosystem.
Provides the `maturity` executable without relying on cargo plugin support.

It exists to support environments where cargo plugin may not be appropriate, such as:
- CI pipelines
- automation scripts
- editor integrations
- external tooling
- custom workflows
