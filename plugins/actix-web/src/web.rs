use crate::{ApiOperation, Mountable};
use actix_service::NewService;
use actix_web::dev::{AppService, HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{Error, Factory, FromRequest, Responder};
use paperclip::v2::models::{DefaultSchemaRaw, Operation};

pub use actix_web::web::{Json, Path};

pub struct Resource<T> {
    path: String,
    operations: Vec<Operation<DefaultSchemaRaw>>,
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
    pub fn to<F, I, R>(self, handler: F) -> Self
    where
        F: ApiOperation + Factory<I, R> + 'static,
        I: FromRequest + 'static,
        R: Responder + 'static,
    {
        let mut ops = self.operations;
        ops.push(F::operation());
        Resource {
            path: self.path,
            operations: ops,
            inner: self.inner.to(handler),
        }
    }
}

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
        operations: vec![],
        inner: actix_web::web::resource(path),
    }
}
