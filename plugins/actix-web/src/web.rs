pub use actix_web::web::{
    block, service, to, to_async, Bytes, BytesMut, Data, Form, FormConfig, HttpRequest,
    HttpResponse, Json, JsonConfig, Path, PathConfig, Payload, PayloadConfig, Query, QueryConfig,
    ServiceConfig,
};

use crate::{ApiOperation, Mountable};
use actix_service::NewService;
use actix_web::dev::{AppService, Factory, HttpServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{http::Method, Error, FromRequest, Responder};
use paperclip::v2::models::{DefaultSchemaRaw, HttpMethod, Operation, OperationMap};

use std::collections::BTreeMap;
use std::mem;

const METHODS: &[Method] = &[
    Method::GET,
    Method::PUT,
    Method::POST,
    Method::DELETE,
    Method::OPTIONS,
    Method::HEAD,
    Method::PATCH,
];

/* Resource */

/// Wrapper for [`actix_web::Resource`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html)
pub struct Resource<T> {
    path: String,
    operations: BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: actix_web::Resource<T>,
}

impl Resource<()> {
    /// See [`actix_web::Resource::new`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.new).
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
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
            inner: actix_web::Resource::new(path),
        }
    }
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

    fn operations(&mut self) -> BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>> {
        mem::replace(&mut self.operations, BTreeMap::new())
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
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

        self.inner = self.inner.to(handler);
        self
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
        self.inner = self.inner.route(route.inner);
        self
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
    Resource::new(path)
}

/* Scope */

/// Wrapper for [`actix_web::Scope`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html)
pub struct Scope<T> {
    path: String,
    path_map: BTreeMap<String, OperationMap<DefaultSchemaRaw>>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: actix_web::Scope<T>,
}

impl Scope<()> {
    /// See [`actix_web::Scope::new`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.new)
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        path: &str,
    ) -> Scope<
        impl NewService<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        >,
    > {
        Scope {
            path: path.into(),
            path_map: BTreeMap::new(),
            definitions: BTreeMap::new(),
            inner: actix_web::Scope::new(path),
        }
    }
}

impl<T> HttpServiceFactory for Scope<T>
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

impl<T> Scope<T>
where
    T: NewService<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// See [`actix_web::Scope::service`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.service).
    pub fn service<F>(mut self, mut factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.definitions.extend(factory.definitions().into_iter());
        let mut path_map = BTreeMap::new();
        factory.update_operations(&mut path_map);
        for (path, map) in path_map {
            self.path_map.insert(self.path.clone() + &path, map);
        }

        self.inner = self.inner.service(factory);
        self
    }
}

impl<T> Mountable for Scope<T> {
    fn path(&self) -> &str {
        unimplemented!("Scope has multiple paths. Use `update_operations` object instead.");
    }

    fn operations(&mut self) -> BTreeMap<HttpMethod, Operation<DefaultSchemaRaw>> {
        unimplemented!("Scope has multiple operation maps. Use `update_operations` object instead.")
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
    }

    fn update_operations(&mut self, map: &mut BTreeMap<String, OperationMap<DefaultSchemaRaw>>) {
        *map = mem::replace(&mut self.path_map, BTreeMap::new());
    }
}

/// See [`actix_web::web::scope`](https://docs.rs/actix-web/*/actix_web/web/fn.scope.html).
pub fn scope(
    path: &str,
) -> Scope<
    impl NewService<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    Scope::new(path)
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
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(HttpMethod::from(&method));
        self.inner = self.inner.method(method);
        self
    }

    /// See [`actix_web::Route::to`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.to)
    pub fn to<F, I, R>(mut self, handler: F) -> Self
    where
        F: ApiOperation + Factory<I, R> + 'static,
        I: FromRequest + 'static,
        R: Responder + 'static,
    {
        self.operation = Some(F::operation());
        self.definitions = F::definitions();
        self.inner = self.inner.to(handler);
        self
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
