use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

/// Factory struct for producing operation definitions.
pub struct OperationProducer<'a> {
    f: &'a ItemFn,
    body_schema: Option<Type>,
    defs: proc_macro2::TokenStream,
    stream: proc_macro2::TokenStream,
}

impl<'a> From<&'a ItemFn> for OperationProducer<'a> {
    fn from(f: &'a ItemFn) -> Self {
        OperationProducer {
            f,
            body_schema: None,
            defs: quote!(),
            stream: quote!(),
        }
    }
}

impl<'a> OperationProducer<'a> {
    /// Attempts to come up with `Apiv2Operation` impl based on the given function definition.
    pub fn generate_definition(mut self) -> Result<proc_macro2::TokenStream, TokenStream> {
        for arg in &self.f.decl.inputs {
            if let FnArg::Captured(ref cap) = &arg {
                self.add_param_from_input_arg(&cap.ty);
            }
        }

        let ret = match &self.f.decl.output {
            ReturnType::Default => {
                return Err(crate::call_site_error_with_msg(
                    "function must return something",
                ))
            }
            ReturnType::Type(_, ref ty) => ty,
        };

        if let Some((c, ty)) = Container::matches(ret) {
            self.add_def_from_ty(&ty);
            if c.is_format() {
                self.stream.extend(quote!(
                    op.responses.insert("200".into(), Response {
                        description: None,
                        schema: Some({
                            let mut def = #ty::schema_with_ref();
                            def.retain_ref();
                            def
                        }),
                    });
                ));
            }
        }

        if let Some(ty) = self.body_schema.take() {
            self.add_def_from_ty(&ty);
        }

        let (gen, def) = (&self.stream, &self.defs);
        Ok(quote!(
            fn operation() -> paperclip::v2::models::Operation<paperclip::v2::models::DefaultSchemaRaw> {
                use paperclip::v2::{models::*, schema::{Apiv2Schema, TypedData}};

                let mut op = Operation::default();
                #gen
                op
            }

            fn definitions() -> std::collections::BTreeMap<String, paperclip::v2::models::DefaultSchemaRaw> {
                use paperclip::v2::{models::*, schema::{Apiv2Schema, TypedData}};

                let mut map = std::collections::BTreeMap::new();
                #def
                map
            }
        ))
    }

    /// Given a type, add code to get schema at runtime.
    fn add_def_from_ty(&mut self, ty: &Type) {
        self.defs.extend(quote!(
            let mut schema = #ty::schema_with_ref();
            loop {
                if let Some(s) = schema.items {
                    schema = *s;
                    continue
                } else if let Some(s) = schema.extra_props {
                    schema = *s;
                    continue
                } else if let Some(n) = schema.name.take() {
                    schema.remove_refs();
                    map.insert(n, schema);
                }

                break
            }
        ));
    }

    /// Checks arg type and updates the token stream as required.
    fn add_param_from_input_arg(&mut self, ty: &'a Type) {
        let (c, ty) = match Container::matches(ty) {
            Some(m) => m,
            None => return,
        };

        if c.is_format() && self.body_schema.is_none() {
            self.stream.extend(quote!(
                op.parameters.push(Parameter {
                    description: None,
                    in_: ParameterIn::Body,
                    name: "body".into(),
                    required: true,
                    schema: Some({
                        let mut def = #ty::schema_with_ref();
                        def.retain_ref();
                        def
                    }),
                    data_type: None,
                    format: None,
                    items: None,
                });
            ));
            self.body_schema = Some(ty);
        } else if c.is_extractor() {
            if let Type::Path(ref p) = ty {
                if let Some(seg) = p.path.segments.last() {
                    let inner = &seg.value().ident;
                    let (p_in, required) = match c {
                        Container::Path => (quote!(ParameterIn::Path), quote!(true)),
                        Container::Query => (
                            quote!(ParameterIn::Query),
                            quote!(def.required.contains(&k)),
                        ),
                        // FIXME: body and formData parameters are mutually exclusive.
                        Container::Form => (
                            quote!(ParameterIn::FormData),
                            quote!(def.required.contains(&k)),
                        ),
                        _ => unreachable!(),
                    };

                    self.stream.extend(quote!(
                        let def = #inner::raw_schema();
                        for (k, v) in def.properties {
                            op.parameters.push(Parameter {
                                description: None,
                                in_: #p_in,
                                required: #required,
                                name: k,
                                schema: None,
                                data_type: v.data_type,
                                format: v.format,
                                items: None,
                            });
                        }
                    ));
                }
            } else if let Type::Tuple(ref t) = ty {
                for ty in &t.elems {
                    // NOTE: We're setting empty name, because we don't know
                    // the name in this context. We'll get it when we add services.
                    self.stream.extend(quote!(
                        op.parameters.push(Parameter {
                            name: String::new(),
                            description: None,
                            in_: ParameterIn::Path,
                            required: true,
                            schema: None,
                            data_type: Some(#ty::data_type()),
                            format: #ty::format(),
                            items: None,
                        });
                    ));
                }
            }
        }
    }
}

/// We use this to ease enum definition, `AsRef<str>` impl and variants list (array).
macro_rules! str_enum {
    ($arr_name:ident > $name:ident: $($var:ident),+) => {
        #[derive(Copy, Clone, Debug)]
        enum $name {
            $($var),+
        }

        use self::$name::*;
        const $arr_name: &[$name] = &[$($var),+];

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match self {
                    $(
                        $var => stringify!($var)
                    ),+
                }
            }
        }
    };
}

str_enum! { SUPPORTED_CONTAINERS > Container:
    Json,
    Path,
    Query,
    Form
}

impl Container {
    /// Checks whether this is a data format.
    fn is_format(self) -> bool {
        match self {
            Container::Json => true,
            _ => false,
        }
    }

    /// Checks whether this is an extractor.
    fn is_extractor(self) -> bool {
        match self {
            Container::Path | Container::Query | Container::Form => true,
            _ => false,
        }
    }

    /// Checks whether this matches a container and returns it along with
    /// the contained type (if any).
    fn matches(ty: &Type) -> Option<(Container, Type)> {
        match ty {
            Type::Path(ref p) => p.path.segments.last().and_then(|seg| {
                let args = match &seg.value().arguments {
                    PathArguments::AngleBracketed(ref a) => a,
                    _ => return None,
                };

                if let Some(GenericArgument::Type(ref ty)) = args.args.iter().next() {
                    return SUPPORTED_CONTAINERS
                        .iter()
                        .find(|&c| seg.value().ident == c)
                        // Prefix that type with `::` if necessary.
                        .map(|c| (*c, crate::address_type_for_fn_call(ty)));
                }

                None
            }),
            _ => None,
        }
    }
}
