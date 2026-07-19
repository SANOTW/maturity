<!-- markdownlint-disable MD033 -->
# maturity-cli

Stand-alone command-line interface for the `maturity` ecosystem.
Provides the `maturity` executable without relying on cargo plugin support.

It provides the stand-alone `maturity` executable for environments where invoking `cargo maturity` is unnecessary or inconvenient, including:

- CI pipelines
- automation scripts
- editor integrations
- external tooling
- custom workflows

## Current

Version `0.2.0` annotation infrastructure.
It currently provides:

- project file counts
- Rust file counts
- Rust item counts
- `#[maturity]` - annotated item counts
- Annotation attribute reporting

## Installation

```shell
cargo install maturity-cli
```

## Screenshot

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/maturity-cli/images/release_maturity_inventory.png" alt="Maturity Report of the maturity workspace using the `target/release/maturity` plugin" style="max-width:100%;">

Maturity Report of the maturity workspace using the stand-alone `maturity` executable

## Commands

### `report`

> [!NOTE]
> Currently displays the same project inventory as the `inventory` command.

---

### `inventory`

Displays the discovered Rust inventory for the current project.

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/maturity-cli/images/release_maturity_inventory.png" alt="usage display of release maturity report command" style="max-width:100%;">

---

### `annotations`

Displays every discovered `#[maturity]` annotation together with its parsed attributes

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/maturity-cli/images/release_maturity_annotations.png" alt="usage display of release maturity report command" style="max-width:100%;">

---

## Learn More

See the workspace README for an overview of the maturity ecosystem.

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

## License

Licensed under Apache-2.0. See the workspace LICENSE file for details.
