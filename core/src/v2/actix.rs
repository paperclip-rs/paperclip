use super::models::{
    DefaultOperationRaw, DefaultResponseRaw, DefaultSchemaRaw, Either, Parameter, ParameterIn,
    Response, SecurityScheme,
};
#[cfg(feature = "actix-multipart")]
use super::schema::TypedData;
use super::schema::{Apiv2Errors, Apiv2Operation, Apiv2Schema};
use actix_web::{
    web::{Bytes, Data, Form, Json, Path, Payload, Query},
    HttpRequest, HttpResponse, Responder,
};

use std::collections::BTreeMap;
use std::future::Future;

/// Actix-specific trait for indicating that this entity can modify an operation
/// and/or update the global map of definitions.
pub trait OperationModifier: Apiv2Schema + Sized {
    /// Update the parameters list in the given operation (if needed).
    fn update_parameter(_op: &mut DefaultOperationRaw) {}

    /// Update the responses map in the given operation (if needed).
    fn update_response(_op: &mut DefaultOperationRaw) {}

    /// Update the definitions map (if needed).
    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        update_definitions_from_schema_type::<Self>(map);
    }

    /// Update the security map in the given operation (if needed).
    fn update_security(op: &mut DefaultOperationRaw) {
        update_security(op, Self::security_schema());
    }
}

/// All schema types default to updating the definitions map.
#[cfg(feature = "nightly")]
impl<T> OperationModifier for T
where
    T: Apiv2Schema,
{
    default fn update_parameter(_op: &mut DefaultOperationRaw) {}

    default fn update_response(_op: &mut DefaultOperationRaw) {}

    default fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        update_definitions_from_schema_type::<Self>(map);
    }

    /// Update the security map in the given operation (if needed).
    default fn update_security(op: &mut DefaultOperationRaw) {
        update_security(op, Self::security_schema());
    }
}

impl<T> OperationModifier for Option<T>
where
    T: OperationModifier,
{
    fn update_parameter(op: &mut DefaultOperationRaw) {
        T::update_parameter(op);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        T::update_response(op);
    }

    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        T::update_definitions(map);
    }

    fn update_security(op: &mut DefaultOperationRaw) {
        T::update_security(op);
    }
}

#[cfg(feature = "nightly")]
impl<T, E> OperationModifier for Result<T, E>
where
    T: OperationModifier,
{
    default fn update_parameter(op: &mut DefaultOperationRaw) {
        T::update_parameter(op);
    }

    default fn update_response(op: &mut DefaultOperationRaw) {
        T::update_response(op);
    }

    default fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        T::update_definitions(map);
    }
}

impl<T, E> OperationModifier for Result<T, E>
where
    T: OperationModifier,
    E: Apiv2Errors,
{
    fn update_parameter(op: &mut DefaultOperationRaw) {
        T::update_parameter(op);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        T::update_response(op);
        update_error_definitions_from_schema_type::<E>(op);
    }

    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        T::update_definitions(map);
    }
}

// We don't know what we should do with these abstractions
// as they could be anything.
impl<T> Apiv2Schema for Data<T> {}
#[cfg(not(feature = "nightly"))]
impl<T> OperationModifier for Data<T> {}

macro_rules! impl_empty({ $($ty:ty),+ } => {
    $(
        impl Apiv2Schema for $ty {}
        #[cfg(not(feature = "nightly"))]
        impl OperationModifier for $ty {}
    )+
});

impl_empty!(HttpRequest, HttpResponse, Bytes, Payload);

#[cfg(not(feature = "nightly"))]
mod manual_impl {
    use super::OperationModifier;

    impl<'a> OperationModifier for &'a str {}
    impl<'a, T: OperationModifier> OperationModifier for &'a [T] {}

    macro_rules! impl_simple({ $ty:ty } => {
        impl OperationModifier for $ty {}
    });

    impl_simple!(char);
    impl_simple!(String);
    impl_simple!(bool);
    impl_simple!(f32);
    impl_simple!(f64);
    impl_simple!(i8);
    impl_simple!(i16);
    impl_simple!(i32);
    impl_simple!(u8);
    impl_simple!(u16);
    impl_simple!(u32);
    impl_simple!(i64);
    impl_simple!(i128);
    impl_simple!(isize);
    impl_simple!(u64);
    impl_simple!(u128);
    impl_simple!(usize);
    #[cfg(feature = "datetime")]
    impl_simple!(chrono::NaiveDateTime);
    #[cfg(feature = "decimal")]
    impl_simple!(rust_decimal::Decimal);
    #[cfg(feature = "uid")]
    impl_simple!(uuid::Uuid);
}

#[cfg(feature = "datetime")]
impl<T: chrono::TimeZone> OperationModifier for chrono::DateTime<T> {}

// Other extractors

#[cfg(feature = "nightly")]
impl<T> Apiv2Schema for Json<T> {
    default const NAME: Option<&'static str> = None;

    default fn raw_schema() -> DefaultSchemaRaw {
        Default::default()
    }
}

/// JSON needs specialization because it updates the global definitions.
impl<T: Apiv2Schema> Apiv2Schema for Json<T> {
    const NAME: Option<&'static str> = T::NAME;

    fn raw_schema() -> DefaultSchemaRaw {
        T::raw_schema()
    }
}

impl<T> OperationModifier for Json<T>
where
    T: Apiv2Schema,
{
    fn update_parameter(op: &mut DefaultOperationRaw) {
        op.parameters.push(Either::Right(Parameter {
            description: None,
            in_: ParameterIn::Body,
            name: "body".into(),
            required: true,
            schema: Some({
                let mut def = T::schema_with_ref();
                def.retain_ref();
                def
            }),
            ..Default::default()
        }));
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        op.responses.insert(
            "200".into(),
            Either::Right(Response {
                // TODO: Support configuring other 2xx codes using macro attribute.
                description: Some("OK".into()),
                schema: Some({
                    let mut def = T::schema_with_ref();
                    def.retain_ref();
                    def
                }),
                ..Default::default()
            }),
        );
    }
}

#[cfg(feature = "actix-multipart")]
impl OperationModifier for actix_multipart::Multipart {
    fn update_parameter(op: &mut DefaultOperationRaw) {
        op.parameters.push(Either::Right(Parameter {
            description: None,
            in_: ParameterIn::FormData,
            name: "file_data".into(),
            required: true,
            data_type: Some(<actix_multipart::Multipart as TypedData>::data_type()),
            format: <actix_multipart::Multipart as TypedData>::format(),
            ..Default::default()
        }));
    }
}

macro_rules! impl_param_extractor ({ $ty:ty => $container:ident } => {
    #[cfg(feature = "nightly")]
    impl<T> Apiv2Schema for $ty {
        default const NAME: Option<&'static str> = None;

        default fn raw_schema() -> DefaultSchemaRaw {
            Default::default()
        }
    }

    #[cfg(not(feature = "nightly"))]
    impl<T: Apiv2Schema> Apiv2Schema for $ty {}

    impl<T: Apiv2Schema> OperationModifier for $ty {
        fn update_parameter(op: &mut DefaultOperationRaw) {
            let def = T::raw_schema();
            // If there aren't any properties and if it's a path parameter,
            // then add a parameter whose name will be overridden later.
            if def.properties.is_empty() && ParameterIn::$container == ParameterIn::Path {
                op.parameters.push(Either::Right(Parameter {
                    name: String::new(),
                    in_: ParameterIn::Path,
                    required: true,
                    data_type: def.data_type,
                    format: def.format,
                    enum_: def.enum_,
                    ..Default::default()
                }));
            }

            for (k, v) in def.properties {
                op.parameters.push(Either::Right(Parameter {
                    in_: ParameterIn::$container,
                    required: def.required.contains(&k),
                    name: k,
                    data_type: v.data_type,
                    format: v.format,
                    enum_: v.enum_,
                    ..Default::default()
                }));
            }
        }

        // These don't require updating definitions, as we use them only
        // to get their properties.
        fn update_definitions(_map: &mut BTreeMap<String, DefaultSchemaRaw>) {}
    }
});

/// `formData` can refer to the global definitions.
#[cfg(feature = "nightly")]
impl<T: Apiv2Schema> Apiv2Schema for Form<T> {
    const NAME: Option<&'static str> = T::NAME;

    fn raw_schema() -> DefaultSchemaRaw {
        T::raw_schema()
    }
}

impl_param_extractor!(Path<T> => Path);
impl_param_extractor!(Query<T> => Query);
impl_param_extractor!(Form<T> => FormData);

macro_rules! impl_path_tuple ({ $($ty:ident),+ } => {
    #[cfg(feature = "nightly")]
    impl<$($ty,)+> Apiv2Schema for Path<($($ty,)+)> {}

    #[cfg(not(feature = "nightly"))]
    impl<$($ty: Apiv2Schema,)+> Apiv2Schema for Path<($($ty,)+)> {}

    impl<$($ty,)+> OperationModifier for Path<($($ty,)+)>
        where $($ty: Apiv2Schema,)+
    {
        fn update_parameter(op: &mut DefaultOperationRaw) {
            // NOTE: We're setting empty name, because we don't know
            // the name in this context. We'll get it when we add services.
            $(
                let def = $ty::raw_schema();
                op.parameters.push(Either::Right(Parameter {
                    name: String::new(),
                    in_: ParameterIn::Path,
                    required: true,
                    data_type: def.data_type,
                    format: def.format,
                    enum_: def.enum_,
                    ..Default::default()
                }));
            )+
        }
    }
});

impl_path_tuple!(A);
impl_path_tuple!(A, B);
impl_path_tuple!(A, B, C);
impl_path_tuple!(A, B, C, D);
impl_path_tuple!(A, B, C, D, E);

// Our goal is to implement `OperationModifier` for everything. The problem
// is that we can't specialize these impls for foreign trait implementors
// because we're already specializing it for actix types (which are also foreign).
// So, rustc will complain when it finds that a foreign type *may* implement
// a foreign trait in the future, which could then introduce a conflict in
// our `OperationModifier` impl. One solution would be to use trait-specific
// wrappers which can then be added to the actual code using proc macros later.

/// Wrapper for wrapping over `impl Responder` thingies (to avoid breakage).
pub struct ResponderWrapper<T>(pub T);

#[cfg(feature = "nightly")]
impl<T: Responder> Apiv2Schema for ResponderWrapper<T> {
    default const NAME: Option<&'static str> = None;

    default fn raw_schema() -> DefaultSchemaRaw {
        DefaultSchemaRaw::default()
    }
}

#[cfg(not(feature = "nightly"))]
impl<T: Responder> Apiv2Schema for ResponderWrapper<T> {}

#[cfg(not(feature = "nightly"))]
impl<T: Responder> OperationModifier for ResponderWrapper<T> {}

impl<T: Responder> Responder for ResponderWrapper<T> {
    type Error = T::Error;
    type Future = T::Future;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        self.0.respond_to(req)
    }
}

macro_rules! impl_fn_operation ({ $($ty:ident),* } => {
    impl<U, $($ty,)* R, RV> Apiv2Operation<($($ty,)*), RV> for U
        where U: Fn($($ty,)*) -> R,
              $($ty: OperationModifier,)*
              R: Future<Output=RV>,
              RV: OperationModifier,
    {
        fn operation() -> DefaultOperationRaw {
            let mut op = DefaultOperationRaw::default();
            $(
                $ty::update_parameter(&mut op);
                $ty::update_security(&mut op);
            )*
            RV::update_response(&mut op);
            op
        }

        fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
            let mut map = BTreeMap::new();
            $(
                $ty::update_definitions(&mut map);
            )*
            RV::update_definitions(&mut map);
            map
        }
    }
});

impl_fn_operation!();
impl_fn_operation!(A);
impl_fn_operation!(A, B);
impl_fn_operation!(A, B, C);
impl_fn_operation!(A, B, C, D);
impl_fn_operation!(A, B, C, D, E);
impl_fn_operation!(A, B, C, D, E, F);
impl_fn_operation!(A, B, C, D, E, F, G);
impl_fn_operation!(A, B, C, D, E, F, G, H);
impl_fn_operation!(A, B, C, D, E, F, G, H, I);
impl_fn_operation!(A, B, C, D, E, F, G, H, I, J);

/// Given the schema type, recursively update the map of definitions.
fn update_definitions_from_schema_type<T>(map: &mut BTreeMap<String, DefaultSchemaRaw>)
where
    T: Apiv2Schema,
{
    let mut schema = T::schema_with_ref();
    loop {
        if let Some(s) = schema.items {
            schema = *s;
            continue;
        } else if let Some(Either::Right(s)) = schema.extra_props {
            schema = *s;
            continue;
        } else if let Some(n) = schema.name.take() {
            schema.remove_refs();
            map.insert(n, schema);
        }

        break;
    }
}

/// Given a schema type that represents an error, add the responses
/// representing those errors.
fn update_error_definitions_from_schema_type<T>(op: &mut DefaultOperationRaw)
where
    T: Apiv2Errors,
{
    for (status, def_name) in T::ERROR_MAP {
        let response = DefaultResponseRaw {
            description: Some((*def_name).to_string()),
            ..Default::default()
        };
        op.responses
            .insert(status.to_string(), Either::Right(response));
    }
}

/// Add security definitions to operation.
fn update_security(op: &mut DefaultOperationRaw, security_def: Option<(String, SecurityScheme)>) {
    if let Some(security_def) = &security_def {
        let mut security_roles = Vec::new();
        for role in security_def.1.scopes.keys() {
            security_roles.push(role.clone());
        }

        let mut security_def_map = BTreeMap::new();
        security_def_map.insert(security_def.0.to_string(), security_roles);

        op.security.push(security_def_map);
        op.security_definitions
            .insert(security_def.0.to_string(), security_def.1.clone());
    }
}
