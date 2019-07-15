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

/* Resource */

/// Wrapper for [`actix_web::Resource`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html)
pub struct Resource<T> {
    path: String,
    operations: BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
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

    fn definitions(&self) -> &BTreeMap<String, DefaultSchemaRaw> {
        &self.definitions
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
    /// See [`actix_web::Resource::to`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.to).
    pub fn to<F, I, R>(mut self, handler: F) -> Self
    where
        F: ApiOperation + Factory<I, R> + 'static,
        I: FromRequest + 'static,
        R: Responder + 'static,
    {
        let mut op = F::operation();
        op.set_parameter_names_from_path_template(&self.path);

        for method in METHODS {
            self.operations.insert(method.into(), op.clone());
        }

        Resource {
            path: self.path,
            operations: self.operations,
            definitions: self.definitions,
            inner: self.inner.to(handler),
        }
    }

    /// See [`actix_web::Resource::route`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.route).
    pub fn route(mut self, route: Route) -> Self {
        if let Some(mut op) = route.operation {
            op.set_parameter_names_from_path_template(&self.path);

            if let Some(meth) = route.method {
                self.operations.insert(meth, op);
            } else {
                for method in METHODS {
                    self.operations.insert(method.into(), op.clone());
                }
            }
        }

        self.definitions.extend(route.definitions.into_iter());

        Resource {
            path: self.path,
            operations: self.operations,
            definitions: self.definitions,
            inner: self.inner.route(route.inner),
        }
    }
}

/// See [`actix_web::web::resource`](https://docs.rs/actix-web/*/actix_web/web/fn.resource.html).
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
        definitions: BTreeMap::new(),
        inner: actix_web::web::resource(path),
    }
}

/* Route */

/// Wrapper for [`actix_web::Route`](https://docs.rs/actix-web/*/actix_web/struct.Route.html)
pub struct Route {
    method: Option<HttpMethod>,
    operation: Option<Operation<DefaultSchemaRaw>>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: actix_web::Route,
}

impl Route {
    /// See [`actix_web::Route::new`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.new)
    #[allow(clippy::new_without_default)]
    pub fn new() -> Route {
        Route {
            method: None,
            operation: None,
            definitions: BTreeMap::new(),
            inner: actix_web::Route::new(),
        }
    }

    /// See [`actix_web::Route::method`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.method)
    pub fn method(self, method: Method) -> Self {
        Route {
            method: Some(HttpMethod::from(&method)),
            operation: self.operation,
            definitions: self.definitions,
            inner: self.inner.method(method),
        }
    }

    /// See [`actix_web::Route::to`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.to)
    pub fn to<F, I, R>(self, handler: F) -> Self
    where
        F: ApiOperation + Factory<I, R> + 'static,
        I: FromRequest + 'static,
        R: Responder + 'static,
    {
        Route {
            method: self.method,
            operation: Some(F::operation()),
            definitions: F::definitions(),
            inner: self.inner.to(handler),
        }
    }
}

/// See [`actix_web::web::method`](https://docs.rs/actix-web/*/actix_web/web/fn.method.html).
pub fn method(method: Method) -> Route {
    Route::new().method(method)
}

/// See [`actix_web::web::get`](https://docs.rs/actix-web/*/actix_web/web/fn.get.html).
pub fn get() -> Route {
    method(Method::GET)
}

/// See [`actix_web::web::put`](https://docs.rs/actix-web/*/actix_web/web/fn.put.html).
pub fn put() -> Route {
    method(Method::PUT)
}

/// See [`actix_web::web::post`](https://docs.rs/actix-web/*/actix_web/web/fn.post.html).
pub fn post() -> Route {
    method(Method::POST)
}

/// See [`actix_web::web::patch`](https://docs.rs/actix-web/*/actix_web/web/fn.patch.html).
pub fn patch() -> Route {
    method(Method::PATCH)
}

/// See [`actix_web::web::delete`](https://docs.rs/actix-web/*/actix_web/web/fn.delete.html).
pub fn delete() -> Route {
    method(Method::DELETE)
}

/// See [`actix_web::web::options`](https://docs.rs/actix-web/*/actix_web/web/fn.options.html).
pub fn options() -> Route {
    method(Method::OPTIONS)
}

/// See [`actix_web::web::head`](https://docs.rs/actix-web/*/actix_web/web/fn.head.html).
pub fn head() -> Route {
    method(Method::HEAD)
}
