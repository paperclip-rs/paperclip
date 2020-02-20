use super::models::{
    DefaultOperationRaw, DefaultSchemaRaw, Either, Parameter, ParameterIn, Response,
};
use super::schema::{Apiv2Operation, Apiv2Schema};
use actix_web::{
    web::{Bytes, Data, Form, Json, Path, Payload, Query},
    HttpRequest, HttpResponse, Responder,
};

use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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
}

/// All schema types default to updating the definitions map.
impl<T> OperationModifier for T
where
    T: Apiv2Schema,
{
    default fn update_parameter(_op: &mut DefaultOperationRaw) {}

    default fn update_response(_op: &mut DefaultOperationRaw) {}

    default fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        update_definitions_from_schema_type::<Self>(map);
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
}

impl<T, E> OperationModifier for Result<T, E>
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
}

// We don't know what we should do with these abstractions
// as they could be anything.
impl<T> Apiv2Schema for Data<T> {}

macro_rules! impl_empty({ $($ty:ty),+ } => {
    $(
        impl Apiv2Schema for $ty {}
    )+
});

impl_empty!(HttpRequest, HttpResponse, Bytes, Payload);

// Other extractors

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
                description: None,
                schema: Some({
                    let mut def = T::schema_with_ref();
                    def.retain_ref();
                    def
                }),
            }),
        );
    }
}

macro_rules! impl_param_extractor ({ $ty:ty => $container:ident } => {
    impl<T> Apiv2Schema for $ty {
        default const NAME: Option<&'static str> = None;

        default fn raw_schema() -> DefaultSchemaRaw {
            Default::default()
        }
    }

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
    }
});

/// `formData` can refer to the global definitions.
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
    impl<$($ty,)+> Apiv2Schema for Path<($($ty,)+)> {}

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

/// Wrapper for wrapping over `impl Future` thingies (to avoid breakage).
pub struct FutureWrapper<T>(pub T);

/// Wrapper for wrapping over `impl Responder` thingies (to avoid breakage).
pub struct ResponderWrapper<T>(pub T);

impl<T: Responder> Apiv2Schema for ResponderWrapper<T> {
    default const NAME: Option<&'static str> = None;

    default fn raw_schema() -> DefaultSchemaRaw {
        DefaultSchemaRaw::default()
    }
}

impl<T: Responder> Responder for ResponderWrapper<T> {
    type Error = T::Error;
    type Future = T::Future;

    #[inline]
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        self.0.respond_to(req)
    }
}

impl<I> Apiv2Schema for FutureWrapper<I>
where
    I: Future,
    I::Output: Apiv2Schema,
{
    const NAME: Option<&'static str> = I::Output::NAME;

    fn raw_schema() -> DefaultSchemaRaw {
        I::Output::raw_schema()
    }
}

impl<I> OperationModifier for FutureWrapper<I>
where
    I: Future,
    I::Output: OperationModifier,
{
    fn update_parameter(op: &mut DefaultOperationRaw) {
        I::Output::update_parameter(op);
    }

    fn update_response(op: &mut DefaultOperationRaw) {
        I::Output::update_response(op);
    }

    fn update_definitions(map: &mut BTreeMap<String, DefaultSchemaRaw>) {
        I::Output::update_definitions(map);
    }
}

impl<I> Future for FutureWrapper<I>
where
    I: Future + std::marker::Unpin
{
    type Output = I::Output;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Future::poll(Pin::new(&mut (*self).0), cx)
    }
}

macro_rules! impl_fn_operation ({ $($ty:ident),* } => {
    impl<U, $($ty,)* R, RU> Apiv2Operation<($($ty,)*), R, RU> for U
        where U: Fn($($ty,)*) -> R,
              $($ty: OperationModifier,)*
              R: OperationModifier,
    {
        default fn operation() -> DefaultOperationRaw {
            let mut op = DefaultOperationRaw::default();
            $(
                $ty::update_parameter(&mut op);
            )*
            R::update_response(&mut op);
            op
        }

        default fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
            let mut map = BTreeMap::new();
            $(
                $ty::update_definitions(&mut map);
            )*
            R::update_definitions(&mut map);
            map
        }
    }

    impl<U, $($ty,)* R, RU> Apiv2Operation<($($ty,)*), R, RU> for U
        where U: Fn($($ty,)*) -> R,
              $($ty: OperationModifier,)*
              R: OperationModifier + Future<Output=RU>,
              RU: OperationModifier,
    {
        fn operation() -> DefaultOperationRaw {
            let mut op = DefaultOperationRaw::default();
            $(
                $ty::update_parameter(&mut op);
            )*
            R::update_response(&mut op);
            op
        }

        fn definitions() -> BTreeMap<String, DefaultSchemaRaw> {
            let mut map = BTreeMap::new();
            $(
                $ty::update_definitions(&mut map);
            )*
            R::update_definitions(&mut map);
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
        if let Some(Either::Left(s)) = schema.items {
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
