#![feature(proc_macro_diagnostic)]
#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed};

fn call_site_error_with_msg(msg: &str) -> TokenStream {
    Span::call_site().error(msg);
    (quote! {}).into()
}

#[proc_macro_attribute]
pub fn api_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
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
            _ => return call_site_error_with_msg("expected field'able struct for schema"),
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
            pub data_type: Option<paperclip_openapi::v2::models::DataType>,
            pub format: Option<paperclip_openapi::v2::models::DataTypeFormat>,
            pub properties: Option<std::collections::BTreeMap<String, paperclip_openapi::v2::im::RcRefCell<#name>>>,
            pub items: Option<paperclip_openapi::v2::im::RcRefCell<#name>>,
        }
    })
    .expect("parsing schema field?");
    fields.extend(default_fields.named);

    let gen = quote! {
        #item_ast

        impl #impl_generics paperclip_openapi::v2::Schemable for #name #ty_generics #where_clause {
            fn reference(&self) -> Option<&str> {
                self.reference.as_ref().map(String::as_str)
            }

            fn items_mut(&mut self) -> Option<&mut paperclip_openapi::v2::im::RcRefCell<Self>> {
                self.items.as_mut()
            }

            fn properties_mut(&mut self) -> Option<&mut std::collections::BTreeMap<String, paperclip_openapi::v2::im::RcRefCell<Self>>> {
                self.properties.as_mut()
            }
        }
    };

    gen.into()
}
