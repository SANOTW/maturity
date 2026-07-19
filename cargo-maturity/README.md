<!-- markdownlint-disable MD033 -->
# cargo-maturity

Cargo plugin integration for the `maturity` ecosystem.
Provides the `cargo maturity` command which allows the usage of the common CLI implementations.

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
cargo install cargo-maturity
```

## Screenshot

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/cargo-maturity/images/cargo_maturity_inventory.png" alt="Maturity Report of the maturity workspace using the `cargo maturity` plugin" style="max-width:100%;">

Maturity Report of the maturity workspace using the  `cargo maturity` plugin

## Commands

### `report`

> [!NOTE]
> Currently displays the same terminal as the `inventory` command.

---

### `inventory`

Displays the discovered Rust inventory for the current project.

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/cargo-maturity/images/cargo_maturity_inventory.png" alt="usage display of cargo maturity inventory command" style="max-width:100%;">

---

### `annotations`

Displays every discovered `#[maturity]` annotation together with its parsed attributes

<img src="https://codeberg.org/SANOTW/maturity/raw/commit/1014eefbcaff92b1955b9a2067c27ba161b724ce/cargo-maturity/images/cargo_maturity_annotations.png" alt="usage display of cargo maturity annotations command" style="max-width:100%;">

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
