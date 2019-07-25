//! Convenience macros for the [actix-web](https://github.com/wafflespeanut/paperclip/tree/master/plugins/actix-web)
//! OpenAPI plugin (exposed by paperclip with `actix` feature).

mod operation;

use self::operation::OperationProducer;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FnArg, ItemFn, PathArguments, ReturnType, Token, Type};

/// Actual parser and emitter for `api_v2_operation` macro.
pub fn emit_v2_operation(input: TokenStream) -> TokenStream {
    let item_ast: ItemFn = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return crate::call_site_error_with_msg("operation must be a function"),
    };

    let name = item_ast.ident.clone();
    let mut arg_types = quote!();
    let mut arg_names = quote!();
    for arg in &item_ast.decl.inputs {
        if let FnArg::Captured(ref cap) = &arg {
            let (pat, ty) = (&cap.pat, &cap.ty);
            arg_types.extend(quote!(#ty,));
            arg_names.extend(quote!(#pat,));
        }
    }

    let op = match OperationProducer::from(&item_ast).generate_definition() {
        Ok(o) => o,
        Err(ts) => return ts,
    };

    let block = &item_ast.block;
    let ret = match &item_ast.decl.output {
        ReturnType::Type(_, ref ty) => ty,
        // unreachable because we've already dealt with this in `OperationProducer::generate_definition`
        ReturnType::Default => unreachable!(),
    };

    let gen = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        struct #name;

        impl actix_web::dev::Factory<(#arg_types), #ret> for #name {
            fn call(&self, (#arg_names): (#arg_types)) -> #ret #block
        }

        impl paperclip::v2::schema::Apiv2Operation for #name {
            #op
        }
    };

    gen.into()
}

/// Actual parser and emitter for `api_v2_schema` macro.
pub fn emit_v2_definition(input: TokenStream) -> TokenStream {
    let item_ast: DeriveInput = match syn::parse(input) {
        Ok(s) => s,
        Err(_) => return crate::call_site_error_with_msg("schema must be struct or enum"),
    };

    let name = &item_ast.ident;
    let generics = &item_ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // FIXME: Support enums and unit structs.
    let fields = match &item_ast.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(ref f) => &f.named,
            _ => {
                return crate::call_site_error_with_msg(
                    "expected struct with zero or more fields for schema",
                )
            }
        },
        _ => return crate::call_site_error_with_msg("expected struct for schema"),
    };

    // FIXME: Use attr path segments to find serde renames, flattening, skipping, etc.
    let mut props_gen = quote! {};
    for field in fields {
        let field_name = field
            .ident
            .as_ref()
            .expect("missing field name?")
            .to_string();

        let mut is_required = true;
        let ty_ref = match field.ty {
            Type::Path(ref p) => {
                let ty = p
                    .path
                    .segments
                    .last()
                    .map(|p| p.into_value())
                    .expect("expected type for struct field");

                if p.path.segments.len() == 1 && &ty.ident == "Option" {
                    is_required = false;
                }

                address_type_for_fn_call(&field.ty)
            }
            _ => return crate::call_site_error_with_msg("unsupported type for schema"),
        };

        let mut gen = quote!(
            {
                let mut s = DefaultSchemaRaw::default();
                s.data_type = Some(#ty_ref::data_type());
                s.format = #ty_ref::format();
                schema.properties.insert(#field_name.into(), s.into());
            }
        );

        if is_required {
            gen.extend(quote! {
                schema.required.insert(#field_name.into());
            });
        }

        props_gen.extend(gen);
    }

    let schema_name = name.to_string();
    let gen = quote! {
        #item_ast

        impl #impl_generics paperclip::v2::schema::Apiv2Schema for #name #ty_generics #where_clause {
            const NAME: Option<&'static str> = Some(#schema_name);

            fn raw_schema() -> paperclip::v2::models::DefaultSchemaRaw {
                use paperclip::v2::models::{DataType, DataTypeFormat, DefaultSchemaRaw};
                use paperclip::v2::schema::TypedData;

                let mut schema = DefaultSchemaRaw::default();
                schema.name = Some(#schema_name.into()); // Add name for later use.
                #props_gen
                schema
            }
        }
    };

    gen.into()
}

/// An associated function of a generic type, say, a vector cannot be called
/// like `Vec::foo` as it doesn't have a default type. We should instead call
/// `Vec::<T>::foo`. This function takes care of that special treatment.
fn address_type_for_fn_call(old_ty: &Type) -> Type {
    let mut ty = old_ty.clone();
    if let Type::Path(ref mut p) = &mut ty {
        p.path.segments.pairs_mut().for_each(|mut pair| {
            let is_empty = pair.value().arguments.is_empty();
            let args = &mut pair.value_mut().arguments;
            match args {
                PathArguments::AngleBracketed(ref mut brack_args) if !is_empty => {
                    brack_args.colon2_token = Some(Token![::](proc_macro2::Span::call_site()));
                }
                _ => (),
            }
        });
    }

    ty
}
