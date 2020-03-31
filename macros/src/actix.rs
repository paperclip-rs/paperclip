//! Convenience macros for the [actix-web](https://github.com/wafflespeanut/paperclip/tree/master/plugins/actix-web)
//! OpenAPI plugin (exposed by paperclip with `actix` feature).

use heck::*;
use http::StatusCode;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use strum_macros::EnumString;
use syn::spanned::Spanned;
use syn::{
    punctuated::Punctuated, Attribute, Data, DataEnum, Field, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Ident, ItemFn, Lit, Meta, NestedMeta, PathArguments, ReturnType, Token, TraitBound,
    Type,
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
            emit_error!(e.span().unwrap(), "operation must be a function.");
            return quote!().into();
        }
    };

    let mut wrapper = None;
    match &mut item_ast.sig.output {
        ReturnType::Default => {
            emit_warning!(
                item_ast.span().unwrap(),
                "operation doesn't seem to return a response."
            );
        }
        ReturnType::Type(_, ty) => {
            let t = quote!(#ty).to_string();
            // FIXME: This is a hack for functions returning known
            // `impl Trait`. Need a better way!
            if t.contains("Responder") {
                wrapper = Some(quote!(paperclip::actix::ResponderWrapper));
            }

            if let (Type::ImplTrait(_), Some(ref w)) = (&**ty, wrapper.as_ref()) {
                if item_ast.sig.asyncness.is_some() {
                    *ty = Box::new(syn::parse2(quote!(#w<#ty>)).expect("parsing wrapper type"));
                } else {
                    *ty = Box::new(
                        syn::parse2(quote!(impl Future<Output=#w<#ty>>))
                            .expect("parsing wrapper type"),
                    );
                }
            }
        }
    }

    if let Some(w) = wrapper {
        let block = item_ast.block;
        let wrapped_value = if item_ast.sig.asyncness.is_some() {
            quote!(#w(f))
        } else {
            quote!(futures::future::ready(#w(f)))
        };
        item_ast.block = Box::new(
            syn::parse2(quote!(
                {
                    let f = (|| {
                        #block
                    })();
                    #wrapped_value
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

/// Actual parser and emitter for `api_v2_errors` macro.
pub fn emit_v2_errors(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item_ast = match crate::expect_struct_or_enum(input) {
        Ok(i) => i,
        Err(ts) => return ts,
    };

    let name = &item_ast.ident;
    let attrs = crate::parse_input_attrs(attrs);
    let generics = item_ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Convert macro attributes to tuples in form of (u16, &str)
    let error_codes = attrs
        .0
        .iter()
        // Pair code attrs with description attrs; save attr itself to properly span error messages at later stage
        .fold(Vec::new(), |mut list: Vec<(Option<u16>, Option<String>, _)>, attr| {
            let span = attr.span().unwrap();
            match attr {
                // Read named attribute.
                NestedMeta::Meta(Meta::NameValue(name_value)) => {
                    let attr_name = name_value.path.get_ident().map(|ident| ident.to_string());
                    let attr_value = &name_value.lit;

                    match (attr_name.as_deref(), attr_value) {
                        // "code" attribute adds new element to list
                        (Some("code"), Lit::Int(attr_value)) => {
                            let status_code = attr_value.base10_parse::<u16>()
                                .map_err(|_| emit_error!(span, "Invalid u16 in code argument")).ok();
                            list.push((status_code, None, attr));
                        },
                        // "description" attribute updates last element in list
                        (Some("description"), Lit::Str(attr_value)) =>
                            if let Some(last_value) = list.last_mut() {
                                if last_value.1.is_some() {
                                    emit_warning!(span, "This attribute overwrites previous description");
                                }
                                last_value.1 = Some(attr_value.value());
                            } else {
                                emit_error!(span, "Attribute 'description' can be only placed after prior 'code' argument");
                            },
                        _ => emit_error!(span, "Invalid macro attribute. Should be plain u16, 'code = u16' or 'description = str'")
                    }
                },
                // Read plain status code as attribute.
                NestedMeta::Lit(Lit::Int(attr_value)) => {
                    let status_code = attr_value.base10_parse::<u16>()
                    .map_err(|_| emit_error!(span, "Invalid u16 in code argument")).ok();
                    list.push((status_code, None, attr));
                },
                _ => emit_error!(span, "This macro supports only named attributes - 'code' (u16) or 'description' (str)")
            }

            list
        })
        .iter()
        // Map code-message pairs into bits of code, filter empty codes out
        .filter_map(|triple| {
            let (code, description) = match triple {
                (Some(code), Some(description), _) => (code, description.to_owned()),
                (Some(code), None, attr) => {
                    let span = attr.span().unwrap();
                    let description = StatusCode::from_u16(*code)
                        .map_err(|_| {
                            emit_warning!(span, format!("Invalid status code {}", code));
                            String::new()
                        })
                        .map(|s| s.canonical_reason()
                            .map(str::to_string)
                            .unwrap_or_else(|| {
                                emit_warning!(span, format!("Status code {} doesn't have a canonical name", code));
                                String::new()
                            })
                        )
                        .unwrap_or_else(|_| String::new());
                    (code, description)
                },
                (None, _, _) => return None,
            };

            Some(quote! {
                (#code, #description),
            })
        })
        .fold(proc_macro2::TokenStream::new(), |mut stream, tokens| {
            stream.extend(tokens);
            stream
        });

    let gen = quote! {
        #item_ast

        impl #impl_generics paperclip::v2::schema::Apiv2Errors for #name #ty_generics #where_clause {
            const ERROR_MAP: &'static [(u16, &'static str)] = &[
                #error_codes
            ];
        }
    };

    gen.into()
}

/// Actual parser and emitter for `api_v2_schema` macro.
pub fn emit_v2_definition(input: TokenStream) -> TokenStream {
    let item_ast = match crate::expect_struct_or_enum(input) {
        Ok(i) => i,
        Err(ts) => return ts,
    };

    let docs = extract_documentation(&item_ast.attrs);
    let docs = docs.trim();

    let props = SerdeProps::from_item_attrs(&item_ast.attrs);
    let needs_empty_schema = extract_openapi_attrs(&item_ast.attrs).any(|nested| {
        nested.len() == 1
            && match &nested[0] {
                NestedMeta::Meta(Meta::Path(path)) => path.is_ident("empty"),
                _ => false,
            }
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

    let opt_impl = add_optional_impl(&name, &generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    if needs_empty_schema {
        return quote!(
            impl #impl_generics paperclip::v2::schema::Apiv2Schema for #name #ty_generics #where_clause {
                const DESCRIPTION: &'static str = #docs;
            }

            #opt_impl
        ).into();
    }

    // FIXME: Use attr path segments to find flattening, skipping, etc.
    let mut props_gen = quote! {};

    match &item_ast.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(ref f) => handle_field_struct(f, &props, &mut props_gen),
            Fields::Unnamed(ref f) => {
                if f.unnamed.len() == 1 {
                    handle_unnamed_field_struct(f, &mut props_gen)
                } else {
                    emit_warning!(
                        f.span().unwrap(),
                        "tuple structs do not have named fields and hence will have empty schema.";
                        help = "{}", &*EMPTY_SCHEMA_HELP;
                    );
                }
            }
            Fields::Unit => {
                emit_warning!(
                    s.struct_token.span().unwrap(),
                    "unit structs do not have any fields and hence will have empty schema.";
                    help = "{}", &*EMPTY_SCHEMA_HELP;
                );
            }
        },
        Data::Enum(ref e) => handle_enum(e, &props, &mut props_gen),
        Data::Union(ref u) => emit_error!(
            u.union_token.span().unwrap(),
            "unions are unsupported for deriving schema"
        ),
    };

    let schema_name = name.to_string();
    let gen = quote! {
        impl #impl_generics paperclip::v2::schema::Apiv2Schema for #name #ty_generics #where_clause {
            const NAME: Option<&'static str> = Some(#schema_name);

            const DESCRIPTION: &'static str = #docs;

            fn raw_schema() -> paperclip::v2::models::DefaultSchemaRaw {
                use paperclip::v2::models::{DataType, DataTypeFormat, DefaultSchemaRaw};
                use paperclip::v2::schema::TypedData;

                let mut schema = DefaultSchemaRaw::default();
                #props_gen
                schema.name = Some(#schema_name.into()); // Add name for later use.
                schema
            }
        }

        #opt_impl
    };

    gen.into()
}

#[cfg(feature = "nightly")]
fn add_optional_impl(_: &Ident, _: &Generics) -> proc_macro2::TokenStream {
    // Empty impl for "nightly" feature because specialization helps us there.
    quote!()
}

#[cfg(not(feature = "nightly"))]
fn add_optional_impl(name: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics paperclip::actix::OperationModifier for #name #ty_generics #where_clause {}
    }
}

fn get_field_type(field: &Field) -> (Option<proc_macro2::TokenStream>, bool) {
    let mut is_required = true;
    match field.ty {
        Type::Path(ref p) => {
            let ty = p
                .path
                .segments
                .last()
                .expect("expected type for struct field");

            if p.path.segments.len() == 1 && &ty.ident == "Option" {
                is_required = false;
            }

            (Some(address_type_for_fn_call(&field.ty)), is_required)
        }
        Type::Reference(_) => (Some(address_type_for_fn_call(&field.ty)), is_required),
        _ => {
            emit_warning!(
                field.ty.span().unwrap(),
                "unsupported field type will be ignored."
            );
            (None, is_required)
        }
    }
}

/// Generates code for a tuple struct with fields.
fn handle_unnamed_field_struct(fields: &FieldsUnnamed, props_gen: &mut proc_macro2::TokenStream) {
    let field = fields.unnamed.iter().next().unwrap();
    let (ty_ref, _) = get_field_type(&field);

    if let Some(ty_ref) = ty_ref {
        props_gen.extend(quote!({
            schema = #ty_ref::raw_schema();
        }));
    }
}

/// Checks for `api_v2_empty` attributes and removes them.
fn extract_openapi_attrs<'a>(
    field_attrs: &'a [Attribute],
) -> impl Iterator<Item = Punctuated<syn::NestedMeta, syn::token::Comma>> + 'a {
    field_attrs.iter().filter_map(|a| match a.parse_meta() {
        Ok(Meta::List(list)) if list.path.is_ident("openapi") => Some(list.nested),
        _ => None,
    })
}

/// Checks for `api_v2_empty` attributes and removes them.
fn extract_documentation(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|a| match a.parse_meta() {
            Ok(Meta::NameValue(mnv)) if mnv.path.is_ident("doc") => match &mnv.lit {
                Lit::Str(s) => Some(s.value()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// Generates code for a struct with fields.
fn handle_field_struct(
    fields: &FieldsNamed,
    serde: &SerdeProps,
    props_gen: &mut proc_macro2::TokenStream,
) {
    for field in &fields.named {
        let mut field_name = field
            .ident
            .as_ref()
            .expect("missing field name?")
            .to_string();

        if let Some(renamed) = SerdeRename::from_field_attrs(&field.attrs) {
            field_name = renamed;
        } else if let Some(prop) = serde.rename {
            field_name = prop.rename(&field_name);
        }

        let (ty_ref, is_required) = get_field_type(&field);

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
fn handle_enum(e: &DataEnum, serde: &SerdeProps, props_gen: &mut proc_macro2::TokenStream) {
    props_gen.extend(quote!(
        schema.data_type = Some(DataType::String);
    ));

    for var in &e.variants {
        let mut name = var.ident.to_string();
        match &var.fields {
            Fields::Unit => (),
            Fields::Named(ref f) => {
                emit_warning!(
                    f.span().unwrap(),
                    "skipping enum variant with named fields in schema."
                );
                continue;
            }
            Fields::Unnamed(ref f) => {
                emit_warning!(f.span().unwrap(), "skipping tuple enum variant in schema.");
                continue;
            }
        }

        if let Some(renamed) = SerdeRename::from_field_attrs(&var.attrs) {
            name = renamed;
        } else if let Some(prop) = serde.rename {
            name = prop.rename(&name);
        }

        props_gen.extend(quote!(
            schema.enum_.push(serde_json::json!(#name));
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

/* Serde attributes */

/// Supported renaming options in serde (https://serde.rs/variant-attrs.html).
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString)]
enum SerdeRename {
    #[strum(serialize = "lowercase")]
    Lower,
    #[strum(serialize = "UPPERCASE")]
    Upper,
    #[strum(serialize = "PascalCase")]
    Pascal,
    #[strum(serialize = "camelCase")]
    Camel,
    #[strum(serialize = "snake_case")]
    Snake,
    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
    #[strum(serialize = "kebab-case")]
    Kebab,
    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

impl SerdeRename {
    /// Traverses the field attributes and returns the renamed value from the first matching
    /// `#[serde(rename = "...")]` pattern.
    fn from_field_attrs(field_attrs: &[Attribute]) -> Option<String> {
        for meta in field_attrs.iter().filter_map(|a| a.parse_meta().ok()) {
            let inner_meta = match meta {
                Meta::List(ref l)
                    if l.path
                        .segments
                        .last()
                        .map(|p| p.ident == "serde")
                        .unwrap_or(false) =>
                {
                    &l.nested
                }
                _ => continue,
            };

            for meta in inner_meta {
                let rename = match meta {
                    NestedMeta::Meta(Meta::NameValue(ref v))
                        if v.path
                            .segments
                            .last()
                            .map(|p| p.ident == "rename")
                            .unwrap_or(false) =>
                    {
                        &v.lit
                    }
                    _ => continue,
                };

                if let Lit::Str(ref s) = rename {
                    return Some(s.value());
                }
            }
        }

        None
    }

    /// Renames the given value using the current option.
    fn rename(self, name: &str) -> String {
        match self {
            SerdeRename::Lower => name.to_lowercase(),
            SerdeRename::Upper => name.to_uppercase(),
            SerdeRename::Pascal => name.to_camel_case(),
            SerdeRename::Camel => name.to_mixed_case(),
            SerdeRename::Snake => name.to_snek_case(),
            SerdeRename::ScreamingSnake => name.to_snek_case().to_uppercase(),
            SerdeRename::Kebab => name.to_kebab_case(),
            SerdeRename::ScreamingKebab => name.to_kebab_case().to_uppercase(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct SerdeProps {
    rename: Option<SerdeRename>,
}

impl SerdeProps {
    /// Traverses the serde attributes in the given item attributes and returns
    /// the applicable properties.
    fn from_item_attrs(item_attrs: &[Attribute]) -> Self {
        let mut props = Self::default();
        for meta in item_attrs.iter().filter_map(|a| a.parse_meta().ok()) {
            let inner_meta = match meta {
                Meta::List(ref l)
                    if l.path
                        .segments
                        .last()
                        .map(|p| p.ident == "serde")
                        .unwrap_or(false) =>
                {
                    &l.nested
                }
                _ => continue,
            };

            for meta in inner_meta {
                let global_rename = match meta {
                    NestedMeta::Meta(Meta::NameValue(ref v))
                        if v.path
                            .segments
                            .last()
                            .map(|p| p.ident == "rename_all")
                            .unwrap_or(false) =>
                    {
                        &v.lit
                    }
                    _ => continue,
                };

                if let Lit::Str(ref s) = global_rename {
                    props.rename = s.value().parse().ok();
                }
            }
        }

        props
    }
}
