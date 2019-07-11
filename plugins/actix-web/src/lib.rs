pub mod web;

pub use actix_web::{
    body, client, cookie, dev, error, guard, http, middleware, test, Error, Factory, HttpRequest,
    HttpResponse, HttpServer, Responder, Route,
};
pub use paperclip_actix_macros::{api_v2_schema, api_v2_operation};

use actix_service::NewService;
use actix_web::dev::{HttpServiceFactory, MessageBody, ServiceRequest, ServiceResponse};
use paperclip::v2::models::{DefaultSchemaRaw, GenericApi, HttpMethod, Operation, OperationMap};
use parking_lot::RwLock;

use std::collections::BTreeMap;
use std::sync::Arc;

/// Wrapper for actix-web [`App`](https://docs.rs/actix-web/*/actix_web/struct.App.html).
pub struct App<T, B> {
    spec: Arc<RwLock<GenericApi<DefaultSchemaRaw>>>,
    inner: actix_web::App<T, B>,
}

/// Extension trait for applications.
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

/// Indicates that this thingmabob has a path and a bunch of operations.
pub trait Mountable {
    /// Where this thing gets mounted.
    fn path(&self) -> &str;

    /// Map of HTTP methods and the associated API operations.
    fn operations(&self) -> &BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>>;
}

/// Represents a OpenAPI v2 schema convertible. This is auto-implemented by
/// [`api_v2_schema`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_schema.html) macro.
pub trait Apiv2Schema {
    /// Name of this schema. This is the object's name.
    const NAME: &'static str;

    /// Returns the schema for this object.
    fn schema() -> DefaultSchemaRaw;
}


/// Represents a OpenAPI v2 operation convertible. This is auto-implemented by
/// [`api_v2_operation`](https://paperclip.waffles.space/paperclip_actix_macros/attr.api_v2_operation.html) macro.
pub trait ApiOperation {
    /// Returns the definition for this operation.
    fn operation() -> Operation<DefaultSchemaRaw>;
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
    pub fn service<F>(self, factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        {
            let mut api = self.spec.write();
            let map = api
                .paths
                .entry(factory.path().into())
                .or_insert_with(OperationMap::default);
            map.methods.extend(factory.operations().clone().into_iter());
        }

        App {
            spec: self.spec,
            inner: self.inner.service(factory),
        }
    }

    /// Mounts the specification for all operations and definitions
    /// recorded by the wrapper and serves them in the given path
    /// as a JSON.
    pub fn with_json_spec_at(self, path: &str) -> Self {
        App {
            inner: self
                .inner
                .service(actix_web::web::resource(path).to(SpecHandler(self.spec.clone()))),
            spec: self.spec,
        }
    }

    /// Builds and returns the actix-web `App`.
    pub fn build(self) -> actix_web::App<T, B> {
        self.inner
    }
}

#[derive(Clone)]
struct SpecHandler(Arc<RwLock<GenericApi<DefaultSchemaRaw>>>);

impl Factory<(), HttpResponse> for SpecHandler {
    fn call(&self, _: ()) -> HttpResponse {
        HttpResponse::Ok().json(&*self.0.read())
    }
}
