# maturity-report

This crate contains the core implementation behind the `maturity` tooling.

Its responsibilities include:

- project scanning
- file discovery
- rust syntax parsing
- metric collection
- report generation
- command execution

## Installation

```shell
cargo add maturity-report
```

## Structure

```text
maturity-report
└── src
    ├── cli
    │   └── commands
    ├── collector
    ├── reporter
    ├── scanner
    └── utilities
        └── logger
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
