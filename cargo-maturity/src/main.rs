//! Cargo plugin integration for `maturity`
//!
//! This crate provides the bridge between cargo's plugin system and the shared command-line implementation provided by the
//! `maturity-report`.
//!
//! The actual command implementations, reporting logic, and infrastructure do not live here. This crate only handles
//! cargo-specific behaviour so that the standalone CLI and cargo plugin can share the same underlying functionality without
//! duplicating code.
//!
//! Cargo documentations encourages interacting through the command-line interface as the cargo as a library is unstable and
//! it's api's could change without deprecation. To keep cargo-specific integration isolated here makes those differences
//! easier to change.
//!

fn main() {
    // cargo invokes `cargo-maturity` and forwards `maturity` as the first argument. Removing it such that the cargo
    // and the standalone CLI have the identical parsing behaviour.
    let mut args = std::env::args().collect::<Vec<_>>();

    if args.get(1).is_some_and(|arg| arg == "maturity") {
        args.remove(1);
    }

    maturity_report::cli::run(args);
}
