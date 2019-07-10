//! Convenience macros for [paperclip](https://github.com/wafflespeanut/paperclip).
//!
//! You shouldn't need to depend on this, because the attributes here are
//! already exposed by paperclip.

#![feature(proc_macro_diagnostic)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, Ident};

fn call_site_error_with_msg(msg: &str) -> TokenStream {
    Span::call_site().error(msg);
    (quote! {}).into()
}

fn schema_fields(name: &Ident, is_ref: bool) -> proc_macro2::TokenStream {
    let mut gen = quote!();
    let add_self = |gen: &mut proc_macro2::TokenStream| {
        if is_ref {
            gen.extend(quote!(paperclip::v2::models::SchemaRepr<#name>>,));
        } else {
            gen.extend(quote!(Box<#name>>,));
        }
    };

    gen.extend(quote!(
        #[serde(rename = "$ref")]
        pub reference: Option<String>,
    ));
    gen.extend(quote!(
        pub title: Option<String>,
    ));
    gen.extend(quote!(
        pub description: Option<String>,
    ));
    gen.extend(quote!(
        #[serde(rename = "type")]
        pub data_type: Option<paperclip::v2::models::DataType>,
    ));
    gen.extend(quote!(
        pub format: Option<paperclip::v2::models::DataTypeFormat>,
    ));

    gen.extend(quote!(#[serde(default)] pub properties: std::collections::BTreeMap<String, ));
    add_self(&mut gen);

    gen.extend(quote!(pub items: Option<));
    add_self(&mut gen);

    gen.extend(quote!(#[serde(rename = "additionalProperties")] pub extra_props: Option<));
    add_self(&mut gen);

    gen.extend(quote!(
        #[serde(default)]
        pub required: std::collections::BTreeSet<String>,
    ));

    if is_ref {
        gen.extend(quote!(
            #[serde(skip)]
            name: Option<String>,
            #[serde(skip)]
            cyclic: bool,
        ));
    }

    quote!({
        #gen
    })
}

fn named_fields(item_ast: &mut DeriveInput) -> Result<&mut FieldsNamed, TokenStream> {
    match &mut item_ast.data {
        Data::Struct(s) => match &mut s.fields {
            Fields::Named(ref mut f) => Ok(f),
            _ => Err(call_site_error_with_msg(
                "expected struct with zero or more fields for schema",
            )),
        },
        _ => Err(call_site_error_with_msg("expected struct for schema")),
    }
}

fn raw_schema(mut item_ast: DeriveInput) -> Result<proc_macro2::TokenStream, TokenStream> {
    let ident = Ident::new(
        &format!("{}Raw", item_ast.ident),
        proc_macro2::Span::call_site(),
    );
    item_ast.ident = ident.clone();

    let fields = match named_fields(&mut item_ast) {
        Ok(f) => f,
        Err(s) => return Err(s),
    };

    let default_fields: FieldsNamed =
        syn::parse2(schema_fields(&ident, false)).expect("parsing schema fields?");
    fields.named.extend(default_fields.named);
    Ok(quote!(#item_ast))
}

/// Converts your struct to support deserializing from an OpenAPI v2
/// [Schema](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject)
/// object ([example](https://paperclip.waffles.space/paperclip/v2/)). This adds the necessary fields (in addition to your own fields) and implements the
/// `Schema` trait for parsing and codegen.
#[proc_macro_attribute]
pub fn api_v2_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_ast: DeriveInput = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return call_site_error_with_msg("error parsing derive input"),
    };

    let name = item_ast.ident.clone();
    let generics = item_ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut gen = match raw_schema(item_ast.clone()) {
        Ok(ts) => ts,
        Err(ts) => return ts,
    };

    let fields = match named_fields(&mut item_ast) {
        Ok(f) => f,
        Err(s) => return s,
    };

    let default_fields: FieldsNamed =
        syn::parse2(schema_fields(&name, true)).expect("parsing schema fields?");
    fields.named.extend(default_fields.named);

    let mut defaults = quote!();
    for field in &fields.named {
        let f_name = field.ident.as_ref().expect("fields not named?");
        defaults.extend(quote!(#f_name: Default::default(),));
    }

    gen.extend(quote! {
        #item_ast

        impl Default for #name {
            fn default() -> #name {
                #name {
                    #defaults
                }
            }
        }

        impl #impl_generics paperclip::v2::Schema for #name #ty_generics #where_clause {
            #[inline]
            fn name(&self) -> Option<&str> {
                self.name.as_ref().map(String::as_str)
            }

            #[inline]
            fn set_name(&mut self, name: &str) {
                self.name = Some(name.into());
            }

            #[inline]
            fn set_cyclic(&mut self, cyclic: bool) {
                self.cyclic = cyclic;
            }

            #[inline]
            fn is_cyclic(&self) -> bool {
                self.cyclic
            }

            #[inline]
            fn description(&self) -> Option<&str> {
                self.description.as_ref().map(String::as_str)
            }

            #[inline]
            fn reference(&self) -> Option<&str> {
                self.reference.as_ref().map(String::as_str)
            }

            #[inline]
            fn data_type(&self) -> Option<paperclip::v2::models::DataType> {
                self.data_type
            }

            #[inline]
            fn format(&self) -> Option<&paperclip::v2::models::DataTypeFormat> {
                self.format.as_ref()
            }

            #[inline]
            fn items(&self) -> Option<&paperclip::v2::models::SchemaRepr<Self>> {
                self.items.as_ref()
            }

            #[inline]
            fn items_mut(&mut self) -> Option<&mut paperclip::v2::models::SchemaRepr<Self>> {
                self.items.as_mut()
            }

            #[inline]
            fn additional_properties(&self) -> Option<&paperclip::v2::models::SchemaRepr<Self>> {
                self.extra_props.as_ref()
            }

            #[inline]
            fn additional_properties_mut(&mut self) -> Option<&mut paperclip::v2::models::SchemaRepr<Self>> {
                self.extra_props.as_mut()
            }

            #[inline]
            fn properties(&self) -> Option<&std::collections::BTreeMap<String, paperclip::v2::models::SchemaRepr<Self>>> {
                if self.properties.is_empty() {
                    None
                } else {
                    Some(&self.properties)
                }
            }

            #[inline]
            fn properties_mut(&mut self) -> Option<&mut std::collections::BTreeMap<String, paperclip::v2::models::SchemaRepr<Self>>> {
                if self.properties.is_empty() {
                    None
                } else {
                    Some(&mut self.properties)
                }
            }

            #[inline]
            fn required_properties(&self) -> Option<&std::collections::BTreeSet<String>> {
                if self.required.is_empty() {
                    None
                } else {
                    Some(&self.required)
                }
            }
        }
    });

    // panic!("{}", gen);
    gen.into()
}
