//! Convenience macros for the [actix-web](https://github.com/wafflespeanut/paperclip/tree/master/plugins/actix-web)
//! OpenAPI plugin (exposed by paperclip with `actix` feature).

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Data, DataEnum, Fields, FieldsNamed, ItemFn, Meta, NestedMeta, PathArguments, ReturnType,
    Token, TraitBound, Type,
};

const SCHEMA_MACRO: &str = "api_v2_schema";

lazy_static! {
    static ref EMPTY_SCHEMA_HELP: String = format!(
        "you can mark the struct with #[{}(empty)] to ignore this warning.",
        SCHEMA_MACRO
    );
}

/// Actual parser and emitter for `api_v2_operation` macro.
///
/// **NOTE:** This is a no-op right now. It's only reserved for
/// future use to avoid introducing breaking changes.
pub fn emit_v2_operation(input: TokenStream) -> TokenStream {
    let mut item_ast: ItemFn = match syn::parse(input) {
        Ok(s) => s,
        Err(e) => {
            e.span()
                .unwrap()
                .error("operation must be a function.")
                .emit();
            return quote!().into();
        }
    };

    let mut wrapper = None;
    match &mut item_ast.sig.output {
        ReturnType::Default => item_ast
            .span()
            .unwrap()
            .warning("operation doesn't seem to return a response.")
            .emit(),
        ReturnType::Type(_, ty) => {
            let t = quote!(#ty).to_string();
            // FIXME: This is a hack for functions returning known
            // `impl Trait`. Need a better way!
            if t.contains("Future") {
                wrapper = Some(quote!(paperclip::actix::FutureWrapper));
            } else if t.contains("Responder") {
                wrapper = Some(quote!(paperclip::actix::ResponderWrapper));
            }

            if let (Type::ImplTrait(_), Some(ref w)) = (&**ty, wrapper.as_ref()) {
                *ty = Box::new(syn::parse2(quote!(#w<#ty>)).expect("parsing wrapper type"));
            }
        }
    }

    if let Some(w) = wrapper {
        let block = item_ast.block;
        item_ast.block = Box::new(
            syn::parse2(quote!(
                {
                    let f = #block;
                    #w(f)
                }
            ))
            .expect("parsing wrapped block"),
        );
    }

    quote!(
        #item_ast
    )
    .into()
}

/// Actual parser and emitter for `api_v2_schema` macro.
pub fn emit_v2_definition(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item_ast = match crate::expect_struct_or_enum(input) {
        Ok(i) => i,
        Err(ts) => return ts,
    };

    let attrs = crate::parse_input_attrs(attrs);
    let needs_empty_schema = attrs.0.iter().any(|meta| match meta {
        NestedMeta::Meta(Meta::Path(ref n)) => n
            .segments
            .last()
            .map(|p| p.ident == "empty")
            .unwrap_or(false),
        _ => false,
    });

    let name = &item_ast.ident;

    // Add `Apiv2Schema` bound for impl if the type is generic.
    let mut generics = item_ast.generics.clone();
    if !needs_empty_schema {
        let bound = syn::parse2::<TraitBound>(quote!(paperclip::v2::schema::Apiv2Schema))
            .expect("expected to parse trait bound");
        generics.type_params_mut().for_each(|param| {
            param.bounds.push(bound.clone().into());
        });
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    if needs_empty_schema {
        return quote!(
            #item_ast

            impl #impl_generics paperclip::v2::schema::Apiv2Schema for #name #ty_generics #where_clause {}
        ).into();
    }

    // FIXME: Use attr path segments to find serde renames, flattening, skipping, etc.
    let mut props_gen = quote! {};

    match &item_ast.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(ref f) => handle_field_struct(f, &mut props_gen),
            Fields::Unnamed(ref f) => {
                let s = f.span().unwrap();
                s.warning(
                    "tuple structs do not have named fields and hence will have empty schema.",
                )
                .emit();
                s.help(&*EMPTY_SCHEMA_HELP).emit();
            }
            Fields::Unit => {
                let s = s.struct_token.span().unwrap();
                s.warning("unit structs do not have any fields and hence will have empty schema.")
                    .emit();
                s.help(&*EMPTY_SCHEMA_HELP).emit();
            }
        },
        Data::Enum(ref e) => handle_enum(e, &mut props_gen),
        Data::Union(ref u) => u
            .union_token
            .span()
            .unwrap()
            .error("unions are unsupported for deriving schema")
            .emit(),
    };

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

/// Generates code for a struct with fields.
fn handle_field_struct(fields: &FieldsNamed, props_gen: &mut proc_macro2::TokenStream) {
    for field in &fields.named {
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
                    .expect("expected type for struct field");

                if p.path.segments.len() == 1 && &ty.ident == "Option" {
                    is_required = false;
                }

                address_type_for_fn_call(&field.ty)
            }
            Type::Reference(_) => address_type_for_fn_call(&field.ty),
            _ => {
                field
                    .ty
                    .span()
                    .unwrap()
                    .warning("unsupported field type will be ignored.")
                    .emit();
                continue;
            }
        };

        let mut gen = quote!(
            {
                let s = #ty_ref::raw_schema();
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
}

/// Generates code for an enum (if supported).
fn handle_enum(e: &DataEnum, props_gen: &mut proc_macro2::TokenStream) {
    props_gen.extend(quote!(
        schema.data_type = Some(DataType::String);
    ));

    for var in &e.variants {
        let name = var.ident.to_string();
        match &var.fields {
            Fields::Unit => (),
            Fields::Named(ref f) => {
                f.span()
                    .unwrap()
                    .warning("skipping enum variant with named fields in schema.")
                    .emit();
                continue;
            }
            Fields::Unnamed(ref f) => {
                f.span()
                    .unwrap()
                    .warning("skipping tuple enum variant in schema.")
                    .emit();
                continue;
            }
        }

        props_gen.extend(quote!(
            schema.enum_.insert(#name.into());
        ));
    }
}

/// An associated function of a generic type, say, a vector cannot be called
/// like `Vec::foo` as it doesn't have a default type. We should instead call
/// `Vec::<T>::foo`. Something similar applies to `str`. This function takes
/// care of that special treatment.
fn address_type_for_fn_call(old_ty: &Type) -> proc_macro2::TokenStream {
    if let Type::Reference(_) = old_ty {
        return quote!(<(#old_ty)>);
    }

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

    quote!(#ty)
}
