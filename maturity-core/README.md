<!-- markdownlint-disable MD024-->
# maturity-core

This crate contains common data structures used across the workspace.

Keeping these definitions in a dedicated crate ensures consistency across the entire workspace.

## Installation

```shell
cargo add maturity-core
```

## Public Types

### Inventory

- `ItemKind`
- `ItemPath`

### Metrics

- `FileMetric`
- `ItemMetric`
- `MetricCount`

### Maturity Annotation

#### States

- `MaturityState`

#### Attributes

- `MaturityAttributes`
- `KnownAttribute`
- `UnknownAttribute`

#### Inventory

- `AnnotatedItemInfo`
- `AnnotatedItemInventory`

### Source Location

- `SourceLocation`

> [!NOTE]
> `SourceLocation` is currently reserved for future use

## Learn More

See the workspace README for an overview of the maturity ecosystem.

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

## License

Licensed under Apache-2.0. See the workspace LICENSE file for details.
