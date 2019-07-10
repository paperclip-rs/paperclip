pub mod web;

pub use actix_web::{
    body, client, cookie, dev, error, guard, http, middleware, test, Error, HttpRequest,
    HttpResponse, HttpServer, Route,
};
pub use paperclip_actix_macros::*;

use actix_service::NewService;
use actix_web::dev::{HttpServiceFactory, MessageBody, ServiceRequest, ServiceResponse};
use paperclip::v2::models::{DefaultSchema, Operation};

pub struct App<T, B> {
    paths: Vec<String>,
    inner: actix_web::App<T, B>,
}

impl<T, B> OpenApiExt<T, B> for actix_web::App<T, B> {
    fn record_operations(self) -> App<T, B> {
        App {
            paths: vec![],
            inner: self,
        }
    }
}

pub trait OpenApiExt<T, B> {
    fn record_operations(self) -> App<T, B>;
}

pub trait Mountable {
    fn path(&self) -> &str;
}

pub trait Apiv2Schema {
    const NAME: &'static str;

    fn schema() -> DefaultSchema;
}

pub trait ApiOperation {
    const NAME: &'static str;

    fn operation() -> Operation<DefaultSchema>;
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
            inner: self.inner.service(factory),
        }
    }
}
