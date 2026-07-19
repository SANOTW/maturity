# maturity-macro

This crate provides the `#[maturity]` attribute macros used to annotate Rust items with maturity metadata.

The collected metadata is consumed by the `maturity` tooling to generate inventories and annotation reports.

---

## Behaviour

The macro performs no code generation or behavioural modification.

Its purpose is to attach maturity metadata that can later be discovered by the `maturity` tooling.

---

## Installation

```shell
cargo add maturity-macro
```

---

## Usage Example

The `#[maturity]` attribute can be attached to Rust items with additional metadata.

```rust
#[maturity(developing, todo = "Count internal items")]
fn foo() {
    // coding...
    // coding...
}
```

### Supported Development States

- `planned`
- `experimental`
- `developing`
- `stable`
- `deprecated`

### Supported Attributes

- `todo`
- `refactor`

> [!NOTE]
> Unknown attributes are preserved and reported, allowing experimentation with custom metadata without breaking parsing.

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
