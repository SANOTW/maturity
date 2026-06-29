//! Reporting infrastructure for the `maturity` ecosystem.
//!
//! This crate is responsible for:
//!     - discovering rust source files
//!     - collecting project metrics
//!     - generating reports
//!     - exposing command-line interfaces for interacting with them
//!

pub mod cli;
pub mod collector;
pub mod reporter;
pub mod scanner;
pub mod utilities;
