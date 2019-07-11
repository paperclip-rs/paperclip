pub mod web;

pub use actix_web::{
    body, client, cookie, dev, error, guard, http, middleware, test, Error, Factory, HttpRequest,
    HttpResponse, HttpServer, Responder, Route,
};
pub use paperclip_actix_macros::*;

use actix_service::NewService;
use actix_web::dev::{HttpServiceFactory, MessageBody, ServiceRequest, ServiceResponse};
use paperclip::v2::models::{DefaultSchemaRaw, GenericApi, Operation};
use parking_lot::RwLock;

use std::sync::Arc;

pub struct App<T, B> {
    paths: Vec<String>,
    spec: Arc<RwLock<GenericApi<DefaultSchemaRaw>>>,
    inner: actix_web::App<T, B>,
}

impl<T, B> OpenApiExt<T, B> for actix_web::App<T, B> {
    type Wrapper = App<T, B>;

    fn record_operations(self) -> Self::Wrapper {
        App {
            paths: vec![],
            spec: Arc::new(RwLock::new(GenericApi::default())),
            inner: self,
        }
    }
}

pub trait OpenApiExt<T, B> {
    type Wrapper;

    fn record_operations(self) -> Self::Wrapper;
}

pub trait Mountable {
    fn path(&self) -> &str;
}

pub trait Apiv2Schema {
    const NAME: &'static str;

    fn schema() -> DefaultSchemaRaw;
}

pub trait ApiOperation {
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
    pub fn service<F>(mut self, factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.paths.push(factory.path().into());
        App {
            paths: self.paths,
            spec: self.spec,
            inner: self.inner.service(factory),
        }
    }

    pub fn with_json_spec_at(self, path: &str) -> Self {
        App {
            paths: self.paths,
            inner: self
                .inner
                .service(actix_web::web::resource(path).to(SpecHandler(self.spec.clone()))),
            spec: self.spec,
        }
    }

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
