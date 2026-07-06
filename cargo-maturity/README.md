# cargo-maturity

Cargo plugin integration for the `maturity` ecosystem.
Provides the `cargo maturity` command which allows the usage of the common CLI implementations.

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
cargo install cargo-maturity
```

## Screenshot

![Maturity Report of the maturity workspace using the `cargo maturity` plugin](images/usage_of_cargo_maturity_command.png)

Maturity Report of the maturity workspace using the `cargo maturity` plugin

## Commands

### `report`

> [!NOTE]
> v0.1.2 - generates an inventory report for the current project or workspace

![usage display of cargo maturity report command](images/cargo_maturity_report.png)

## Learn More

See the workspace README for an overview of the maturity ecosystem.

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

## License

Licensed under Apache-2.0. See the workspace LICENSE file for details.
