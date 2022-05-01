//! Convenience macros for paperclip (exposed by default).

use proc_macro::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident};

/// Actual parser and emitter for `api_v2_schema_struct` macro.
pub fn emit_v2_schema_struct(input: TokenStream) -> TokenStream {
    let mut item_ast = match crate::expect_struct_or_enum(input) {
        Ok(i) => i,
        Err(ts) => return ts,
    };

    let name = item_ast.ident.clone();
    let generics = item_ast.generics.clone();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate raw schema struct.
    let mut raw_item_ast = item_ast.clone();
    let raw_defaults = match raw_schema(&mut raw_item_ast) {
        Ok(s) => s,
        Err(ts) => return ts,
    };

    let raw_struct_name = &raw_item_ast.ident;
    let mut gen = quote!(
        /// Raw version of schema.
        ///
        /// **NOTE:** This doesn't have smart pointers to reuse definitions
        /// throughout the spec. Instead, it contains the actual schema with
        /// unresolvable `$ref` fields.
        ///
        #raw_item_ast
    );

    let defaults = match actual_schema(&mut item_ast) {
        Ok(s) => s,
        Err(ts) => return ts,
    };

    gen.extend(quote! {
        #item_ast

        impl Default for #name {
            fn default() -> Self {
                #name {
                    #defaults
                }
            }
        }

        impl Default for #raw_struct_name {
            fn default() -> Self {
                #raw_struct_name {
                    #raw_defaults
                }
            }
        }

        impl #raw_struct_name {
            /// Recursively removes all `$ref` values in this schema.
            pub fn remove_refs(&mut self) {
                self.properties.values_mut().for_each(|s| s.remove_refs());
                self.items.as_mut().map(|s| s.remove_refs());
                self.extra_props.as_mut().and_then(|s| s.right_mut()).map(|s| s.remove_refs());
                self.reference = None;
            }

            /// Recursively removes all properties other than `$ref` value
            /// if the `$ref` is non-null.
            pub fn retain_ref(&mut self) {
                if self.reference.is_some() {
                    let ref_ = self.reference.take();
                    *self = Self::default();
                    self.reference = ref_;
                } else {
                    self.properties.values_mut().for_each(|s| s.retain_ref());
                    self.items.as_mut().map(|s| s.retain_ref());
                    self.extra_props.as_mut().and_then(|s| s.right_mut()).map(|s| s.retain_ref());
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
            fn set_reference(&mut self, ref_: String) {
                self.reference = Some(ref_);
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
            fn items(&self) -> Option<&paperclip::v2::models::Resolvable<Self>> {
                self.items.as_ref()
            }

            #[inline]
            fn items_mut(&mut self) -> Option<&mut paperclip::v2::models::Resolvable<Self>> {
                self.items.as_mut()
            }

            #[inline]
            fn additional_properties(&self) -> Option<&paperclip::v2::models::Either<bool, paperclip::v2::models::Resolvable<Self>>> {
                self.extra_props.as_ref()
            }

            #[inline]
            fn additional_properties_mut(&mut self) -> Option<&mut paperclip::v2::models::Either<bool, paperclip::v2::models::Resolvable<Self>>> {
                self.extra_props.as_mut()
            }

            #[inline]
            fn properties(&self) -> Option<&std::collections::BTreeMap<String, paperclip::v2::models::Resolvable<Self>>> {
                if self.properties.is_empty() {
                    None
                } else {
                    Some(&self.properties)
                }
            }

            #[inline]
            fn properties_mut(&mut self) -> Option<&mut std::collections::BTreeMap<String, paperclip::v2::models::Resolvable<Self>>> {
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

            #[inline]
            fn enum_variants(&self) -> Option<&[serde_json::Value]> {
                if self.enum_.is_empty() {
                    return None
                } else {
                    Some(&self.enum_)
                }
            }
        }
    });

    gen.into()
}

/// Generates a raw schema struct with suffix "{structName}Raw".
fn raw_schema(item_ast: &mut DeriveInput) -> Result<proc_macro2::TokenStream, TokenStream> {
    let ident = Ident::new(
        &format!("{}Raw", item_ast.ident),
        proc_macro2::Span::call_site(),
    );
    item_ast.ident = ident.clone();

    let fields = named_fields(item_ast)?;
    let default_fields: FieldsNamed =
        syn::parse2(schema_fields(&ident, false)).expect("parsing schema fields?");
    fields.named.extend(default_fields.named);

    let mut defaults = quote!();
    for field in &fields.named {
        let f_name = field.ident.as_ref().expect("fields not named?");
        defaults.extend(quote!(#f_name: Default::default(),));
    }

    Ok(defaults)
}

/// Generates the actual schema struct with the actual name.
fn actual_schema(item_ast: &mut DeriveInput) -> Result<proc_macro2::TokenStream, TokenStream> {
    let name = item_ast.ident.clone();
    let fields = named_fields(item_ast)?;

    let default_fields: FieldsNamed =
        syn::parse2(schema_fields(&name, true)).expect("parsing schema fields?");
    fields.named.extend(default_fields.named);

    let mut defaults = quote!();
    for field in &fields.named {
        let f_name = field.ident.as_ref().expect("fields not named?");
        defaults.extend(quote!(#f_name: Default::default(),));
    }

    Ok(defaults)
}

/// Extracts named fields from the given struct.
fn named_fields(item_ast: &mut DeriveInput) -> Result<&mut FieldsNamed, TokenStream> {
    let span = item_ast.span();
    if let Data::Struct(s) = &mut item_ast.data {
        match &mut s.fields {
            Fields::Named(ref mut f) => Ok(f),
            Fields::Unnamed(ref f) => Err(crate::span_error_with_msg(
                f,
                "expected struct with zero or more fields for schema",
            )),
            f @ Fields::Unit => {
                *f = Fields::Named(syn::parse2(quote!({})).expect("parsing empty named fields"));
                match f {
                    Fields::Named(ref mut f) => Ok(f),
                    _ => unreachable!(),
                }
            }
        }
    } else {
        emit_error!(span.unwrap(), "expected struct for schema");
        Err(quote!().into())
    }
}

/// Generates fields for a schema struct using its name. Also takes a
/// boolean to indicate whether this struct's fields hold references.
fn schema_fields(name: &Ident, is_ref: bool) -> proc_macro2::TokenStream {
    let mut gen = quote!();
    let add_self = |gen: &mut proc_macro2::TokenStream| {
        if is_ref {
            gen.extend(quote!(paperclip::v2::models::Resolvable<#name>));
        } else {
            gen.extend(quote!(Box<#name>));
        }
    };

    gen.extend(quote!(
        #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
        pub reference: Option<String>,
    ));
    gen.extend(quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
    ));
    gen.extend(quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    ));
    gen.extend(quote!(
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        pub data_type: Option<paperclip::v2::models::DataType>,
    ));
    gen.extend(quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
        pub format: Option<paperclip::v2::models::DataTypeFormat>,
    ));
    gen.extend(quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
        pub example: Option<serde_json::Value>,
    ));

    gen.extend(quote!(
        #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
        pub properties: std::collections::BTreeMap<String,
    ));
    add_self(&mut gen);
    gen.extend(quote!(>,));

    gen.extend(quote!(
        #[serde(skip_serializing_if = "Option::is_none")]
        pub items: Option<
    ));
    add_self(&mut gen);
    gen.extend(quote!(>,));

    gen.extend(quote!(
        #[serde(default, rename = "enum", skip_serializing_if = "Vec::is_empty")]
        pub enum_: Vec<serde_json::Value>,
    ));

    gen.extend(quote!(
        #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
        pub extra_props: Option<paperclip::v2::models::Either<bool,
    ));
    add_self(&mut gen);
    gen.extend(quote!(>>,));

    gen.extend(quote!(
        #[serde(default, skip_serializing_if = "std::collections::BTreeSet::is_empty")]
        pub required: std::collections::BTreeSet<String>,
    ));

    if is_ref {
        gen.extend(quote!(
            #[serde(skip)]
            cyclic: bool,
        ));
    }

    quote!({
        #[doc(hidden)]
        #[serde(skip)]
        pub name: Option<String>,
        #gen
    })
}
