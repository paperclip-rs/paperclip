//! Convenience macros for [paperclip](https://github.com/wafflespeanut/paperclip).
//!
//! You shouldn't need to depend on this, because the stuff here is
//! already exposed by the corresponding crates.

#![recursion_limit = "512"]

extern crate proc_macro;
#[macro_use]
extern crate proc_macro_error;

#[cfg(feature = "actix")]
mod actix;
#[cfg(feature = "default")]
mod core;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    DeriveInput, NestedMeta,
};

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
#[proc_macro_error]
#[proc_macro_attribute]
pub fn api_v2_operation(_attr: TokenStream, input: TokenStream) -> TokenStream {
    self::actix::emit_v2_operation(input)
}

/// Marker attribute for indicating that an object is an OpenAPI v2 compatible definition.
#[cfg(feature = "actix")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn api_v2_schema(attrs: TokenStream, input: TokenStream) -> TokenStream {
    self::actix::emit_v2_definition(attrs, input)
}

/// Marker attribute for indicating that the marked object can represent non-2xx (error)
/// status codes with optional descriptions.
#[cfg(feature = "actix")]
#[proc_macro_error]
#[proc_macro_attribute]
pub fn api_v2_errors(attrs: TokenStream, input: TokenStream) -> TokenStream {
    self::actix::emit_v2_errors(attrs, input)
}

/// Generate an error at the call site and return empty token stream.
fn span_error_with_msg<T: Spanned>(it: &T, msg: &str) -> TokenStream {
    emit_error!(it.span().unwrap(), msg);
    (quote! {}).into()
}

/// Parses this token stream expecting a struct/enum and fails with an error otherwise.
fn expect_struct_or_enum(ts: TokenStream) -> Result<DeriveInput, TokenStream> {
    syn::parse(ts).map_err(|e| {
        emit_error!(
            e.span().unwrap(),
            "expected struct or enum for deriving schema."
        );
        quote!().into()
    })
}

/// Helper struct for parsing proc-macro input attributes.
#[derive(Default)]
struct MacroAttribute(Punctuated<NestedMeta, Comma>);

impl Parse for MacroAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroAttribute(input.call(Punctuated::parse_terminated)?))
    }
}

#[cfg(feature = "actix")]
fn parse_input_attrs(ts: TokenStream) -> MacroAttribute {
    syn::parse(ts)
        .map_err(|e| {
            emit_warning!(
                e.span().unwrap(),
                "cannot parse proc-macro input attributes."
            );
        })
        .ok()
        .unwrap_or_default()
}
