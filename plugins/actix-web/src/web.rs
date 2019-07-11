use crate::{ApiOperation, Mountable};
use actix_service::NewService;
use actix_web::dev::{AppService, HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{http::Method, Error, Factory, FromRequest, Responder};
use paperclip::v2::models::{DefaultSchemaRaw, HttpMethod, Operation};

use std::collections::BTreeMap;

const METHODS: &[Method] = &[
    Method::GET,
    Method::PUT,
    Method::POST,
    Method::DELETE,
    Method::OPTIONS,
    Method::HEAD,
    Method::PATCH,
];

pub use actix_web::web::{Json, Path};

/// Wrapper for actix-web [`Resource`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html)
pub struct Resource<T> {
    path: String,
    operations: BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>>,
    inner: actix_web::Resource<T>,
}

impl<T> HttpServiceFactory for Resource<T>
where
    T: NewService<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        > + 'static,
{
    fn register(self, config: &mut AppService) {
        self.inner.register(config)
    }
}

impl<T> Mountable for Resource<T> {
    fn path(&self) -> &str {
        &self.path
    }

    fn operations(&self) -> &BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>> {
        &self.operations
    }
}

impl<T> Resource<T>
where
    T: NewService<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// See actix-web [`Resource::to`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.to).
    pub fn to<F, I, R>(mut self, handler: F) -> Self
    where
        F: ApiOperation + Factory<I, R> + 'static,
        I: FromRequest + 'static,
        R: Responder + 'static,
    {
        for method in METHODS {
            self.operations.insert(method.into(), F::operation());
        }

        Resource {
            path: self.path,
            operations: self.operations,
            inner: self.inner.to(handler),
        }
    }
}

/// See actix-web [`web::resource`](https://docs.rs/actix-web/*/actix_web/web/fn.resource.html).
pub fn resource(
    path: &str,
) -> Resource<
    impl NewService<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    Resource {
        path: path.into(),
        operations: BTreeMap::new(),
        inner: actix_web::web::resource(path),
    }
}
