//! Standalone CLI entry point for `maturity`
//!
//! This binary exists primarily for local development, testing, and experimentation while the project is still evolving.
//!
//! The actual command implementations live inside `maturity-report`. This crate simply forwards command-line arguments to
//! the shared cli logic infrastructure.
//!

// TODO: Move argument ownership into this crate.
fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    // DEBUG
    // println!("{:?}", std::env::args().collect::<Vec<_>>());
    // for (index, arg) in std::env::args().enumerate() {
    // println!("{index}: {arg}");
    // }
    maturity_report::cli::run(args);
}
