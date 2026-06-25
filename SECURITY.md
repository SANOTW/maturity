# Security

## Current behaviour

- Local execution only
- No network access
- No telemetry, analytics, or usage reporting
- No external services
- Local logging only

The `initialise_logging` function exists for development and debugging purposes and is disabled by default. When enabled, logs are written locally and are never transmitted externally.

## Expected behaviour

- The tool should only traverse files within the user-provided project root.
- By default, the project root is the current working directory from which the CLI or Cargo plugin is executed.
- Directory traversal respects ignore rules through the `ignore` crate, including files such as `.gitignore` and `.ignore`.
- The tool should not intentionally access files outside the requested project boundary

## Future changes

Any future network-related functionality that introduces:

- Network access
- External services
- Telemetry
- Analytics
- Remote storage

will be documented publicly, remain optional, and be disabled by default.

## Reporting issues

Please report security issues privately before public disclosure.
