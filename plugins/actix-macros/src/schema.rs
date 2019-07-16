use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

/// Factory struct for producing operation definitions.
pub struct OperationProducer<'a> {
    f: &'a ItemFn,
    body_schema: Option<&'a Type>,
    stream: proc_macro2::TokenStream,
}

impl<'a> From<&'a ItemFn> for OperationProducer<'a> {
    fn from(f: &'a ItemFn) -> Self {
        OperationProducer {
            f,
            body_schema: None,
            stream: quote!(),
        }
    }
}

impl<'a> OperationProducer<'a> {
    /// Attempts to come up with `ApiOperation` impl based on the given function definition.
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

        match Container::matches(ret) {
            Some((c, ty)) if c.is_format() => {
                self.stream.extend(quote!(
                    op.responses.insert("200".into(), Response {
                        description: None,
                        schema: if let Some(n) = #ty::NAME {
                            let mut def = DefaultSchemaRaw::default();
                            def.reference = Some(String::from("#/definitions/") + n);
                            Some(def)
                        } else {
                            Some(#ty::schema())
                        },
                    });
                ));
            }
            _ => (),
        }

        let mut def = quote!();
        if let Some(ty) = self.body_schema {
            def.extend(quote!(
                if let Some(n) = #ty::NAME {
                    map.insert(n.into(), #ty::schema());
                }
            ));
        }

        let gen = &self.stream;
        Ok(quote!(
            fn operation() -> paperclip::v2::models::Operation<paperclip::v2::models::DefaultSchemaRaw> {
                use paperclip_actix::Apiv2Schema;
                use paperclip::v2::models::*;

                let mut op = Operation::default();
                #gen
                op
            }

            fn definitions() -> std::collections::BTreeMap<String, paperclip::v2::models::DefaultSchemaRaw> {
                use paperclip::v2::models::*;
                use paperclip_actix::Apiv2Schema;

                let mut map = std::collections::BTreeMap::new();
                #def
                map
            }
        ))
    }

    /// Checks arg type and updates the token stream as required.
    fn add_param_from_input_arg(&mut self, ty: &'a Type) {
        match Container::matches(ty) {
            Some((c, ty)) if c.is_format() && self.body_schema.is_none() => {
                self.body_schema = Some(ty);
                self.stream.extend(quote!(
                    op.parameters.push(Parameter {
                        description: None,
                        in_: ParameterIn::Body,
                        name: "body".into(),
                        required: true,
                        schema: if let Some(n) = #ty::NAME {
                            let mut def = DefaultSchemaRaw::default();
                            def.reference = Some(String::from("#/definitions/") + n);
                            Some(def)
                        } else {
                            Some(#ty::schema())
                        },
                        data_type: None,
                        format: None,
                        items: None,
                    });
                ));
            }
            Some((c, ty)) if c.is_extractor() => {
                if let Type::Path(ref p) = ty {
                    if let Some(seg) = p.path.segments.last() {
                        let inner = &seg.value().ident;
                        let (p_in, required) = match c {
                            Container::Path => (quote!(ParameterIn::Path), quote!(true)),
                            Container::Query => (
                                quote!(ParameterIn::Query),
                                quote!(def.required.contains(&k)),
                            ),
                            _ => unreachable!(),
                        };

                        self.stream.extend(quote!(
                            let def = #inner::schema();
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
            _ => (),
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
    Query
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
            Container::Path | Container::Query => true,
            _ => false,
        }
    }

    /// Checks whether this matches a container and returns it along with
    /// the contained type (if any).
    fn matches(ty: &Type) -> Option<(Container, &Type)> {
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
                        .map(|c| (*c, ty));
                }

                None
            }),
            _ => None,
        }
    }
}
