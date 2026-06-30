//! Procedural macros used by `maturity`.
//!
//! These macros provide developer-driven annotations that can later be collected and analysed by reporting tools.

use proc_macro::TokenStream;

/// Marks a Rust item with maturity metadata.
///
/// Currently this macro behaves as a pass-through attribute while the surrounding infrastructure is being developed.
///
/// Future versions will support additional states and reporting features.
#[proc_macro_attribute]
pub fn maturity(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
