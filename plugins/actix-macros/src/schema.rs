use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, GenericArgument, ItemFn, PathArguments, ReturnType, Type};

pub fn infer_operation_definition(f: &ItemFn) -> Result<proc_macro2::TokenStream, TokenStream> {
    let mut gen = quote!(
        use paperclip_actix::Apiv2Schema;
        use paperclip::v2::models::*;

        let mut op = Operation::default();
    );

    for arg in &f.decl.inputs {
        if let FnArg::Captured(ref cap) = &arg {
            if let Some((_, ty)) = Container::matches(&cap.ty) {
                gen.extend(quote!(
                    op.parameters = Some(vec![Parameter {
                        description: None,
                        in_: ParameterIn::Body,
                        name: "body".into(),
                        required: true,
                        schema: Some(#ty::schema()),
                        data_type: None,
                        format: None,
                        items: None,
                    }]);
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

    if let Some((_, _)) = Container::matches(ret) {
        gen.extend(quote!());
    }

    gen.extend(quote!(op));

    Ok(gen)
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
