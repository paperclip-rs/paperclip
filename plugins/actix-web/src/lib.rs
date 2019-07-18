pub mod web;

#[doc(inline)]
pub use self::web::{Resource, Route, Scope};
pub use paperclip_actix_macros::{api_v2_operation, api_v2_schema};

use actix_service::NewService;
use actix_web::dev::{HttpServiceFactory, MessageBody, ServiceRequest, ServiceResponse};
use actix_web::{web::HttpResponse, Error};
use paperclip::v2::models::{
    DataType, DefaultSchemaRaw, GenericApi, HttpMethod, Operation, OperationMap, TypedData,
};
use parking_lot::RwLock;

use std::collections::BTreeMap;
use std::sync::Arc;

/// Wrapper for [`actix_web::App`](https://docs.rs/actix-web/*/actix_web/struct.App.html).
pub struct App<T, B> {
    spec: Arc<RwLock<GenericApi<DefaultSchemaRaw>>>,
    inner: actix_web::App<T, B>,
}

/// Extension trait for actix-web applications.
pub trait OpenApiExt<T, B> {
    type Wrapper;

    /// Consumes this app and produces its wrapper.
    fn wrap_api(self) -> Self::Wrapper;
}

impl<T, B> OpenApiExt<T, B> for actix_web::App<T, B> {
    type Wrapper = App<T, B>;

    fn wrap_api(self) -> Self::Wrapper {
        App {
            spec: Arc::new(RwLock::new(GenericApi::default())),
            inner: self,
        }
    }
}

/// Indicates that this thingmabob has a path and a bunch of definitions and operations.
pub trait Mountable {
    /// Where this thing gets mounted.
    fn path(&self) -> &str;

    /// Map of HTTP methods and the associated API operations.
    fn operations(&mut self) -> BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>>;

    /// The definitions recorded by this object.
    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw>;

    /// Updates the given map of operations with operations tracked by this object.
    ///
    /// **NOTE:** Overriding implementations must ensure that the `OperationMap`
    /// is normalized before updating the input map.
    fn update_operations(&mut self, map: &mut BTreeMap<String, OperationMap<DefaultSchemaRaw>>) {
        let op_map = map
            .entry(self.path().into())
            .or_insert_with(OperationMap::default);
        op_map.methods.extend(self.operations().into_iter());
        op_map.normalize();
    }
}

/// Represents a OpenAPI v2 schema convertible. This is auto-implemented by
/// [`api_v2_schema`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_schema.html) macro.
///
/// This is implemented for primitive types by default.
pub trait Apiv2Schema {
    /// Name of this schema. This is the object's name.
    const NAME: Option<&'static str>;

    /// Returns the schema for this object.
    fn schema() -> DefaultSchemaRaw;
}

impl<T: TypedData> Apiv2Schema for T {
    const NAME: Option<&'static str> = None;

    fn schema() -> DefaultSchemaRaw {
        let mut schema = DefaultSchemaRaw::default();
        schema.data_type = Some(T::data_type());
        schema.format = T::format();

        if let DataType::Array = T::data_type() {
            schema.items = Some(T::schema().into());
        }

        schema
    }
}

/// Represents a OpenAPI v2 operation convertible. This is auto-implemented by
/// [`api_v2_operation`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_operation.html) macro.
pub trait ApiOperation {
    /// Returns the definition for this operation.
    fn operation() -> Operation<DefaultSchemaRaw>;

    /// Returns the definitions used by this operation.
    fn definitions() -> BTreeMap<String, DefaultSchemaRaw>;
}

impl<T, B> App<T, B>
where
    B: MessageBody,
    T: NewService<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    /// See [`actix_web::App::service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.service).
    pub fn service<F>(mut self, mut factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        {
            let mut api = self.spec.write();
            api.definitions.extend(factory.definitions().into_iter());
            factory.update_operations(&mut api.paths);
        }

        self.inner = self.inner.service(factory);
        self
    }

    /// Mounts the specification for all operations and definitions
    /// recorded by the wrapper and serves them in the given path
    /// as a JSON.
    pub fn with_json_spec_at(mut self, path: &str) -> Self {
        self.inner = self.inner.service(
            actix_web::web::resource(path)
                .route(actix_web::web::get().to(SpecHandler(self.spec.clone()))),
        );
        self
    }

    /// Builds and returns the `actix_web::App`.
    pub fn build(self) -> actix_web::App<T, B> {
        self.inner
    }
}

#[derive(Clone)]
struct SpecHandler(Arc<RwLock<GenericApi<DefaultSchemaRaw>>>);

impl actix_web::dev::Factory<(), HttpResponse> for SpecHandler {
    fn call(&self, _: ()) -> HttpResponse {
        HttpResponse::Ok().json(&*self.0.read())
    }
}
