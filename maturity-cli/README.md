# maturity-cli

Standalone command-line interface for the `maturity` ecosystem.
Provides the `maturity` executable without relying on cargo plugin support.

It provides the standalone `maturity` executable for environments where invoking `cargo maturity` is unnecessary or inconvenient, including:

- CI pipelines
- automation scripts
- editor integrations
- external tooling
- custom workflows

## Current

Version `0.1.2` focuses on inventory reporting.
It currently reports:

- project file counts
- Rust file counts
- Rust item counts
- `#[maturity]` - annotated item counts

The only available command is `report`, which is also the default when no command or subcommand is provided.

## Installation

```shell
cargo install maturity-cli
```

## Screenshot

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/3f8ab49b1dbffd3021142d2d25856e4328305610/maturity-cli/images/usage_of_release_maturity_command.png" alt="Maturity Report of the maturity workspace using the `target/release/maturity` plugin" style="max-width:100%;">

Maturity Report of the maturity workspace using the standalone `maturity` executable

## Commands

### `report`

> [!NOTE]
>  v0.1.2 - generates an inventory report for the current project or workspace

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/3f8ab49b1dbffd3021142d2d25856e4328305610/maturity-cli/images/release_maturity_report.png" alt="usage display of release maturity report command" style="max-width:100%;">

## Learn More

See the workspace README for an overview of the maturity ecosystem.

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

## License

Licensed under Apache-2.0. See the workspace LICENSE file for details.
