//! Convenience macros for the [actix-web](https://github.com/wafflespeanut/paperclip/tree/master/plugins/actix-web)
//! OpenAPI plugin.
//!
//! You shouldn't need to depend on this, because the attributes here are
//! already exposed by that plugin.

#![feature(proc_macro_diagnostic)]
#![recursion_limit = "128"]

extern crate proc_macro;

mod schema;

use self::schema::OperationProducer;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, FnArg, ItemFn, ReturnType, Type};

fn call_site_error_with_msg(msg: &str) -> TokenStream {
    Span::call_site().error(msg);
    (quote! {}).into()
}

/// Marker attribute for indicating that a function is an OpenAPI v2 compatible operation.
#[proc_macro_attribute]
pub fn api_v2_operation(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_ast: ItemFn = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return call_site_error_with_msg("operation must be a function"),
    };

    let name = item_ast.ident.clone();
    let mut arg_types = quote!();
    let mut arg_names = quote!();
    for arg in &item_ast.decl.inputs {
        if let FnArg::Captured(ref cap) = &arg {
            let (pat, ty) = (&cap.pat, &cap.ty);
            arg_types.extend(quote!(#ty));
            arg_names.extend(quote!(#pat));
        }
    }

    let op = match OperationProducer::from(&item_ast).generate_definition() {
        Ok(o) => o,
        Err(ts) => return ts,
    };

    let block = &item_ast.block;
    let ret = match &item_ast.decl.output {
        ReturnType::Type(_, ref ty) => ty,
        // unreachable because we've already dealt with this in `infer_operation_definition`
        ReturnType::Default => unreachable!(),
    };

    let gen = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        struct #name;

        impl actix_web::Factory<(#arg_types), #ret> for #name {
            fn call(&self, (#arg_names): (#arg_types)) -> #ret #block
        }

        impl paperclip_actix::ApiOperation for #name {
            #op
        }
    };

    gen.into()
}

/// Marker attribute for indicating that an object is an OpenAPI v2 compatible definition.
#[proc_macro_attribute]
pub fn api_v2_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_ast: DeriveInput = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return call_site_error_with_msg("schema must be struct or enum"),
    };

    let name = &item_ast.ident;
    let generics = &item_ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // FIXME: Support enums and unit structs.
    let fields = match &item_ast.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(ref f) => &f.named,
            _ => {
                return call_site_error_with_msg(
                    "expected struct with zero or more fields for schema",
                )
            }
        },
        _ => return call_site_error_with_msg("expected struct for schema"),
    };

    // FIXME: Use attr path segments to find serde renames, flattening, skipping, etc.
    let mut props_gen = quote! {};
    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .expect("missing field name?")
            .to_string();
        let ty = match field.ty {
            Type::Path(ref p) => {
                &p.path
                    .segments
                    .last()
                    .expect("expected type for struct field")
                    .value()
                    .ident
            }
            _ => return call_site_error_with_msg("unsupported type for schema"),
        };

        let gen = quote!(
            {
                let mut s = DefaultSchemaRaw::default();
                s.data_type = Some(#ty::data_type());
                s.format = #ty::format();
                schema.properties.insert(#field_name.into(), s.into());
            }
        );

        props_gen.extend(gen);
    }

    let schema_name = name.to_string();
    let gen = quote! {
        #item_ast

        impl #impl_generics paperclip_actix::Apiv2Schema for #name #ty_generics #where_clause {
            const NAME: Option<&'static str> = Some(#schema_name);

            fn schema() -> paperclip::v2::models::DefaultSchemaRaw {
                use paperclip::v2::models::{DataType, DataTypeFormat, DefaultSchemaRaw, TypedData};
                let mut schema = DefaultSchemaRaw::default();
                #props_gen
                schema
            }
        }
    };

    // panic!("{}", gen);
    gen.into()
}
