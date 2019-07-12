use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

/// Attempts to come up with `ApiOperation` impl based on the given function definition.
pub fn infer_operation_definition(f: &ItemFn) -> Result<proc_macro2::TokenStream, TokenStream> {
    let mut gen = quote!();
    let mut body_schema = None;

    for arg in &f.decl.inputs {
        if let FnArg::Captured(ref cap) = &arg {
            if let Some((_, ty)) = Container::matches(&cap.ty) {
                body_schema = Some(ty);
                gen.extend(quote!(
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

                break;
            }
        }
    }

    let ret = match &f.decl.output {
        ReturnType::Default => {
            return Err(crate::call_site_error_with_msg(
                "function must return something",
            ))
        }
        ReturnType::Type(_, ref ty) => ty,
    };

    if let Some((_, ty)) = Container::matches(ret) {
        gen.extend(quote!(
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

    let mut def = quote!();
    if let Some(ty) = body_schema {
        def.extend(quote!(
            if let Some(n) = #ty::NAME {
                map.insert(n.into(), #ty::schema());
            }
        ));
    }

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
                        $var => stringify!($var),
                    ),+
                }
            }
        }
    };
}

str_enum! { SUPPORTED_CONTAINERS > Container:
    Json
}

impl Container {
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
