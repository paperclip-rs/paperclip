#![feature(proc_macro_diagnostic)]
#![recursion_limit = "512"]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed};

fn call_site_error_with_msg(msg: &str) -> TokenStream {
    Span::call_site().error(msg);
    (quote! {}).into()
}

/// Converts your struct to support deserializing from an OpenAPI v2
/// [Schema](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject)
/// object. This adds the necessary fields (in addition to your own fields) and implements the
/// `Schema` trait for parsing and codegen.
#[proc_macro_attribute]
pub fn api_v2_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_ast: DeriveInput = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return call_site_error_with_msg("error parsing derive input"),
    };

    let name = &item_ast.ident;
    let generics = &item_ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &mut item_ast.data {
        Data::Struct(s) => match &mut s.fields {
            Fields::Named(f) => &mut f.named,
            _ => {
                return call_site_error_with_msg(
                    "expected struct with zero or more fields for schema",
                )
            }
        },
        _ => return call_site_error_with_msg("expected struct for schema"),
    };

    let default_fields: FieldsNamed = syn::parse2(quote! {
        {
            #[serde(rename = "$ref")]
            pub reference: Option<String>,
            pub title: Option<String>,
            pub description: Option<String>,
            #[serde(rename = "type")]
            pub data_type: Option<paperclip::v2::models::DataType>,
            pub format: Option<paperclip::v2::models::DataTypeFormat>,
            #[serde(default)]
            pub properties: std::collections::BTreeMap<String, paperclip::v2::models::SchemaRepr<#name>>,
            pub items: Option<paperclip::v2::models::SchemaRepr<#name>>,
            #[serde(rename = "additionalProperties")]
            pub extra_props: Option<paperclip::v2::models::SchemaRepr<#name>>,
            #[serde(default)]
            pub required: std::collections::BTreeSet<String>,
            #[serde(skip)]
            name: Option<String>,
            #[serde(skip)]
            cyclic: bool,
        }
    })
    .expect("parsing schema field?");
    fields.extend(default_fields.named);

    let gen = quote! {
        #item_ast

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
    };

    gen.into()
}
