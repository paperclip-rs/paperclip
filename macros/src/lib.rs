//! Convenience macros for [paperclip](https://github.com/wafflespeanut/paperclip).
//!
//! You shouldn't need to depend on this, because the stuff here is
//! already exposed by the corresponding crates.

#![feature(proc_macro_diagnostic)]
#![recursion_limit = "512"]

extern crate proc_macro;

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "default")]
mod core;

use proc_macro::{Span, TokenStream};
use quote::quote;

/// Converts your struct to support deserializing from an OpenAPI v2
/// [Schema](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject)
/// object ([example](https://paperclip.waffles.space/paperclip/v2/)). This adds the necessary fields (in addition to your own fields) and implements the
/// `Schema` trait for parsing and codegen.
#[cfg(feature = "v2")]
#[proc_macro_attribute]
pub fn api_v2_schema_struct(_attr: TokenStream, input: TokenStream) -> TokenStream {
    self::core::emit_v2_schema_struct(input)
}

/// Marker attribute for indicating that a function is an OpenAPI v2 compatible operation.
#[cfg(feature = "actix")]
#[proc_macro_attribute]
pub fn api_v2_operation(_attr: TokenStream, input: TokenStream) -> TokenStream {
    self::actix::emit_v2_operation(input)
}

/// Marker attribute for indicating that an object is an OpenAPI v2 compatible definition.
#[cfg(feature = "actix")]
#[proc_macro_attribute]
pub fn api_v2_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
    self::actix::emit_v2_definition(input)
}

/// Generate an error in the call site and return empty token stream.
fn call_site_error_with_msg(msg: &str) -> TokenStream {
    Span::call_site().error(msg);
    (quote! {}).into()
}
