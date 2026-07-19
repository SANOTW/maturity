<!-- markdownlint-disable MD024 -->
# Roadmap

The roadmap is subject to change as the project evolves.
Features may move, merge, split, or be removed entirely depending on experimentation, implementation complexity, and real-world usage.

---

## v0.2.0 - Annotation Infrastructure

Extends the inventory system with maturity annotation parsing, collection and reporting.

### Features

### Annotation Parsing

- Maturity attribute parser
- State parsing
- Unknown attribute support
- Extensible attribute model

#### State

- Planned
- Experimental
- Developing
- Stable
- Deprecated

#### Supported Attributes

- State
- Todo
- Refactor

### Annotation Collection

- Annotation inventory
- Per-item annotation storage
- Attribute collection
- Inventory integration

### Reporting

- Annotation report
- Annotation summaries
- Detailed annotation listing
- Inventory report improvements

### Metrics

- Grouped metric storage
- Annotation statistics
- Per-item maturity counts

---

## v0.1.0 - Inventory System

Project inventory and structural reporting.

### Features

#### Scanner

- File discovery
- Rust source discovery
- Recursive traversal

#### Collection

- Rust item counting
- Maturity annotation counting

#### Metrics

- Total files
- Rust files
- Structs, enums, traits, functions

#### Reporting

- Terminal output
- Basic project summaries

---

## Future Features

The following features are planned but their release order is intentionally left open.

### Annotation Systems

#### Development State

- Unstable
- Preview

#### Verification

- Tested
- Verified
- Validated
- Benchmarked

#### Documentation

- Documented
- Doctested

#### Quality

- Audited
- Optimised
- BackwardsCompatible

#### Maturity Scoring

Potential goals:

- Configurable scoring systems
- Custom scoring rules
- Templates
- Team-specific maturity definitions

#### Export Formats

Additional report formats

- JSON
- Markdown
- HTML
- YAML

#### Workspace Support

Multi-crate analysis

- Package-level maturity
- Module-level maturity
- Workspace summaries
- Cross-crate reporting

#### Historical Analysis

- version comparisons
- historical reports
- development progression tracking

### CI Integration

- Failing builds when maturity decreases
- Minimum maturity thresholds
- Pull-request reporting

### Trend Analysis

Examples:
Stable items + 11
Experimental Items - 9
Documentation + 11%
Verification + 9 %

---

## Pre-Stable Phase

Before the first stable release, the focus will shift towards:

- Stabilisation
- Configuration systems
- Performance improvements
- Experimental feature evaluation
- Documentation improvements
- Ecosystem polishing
- Public feedback integration

## V1.0.0 - Stable Release

A stable public release of the maturity ecosystem.

The intention is to provide a tool capable of

- Collecting maturity metadata
- Generating reports
- Tracking project evolution
- Providing configurable maturity systems
- Workspace-level reporting
- CI Integration
- Making software development state easier to understand and maintain over long periods of time
