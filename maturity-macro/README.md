# maturity-macro

This crate provides attribute macros used to annotate Rust items with maturity information.

## Installation

```shell
cargo add maturity-macro
```

## Usage Example

The `#[maturity]` attribute can be attached to Rust items with additional metadata.

> [!NOTE]
> v0.1.2 -> `status`, `todo`, and other attributes are accepted but haven't been applied yet. The current release focuses on inventory collection.

> [!NOTE]
> v0.1.2 -> nested functions within `impl` blocks and similar nested constructs are not included yet in the inventory reporting. Support is planned for a future release.

```rust
#[maturity(status = "developing", todo = "Count internal items")]
fn foo() {
    // coding...
    // coding...
}
```

## Learn More

See the workspace README for an overview of the maturity ecosystem.

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

## License

Licensed under Apache-2.0. See the workspace LICENSE file for details.
