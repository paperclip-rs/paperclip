#[cfg(feature = "actix3-validator")]
extern crate actix_web_validator2 as actix_web_validator;
#[cfg(feature = "actix4-validator")]
extern crate actix_web_validator3 as actix_web_validator;

#[cfg(feature = "actix-multipart")]
use super::schema::TypedData;
use super::{
    models::{
        DefaultOperationRaw, DefaultSchemaRaw, Either, Items, Parameter, ParameterIn, Response,
        SecurityScheme,
    },
    schema::{Apiv2Errors, Apiv2Operation, Apiv2Schema},
};
#[cfg(not(feature = "actix4"))]
use crate::util::{ready, Ready};
#[cfg(any(feature = "actix3", feature = "actix4"))]
use actix_web::web::ReqData;
#[cfg(not(feature = "actix4"))]
use actix_web::Error;
#[cfg(feature = "actix4")]
use actix_web::{body::BoxBody, ResponseError};
use actix_web::{
    http::StatusCode,
    web::{Bytes, Data, Form, Json, Path, Payload, Query},
    HttpRequest, HttpResponse, Responder,
};

use pin_project::pin_project;

#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
use actix_web_validator::{
    Json as ValidatedJson, Path as ValidatedPath, QsQuery as ValidatedQsQuery,
    Query as ValidatedQuery,
};
use serde::Serialize;
#[cfg(feature = "serde_qs")]
use serde_qs::actix::QsQuery;

use std::{
    collections::BTreeMap,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Actix-specific trait for indicating that this entity can modify an operation
/// and/or update the global map of definitions.
pub trait OperationModifier: Apiv2Schema + Sized {
    /// Update the parameters list in the given operation (if needed).
    fn update_parameter(op: &mut DefaultOperationRaw) {
        update_parameter::<Self>(op);
    }

    /// Update the responses map in the given operation (if needed).
    fn update_response(_op: &mut DefaultOperationRaw) {}

    /// Update the definitions map (if needed).
    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        update_definitions_from_schema_type::<Self>(map);
    }

    /// Update the security map in the given operation (if needed).
    fn update_security(op: &mut DefaultOperationRaw) {
        update_security::<Self>(op);
    }

    /// Update the security definition map (if needed).
    fn update_security_definitions(map: &mut BTreeMap<String, SecurityScheme>) {
        update_security_definitions::<Self>(map);
    }
}

/// All schema types default to updating the definitions map.
#[cfg(feature = "nightly")]
impl<T> OperationModifier for T
where
    T: Apiv2Schema,
{
    default fn update_parameter(op: &mut DefaultOperationRaw) {
        update_parameter::<Self>(op);
    }

    default fn update_response(_op: &mut DefaultOperationRaw) {}

    default fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        update_definitions_from_schema_type::<Self>(map);
    }

    default fn update_security(op: &mut DefaultOperationRaw) {
        update_security::<Self>(op);
    }

    default fn update_security_definitions(map: &mut BTreeMap<String, SecurityScheme>) {
        update_security_definitions::<Self>(map);
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

    fn update_security_definitions(map: &mut BTreeMap<String, SecurityScheme>) {
        T::update_security_definitions(map);
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

    default fn update_security_definitions(map: &mut BTreeMap<String, SecurityScheme>) {
        T::update_security_definitions(map);
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
        E::update_error_definitions(op);
    }

    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        T::update_definitions(map);
        E::update_definitions(map);
    }

    fn update_security_definitions(map: &mut BTreeMap<String, SecurityScheme>) {
        T::update_security_definitions(map);
    }
}

// We don't know what we should do with these abstractions
// as they could be anything.
impl<T> Apiv2Schema for Data<T> {}
#[cfg(not(feature = "nightly"))]
impl<T> OperationModifier for Data<T> {}
#[cfg(any(feature = "actix3", feature = "actix4"))]
impl<T: std::clone::Clone> Apiv2Schema for ReqData<T> {}
#[cfg(not(feature = "nightly"))]
#[cfg(any(feature = "actix3", feature = "actix4"))]
impl<T: std::clone::Clone> OperationModifier for ReqData<T> {}

macro_rules! impl_empty({ $($ty:ty),+ } => {
    $(
        impl Apiv2Schema for $ty {}
        #[cfg(not(feature = "nightly"))]
        impl OperationModifier for $ty {}
    )+
});

#[cfg(feature = "actix4")]
/// Workaround for possibility to directly return HttpResponse from closure handler.
///
/// This is needed after actix removed `impl Future` from `HttpResponse`:
/// <https://github.com/actix/actix-web/pull/2601>
///
/// Example:
//////
/// ```ignore
/// .route(web::get().to(||
///     async move {
///         paperclip::actix::HttpResponseWrapper(
///             HttpResponse::Ok().body("Hi there!")
///         )
///     }
/// ))
/// ```
pub struct HttpResponseWrapper(pub HttpResponse);

#[cfg(feature = "actix4")]
impl Responder for HttpResponseWrapper {
    type Body = <HttpResponse as Responder>::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.0.respond_to(req)
    }
}

#[cfg(feature = "actix4")]
impl<F> Apiv2Operation for F
where
    F: Future<Output = HttpResponseWrapper>,
{
    fn operation() -> DefaultOperationRaw {
        Default::default()
    }

    fn security_definitions() -> BTreeMap<String, SecurityScheme> {
        Default::default()
    }

    fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
        Default::default()
    }
}

#[cfg(not(feature = "actix4"))]
impl Apiv2Operation for HttpResponse {
    fn operation() -> DefaultOperationRaw {
        Default::default()
    }

    fn security_definitions() -> BTreeMap<String, SecurityScheme> {
        Default::default()
    }

    fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
        Default::default()
    }
}

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
    #[cfg(feature = "chrono")]
    impl_simple!(chrono::NaiveDateTime);
    #[cfg(feature = "rust_decimal")]
    impl_simple!(rust_decimal::Decimal);
    #[cfg(feature = "url")]
    impl_simple!(url::Url);
    #[cfg(feature = "uuid")]
    impl_simple!(uuid::Uuid);
}

#[cfg(feature = "chrono")]
impl<T: chrono::TimeZone> OperationModifier for chrono::DateTime<T> {}

// Other extractors

#[cfg(feature = "nightly")]
impl<T> Apiv2Schema for Json<T> {
    default fn name() -> Option<String> {
        None
    }

    default fn raw_schema() -> DefaultSchemaRaw {
        Default::default()
    }
}

/// JSON needs specialization because it updates the global definitions.
impl<T: Apiv2Schema> Apiv2Schema for Json<T> {
    fn name() -> Option<String> {
        T::name()
    }

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

#[cfg(all(
    any(feature = "actix4-validator", feature = "actix3-validator"),
    feature = "nightly"
))]
impl<T> Apiv2Schema for ValidatedJson<T> {
    fn name() -> Option<String> {
        None
    }

    default fn raw_schema() -> DefaultSchemaRaw {
        Default::default()
    }
}

#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
impl<T: Apiv2Schema> Apiv2Schema for ValidatedJson<T> {
    fn name() -> Option<String> {
        T::name()
    }

    fn raw_schema() -> DefaultSchemaRaw {
        T::raw_schema()
    }
}

#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
impl<T> OperationModifier for ValidatedJson<T>
where
    T: Apiv2Schema,
{
    fn update_parameter(op: &mut DefaultOperationRaw) {
        Json::<T>::update_parameter(op);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        Json::<T>::update_response(op);
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

#[cfg(feature = "actix-session")]
impl OperationModifier for actix_session::Session {
    fn update_definitions(_map: &mut BTreeMap<String, DefaultSchemaRaw>) {}
}

#[cfg(feature = "actix-files")]
impl OperationModifier for actix_files::NamedFile {
    fn update_definitions(_map: &mut BTreeMap<String, DefaultSchemaRaw>) {}
}

macro_rules! impl_param_extractor ({ $ty:ty => $container:ident } => {
    #[cfg(feature = "nightly")]
    impl<T> Apiv2Schema for $ty {
        default fn name() -> Option<String> {
            None
        }

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
                    description: def.description,
                    ..Default::default()
                }));
            }
            for (k, v) in def.properties {
                op.parameters.push(Either::Right(Parameter {
                    in_: ParameterIn::$container,
                    required: def.required.contains(&k),
                    data_type: v.data_type,
                    format: v.format,
                    enum_: v.enum_,
                    description: v.description,
                    collection_format: None, // this defaults to csv
                    items: v.items.as_deref().map(map_schema_to_items),
                    name: k,
                    ..Default::default()
                }));
            }
        }

        // These don't require updating definitions, as we use them only
        // to get their properties.
        fn update_definitions(_map: &mut BTreeMap<String, DefaultSchemaRaw>) {}
    }
});

fn map_schema_to_items(schema: &DefaultSchemaRaw) -> Items {
    Items {
        data_type: schema.data_type,
        format: schema.format.clone(),
        collection_format: None, // this defaults to csv
        enum_: schema.enum_.clone(),
        items: schema
            .items
            .as_deref()
            .map(|schema| Box::new(map_schema_to_items(schema))),
        ..Default::default() // range fields are not emitted
    }
}

/// `formData` can refer to the global definitions.
#[cfg(feature = "nightly")]
impl<T: Apiv2Schema> Apiv2Schema for Form<T> {
    fn name() -> Option<String> {
        T::name()
    }

    fn raw_schema() -> DefaultSchemaRaw {
        T::raw_schema()
    }
}

impl_param_extractor!(Path<T> => Path);
impl_param_extractor!(Query<T> => Query);
impl_param_extractor!(Form<T> => FormData);
#[cfg(feature = "serde_qs")]
impl_param_extractor!(QsQuery<T> => Query);
#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
impl_param_extractor!(ValidatedPath<T> => Path);
#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
impl_param_extractor!(ValidatedQuery<T> => Query);
#[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
impl_param_extractor!(ValidatedQsQuery<T> => Query);

macro_rules! impl_path_tuple ({ $($ty:ident),+ } => {
    #[cfg(all(any(feature = "actix4-validator", feature = "actix3-validator"), feature = "nightly"))]
    impl<$($ty,)+> Apiv2Schema for ValidatedPath<($($ty,)+)> {}

    #[cfg(all(not(feature = "nightly"), any(feature = "actix4-validator", feature = "actix3-validator")))]
    impl<$($ty: Apiv2Schema,)+> Apiv2Schema for ValidatedPath<($($ty,)+)> {}

    #[cfg(any(feature = "actix4-validator", feature = "actix3-validator"))]
    impl<$($ty,)+> OperationModifier for ValidatedPath<($($ty,)+)>
        where $($ty: Apiv2Schema,)+
    {
        fn update_parameter(op: &mut DefaultOperationRaw) {
            $(
                Path::<$ty>::update_parameter(op);
            )+
        }
    }

    #[cfg(feature = "nightly")]
    impl<$($ty,)+> Apiv2Schema for Path<($($ty,)+)> {}

    #[cfg(not(feature = "nightly"))]
    impl<$($ty: Apiv2Schema,)+> Apiv2Schema for Path<($($ty,)+)> {}

    impl<$($ty,)+> OperationModifier for Path<($($ty,)+)>
        where $($ty: Apiv2Schema,)+
    {
        fn update_parameter(op: &mut DefaultOperationRaw) {
            $(
                let def = $ty::raw_schema();
                if def.properties.is_empty() {
                    op.parameters.push(Either::Right(Parameter {
                        // NOTE: We're setting empty name, because we don't know
                        // the name in this context. We'll get it when we add services.
                        name: String::new(),
                        in_: ParameterIn::Path,
                        required: true,
                        data_type: def.data_type,
                        format: def.format,
                        enum_: def.enum_,
                        description: def.description,
                        ..Default::default()
                    }));
                }
                for (k, v) in def.properties {
                    op.parameters.push(Either::Right(Parameter {
                        in_: ParameterIn::Path,
                        required: def.required.contains(&k),
                        data_type: v.data_type,
                        format: v.format,
                        enum_: v.enum_,
                        description: v.description,
                        collection_format: None, // this defaults to csv
                        items: v.items.as_deref().map(map_schema_to_items),
                        name: k,
                        ..Default::default()
                    }));
                }
            )+
        }
    }
});

impl_path_tuple!(A);
impl_path_tuple!(A, B);
impl_path_tuple!(A, B, C);
impl_path_tuple!(A, B, C, D);
impl_path_tuple!(A, B, C, D, E);
impl_path_tuple!(A, B, C, D, E, F);
impl_path_tuple!(A, B, C, D, E, F, G);
impl_path_tuple!(A, B, C, D, E, F, G, H);
impl_path_tuple!(A, B, C, D, E, F, G, H, I);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

/// Wrapper for wrapping over `impl Responder` thingies (to avoid breakage).
pub struct ResponderWrapper<T>(pub T);

#[cfg(feature = "nightly")]
impl<T: Responder> Apiv2Schema for ResponderWrapper<T> {
    default fn name() -> Option<String> {
        None
    }

    default fn raw_schema() -> DefaultSchemaRaw {
        DefaultSchemaRaw::default()
    }
}

#[cfg(not(feature = "nightly"))]
impl<T: Responder> Apiv2Schema for ResponderWrapper<T> {}

#[cfg(not(feature = "nightly"))]
impl<T: Responder> OperationModifier for ResponderWrapper<T> {}

#[cfg(feature = "actix4")]
impl Apiv2Schema for actix_web::dev::Response<actix_web::body::BoxBody> {}

#[cfg(feature = "actix4")]
impl OperationModifier for actix_web::dev::Response<actix_web::body::BoxBody> {}

#[cfg(feature = "actix4")]
impl<T: Responder> Responder for ResponderWrapper<T> {
    type Body = T::Body;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.0.respond_to(req)
    }
}

#[cfg(not(feature = "actix4"))]
impl<T: Responder> Responder for ResponderWrapper<T> {
    type Error = T::Error;
    type Future = T::Future;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        self.0.respond_to(req)
    }
}

/// Wrapper for all response types from handlers. This holds the actual value
/// returned by the handler and a unit struct (autogenerated by the plugin) which
/// is used for generating operation information.
#[pin_project]
pub struct ResponseWrapper<T, H>(#[pin] pub T, pub H);

#[cfg(feature = "actix4")]
impl<T: Responder, H> Responder for ResponseWrapper<T, H> {
    type Body = T::Body;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        self.0.respond_to(req)
    }
}

#[cfg(not(feature = "actix4"))]
impl<T: Responder, H> Responder for ResponseWrapper<T, H> {
    type Error = <T as Responder>::Error;
    type Future = <T as Responder>::Future;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        self.0.respond_to(req)
    }
}

impl<F, T, H> Future for ResponseWrapper<F, H>
where
    F: Future<Output = T>,
    T: OperationModifier + Responder,
    H: Apiv2Operation,
{
    type Output = T;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut().project();
        this.0.poll(ctx)
    }
}

impl<F, T, H> Apiv2Operation for ResponseWrapper<F, H>
where
    F: Future<Output = T>,
    T: OperationModifier + Responder,
    H: Apiv2Operation,
{
    fn operation() -> DefaultOperationRaw {
        H::operation()
    }

    fn security_definitions() -> BTreeMap<String, SecurityScheme> {
        H::security_definitions()
    }

    fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
        H::definitions()
    }

    fn is_visible() -> bool {
        H::is_visible()
    }
}

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

fn update_parameter<T>(op: &mut DefaultOperationRaw)
where
    T: Apiv2Schema,
{
    for parameter in T::header_parameter_schema() {
        op.parameters.push(Either::Right(parameter))
    }
}

/// Add security requirements to operation.
fn update_security<T>(op: &mut DefaultOperationRaw)
where
    T: Apiv2Schema,
{
    if let (Some(name), Some(scheme)) = (T::name(), T::security_scheme()) {
        let mut security_map = BTreeMap::new();
        let scopes = scheme.scopes.keys().map(String::clone).collect();
        security_map.insert(name, scopes);
        op.security.push(security_map);
    }
}

/// Merge security scheme into existing security definitions or add new.
fn update_security_definitions<T>(map: &mut BTreeMap<String, SecurityScheme>)
where
    T: Apiv2Schema,
{
    if let (Some(name), Some(new)) = (T::name(), T::security_scheme()) {
        new.update_definitions(&name, map);
    }
}

macro_rules! json_with_status {
    ($name:ident => $status:expr) => {
        pub struct $name<T: Serialize + Apiv2Schema>(pub T);

        impl<T> fmt::Debug for $name<T>
        where
            T: fmt::Debug + Serialize + Apiv2Schema,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let status: StatusCode = $status;
                let status_str = status.canonical_reason().unwrap_or(status.as_str());
                write!(f, "{} Json: {:?}", status_str, self.0)
            }
        }

        impl<T> fmt::Display for $name<T>
        where
            T: fmt::Display + Serialize + Apiv2Schema,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        #[cfg(feature = "actix4")]
        impl<T> Responder for $name<T>
        where
            T: Serialize + Apiv2Schema,
        {
            type Body = BoxBody;

            fn respond_to(self, _: &HttpRequest) -> HttpResponse<BoxBody> {
                let status: StatusCode = $status;
                let body = match serde_json::to_string(&self.0) {
                    Ok(body) => body,
                    Err(e) => return e.error_response(),
                };

                HttpResponse::build(status)
                    .content_type("application/json")
                    .body(body)
            }
        }

        #[cfg(not(feature = "actix4"))]
        impl<T> Responder for $name<T>
        where
            T: Serialize + Apiv2Schema,
        {
            type Error = Error;
            type Future = Ready<Result<HttpResponse, Error>>;

            fn respond_to(self, _: &HttpRequest) -> Self::Future {
                let status: StatusCode = $status;
                let body = match serde_json::to_string(&self.0) {
                    Ok(body) => body,
                    Err(e) => return ready(Err(e.into())),
                };

                ready(Ok(HttpResponse::build(status)
                    .content_type("application/json")
                    .body(body)))
            }
        }

        impl<T> Apiv2Schema for $name<T>
        where
            T: Serialize + Apiv2Schema,
        {
            fn name() -> Option<String> {
                T::name()
            }

            fn raw_schema() -> DefaultSchemaRaw {
                T::raw_schema()
            }
        }

        impl<T> OperationModifier for $name<T>
        where
            T: Serialize + Apiv2Schema,
        {
            fn update_response(op: &mut DefaultOperationRaw) {
                let status: StatusCode = $status;
                op.responses.insert(
                    status.as_str().into(),
                    Either::Right(Response {
                        description: status.canonical_reason().map(ToString::to_string),
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
    };
}

json_with_status!(CreatedJson => StatusCode::CREATED);
json_with_status!(AcceptedJson => StatusCode::ACCEPTED);

#[derive(Debug)]
pub struct NoContent;

impl fmt::Display for NoContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No Content")
    }
}

#[cfg(feature = "actix4")]
impl Responder for NoContent {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<BoxBody> {
        HttpResponse::build(StatusCode::NO_CONTENT)
            .content_type("application/json")
            .finish()
    }
}

#[cfg(not(feature = "actix4"))]
impl Responder for NoContent {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::build(StatusCode::NO_CONTENT)
            .content_type("application/json")
            .finish()))
    }
}

impl Apiv2Schema for NoContent {}

impl OperationModifier for NoContent {
    fn update_response(op: &mut DefaultOperationRaw) {
        let status = StatusCode::NO_CONTENT;
        op.responses.insert(
            status.as_str().into(),
            Either::Right(Response {
                description: status.canonical_reason().map(ToString::to_string),
                schema: None,
                ..Default::default()
            }),
        );
    }
}
