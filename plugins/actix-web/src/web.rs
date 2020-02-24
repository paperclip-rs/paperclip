//! Proxy module for [`actix_web::web`](https://docs.rs/actix-web/*/actix_web/web/index.html).

pub use actix_web::web::{
    block, service, to, Bytes, BytesMut, Data, Form, FormConfig, HttpRequest, HttpResponse, Json,
    JsonConfig, Path, PathConfig, Payload, PayloadConfig, Query, QueryConfig,
};

use crate::Mountable;
use actix_service::{IntoServiceFactory, ServiceFactory};
use actix_web::dev::{
    AppService, Factory, HttpServiceFactory, ServiceRequest, ServiceResponse, Transform,
};
use actix_web::guard::Guard;
use actix_web::{http::Method, Error, FromRequest, Responder};
use paperclip_core::v2::models::{
    DefaultOperationRaw, DefaultPathItemRaw, DefaultSchemaRaw, HttpMethod,
};
use paperclip_core::v2::schema::Apiv2Operation;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::Future;
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
pub struct Resource<R = actix_web::Resource> {
    path: String,
    operations: BTreeMap<HttpMethod, DefaultOperationRaw>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: R,
}

impl Resource {
    /// Wrapper for [`actix_web::Resource::new`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.new).
    pub fn new(path: &str) -> Resource {
        Resource {
            path: path.into(),
            operations: BTreeMap::new(),
            definitions: BTreeMap::new(),
            inner: actix_web::Resource::new(path),
        }
    }
}

impl<T> HttpServiceFactory for Resource<actix_web::Resource<T>>
where
    T: ServiceFactory<
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

impl<T> IntoServiceFactory<T> for Resource<actix_web::Resource<T>>
where
    T: ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        > + 'static,
{
    fn into_factory(self) -> T {
        self.inner.into_factory()
    }
}

impl<T> Mountable for Resource<T> {
    fn path(&self) -> &str {
        &self.path
    }

    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw> {
        mem::replace(&mut self.operations, BTreeMap::new())
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
    }
}

impl<T> Resource<actix_web::Resource<T>>
where
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// Proxy for [`actix_web::Resource::name`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.name).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn name(mut self, name: &str) -> Self {
        self.inner = self.inner.name(name);
        self
    }

    /// Proxy for [`actix_web::Resource::guard`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.guard).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
        self.inner = self.inner.guard(guard);
        self
    }

    /// Wrapper for [`actix_web::Resource::route`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.route).
    pub fn route(mut self, route: Route) -> Self {
        let w = RouteWrapper::from(&self.path, route);
        self.operations.extend(w.operations.into_iter());
        self.definitions.extend(w.definitions.into_iter());
        self.inner = self.inner.route(w.inner);
        self
    }

    /// Proxy for [`actix_web::Resource::data`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn data<U: 'static>(mut self, data: U) -> Self {
        self.inner = self.inner.data(data);
        self
    }

    /// Proxy for [`actix_web::Resource::app_data`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.app_data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn app_data<U: 'static>(mut self, data: U) -> Self {
        let w = self.inner.app_data(data);
        self.inner = w;
        self
    }

    /// Wrapper for [`actix_web::Resource::to`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.to).
    pub fn to<F, I, R, U>(mut self, handler: F) -> Self
    where
        F: Apiv2Operation<I, U> + Factory<I, R, U> + 'static,
        I: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.update_from_handler::<F, I, R, U>();
        self.inner = self.inner.to(handler);
        self
    }

    /// Proxy for [`actix_web::web::Resource::wrap`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.wrap).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap<M>(
        self,
        mw: M,
    ) -> Resource<
        actix_web::Resource<
            impl ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            >,
        >,
    >
    where
        M: Transform<
            T::Service,
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        >,
    {
        Resource {
            path: self.path,
            operations: self.operations,
            definitions: self.definitions,
            inner: self.inner.wrap(mw),
        }
    }

    /// Proxy for [`actix_web::web::Resource::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.wrap_fn).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap_fn<F, R>(
        self,
        mw: F,
    ) -> Resource<
        actix_web::Resource<
            impl ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            >,
        >,
    >
    where
        F: FnMut(ServiceRequest, &mut T::Service) -> R + Clone,
        R: Future<Output = Result<ServiceResponse, Error>>,
    {
        Resource {
            path: self.path,
            operations: self.operations,
            definitions: self.definitions,
            inner: self.inner.wrap_fn(mw),
        }
    }

    /// Proxy for [`actix_web::web::Resource::default_service`](https://docs.rs/actix-web/*/actix_web/struct.Resource.html#method.default_service).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn default_service<F, U>(mut self, f: F) -> Self
    where
        F: actix_service::IntoServiceFactory<U>,
        U: ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            > + 'static,
        U::InitError: Debug,
    {
        self.inner = self.inner.default_service(f);
        self
    }

    /// Updates this resource using the given handler.
    fn update_from_handler<F, I, R, U>(&mut self)
    where
        F: Apiv2Operation<I, U>,
    {
        let mut op = F::operation();
        op.set_parameter_names_from_path_template(&self.path);

        for method in METHODS {
            self.operations.insert(method.into(), op.clone());
        }

        self.definitions.extend(F::definitions().into_iter());
    }
}

/// Wrapper for [`actix_web::web::resource`](https://docs.rs/actix-web/*/actix_web/web/fn.resource.html).
pub fn resource(path: &str) -> Resource {
    Resource::new(path)
}

/* Scope */

/// Wrapper for [`actix_web::Scope`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html)
pub struct Scope<S = actix_web::Scope> {
    path: String,
    path_map: BTreeMap<String, DefaultPathItemRaw>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: S,
}

impl Scope {
    /// Wrapper for [`actix_web::Scope::new`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.new)
    pub fn new(path: &str) -> Self {
        Scope {
            path: path.into(),
            path_map: BTreeMap::new(),
            definitions: BTreeMap::new(),
            inner: actix_web::Scope::new(path),
        }
    }
}

impl<T> HttpServiceFactory for Scope<actix_web::Scope<T>>
where
    T: ServiceFactory<
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

impl<T> Scope<actix_web::Scope<T>>
where
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// Proxy for [`actix_web::Scope::guard`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.guard).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
        self.inner = self.inner.guard(guard);
        self
    }

    /// Proxy for [`actix_web::Scope::data`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn data<U: 'static>(mut self, data: U) -> Self {
        self.inner = self.inner.data(data);
        self
    }

    /// Wrapper for [`actix_web::Scope::configure`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.configure).
    pub fn configure<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ServiceConfig<actix_web::Scope<T>>),
    {
        let mut cfg = ServiceConfig {
            path_map: BTreeMap::new(),
            definitions: BTreeMap::new(),
            scope: Some(self.inner),
        };

        f(&mut cfg);
        self.inner = cfg
            .scope
            .take()
            .expect("missing scope object after configuring?");
        self.update_from_mountable(&mut cfg);
        self
    }

    /// Wrapper for [`actix_web::Scope::service`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.service).
    pub fn service<F>(mut self, mut factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.update_from_mountable(&mut factory);
        self.inner = self.inner.service(factory);
        self
    }

    /// Wrapper for [`actix_web::Scope::route`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.route).
    pub fn route(mut self, path: &str, route: Route) -> Self {
        let mut w = RouteWrapper::from(path, route);
        self.update_from_mountable(&mut w);
        self.inner = self.inner.route(path, w.inner);
        self
    }

    /// Proxy for [`actix_web::web::Scope::default_service`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.default_service).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn default_service<F, U>(mut self, f: F) -> Self
    where
        F: actix_service::IntoServiceFactory<U>,
        U: ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            > + 'static,
        U::InitError: Debug,
    {
        self.inner = self.inner.default_service(f);
        self
    }

    /// Proxy for [`actix_web::web::Scope::wrap`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.wrap).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap<M>(
        self,
        mw: M,
    ) -> Scope<
        actix_web::Scope<
            impl ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            >,
        >,
    >
    where
        M: Transform<
            T::Service,
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = Error,
            InitError = (),
        >,
    {
        Scope {
            path: self.path,
            path_map: self.path_map,
            definitions: self.definitions,
            inner: self.inner.wrap(mw),
        }
    }

    /// Proxy for [`actix_web::web::Scope::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.Scope.html#method.wrap_fn).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap_fn<F, R>(
        self,
        mw: F,
    ) -> Scope<
        actix_web::Scope<
            impl ServiceFactory<
                Config = (),
                Request = ServiceRequest,
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            >,
        >,
    >
    where
        F: FnMut(ServiceRequest, &mut T::Service) -> R + Clone,
        R: Future<Output = Result<ServiceResponse, Error>>,
    {
        Scope {
            path: self.path,
            path_map: self.path_map,
            definitions: self.definitions,
            inner: self.inner.wrap_fn(mw),
        }
    }

    /// Updates `self` using the given `Mountable` object.
    fn update_from_mountable<M>(&mut self, factory: &mut M)
    where
        M: Mountable,
    {
        self.definitions.extend(factory.definitions().into_iter());
        let mut path_map = BTreeMap::new();
        factory.update_operations(&mut path_map);
        for (path, map) in path_map {
            self.path_map.insert(self.path.clone() + &path, map);
        }
    }
}

impl<T> Mountable for Scope<T> {
    fn path(&self) -> &str {
        unimplemented!("Scope has multiple paths. Use `update_operations` object instead.");
    }

    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw> {
        unimplemented!("Scope has multiple operation maps. Use `update_operations` object instead.")
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
    }

    fn update_operations(&mut self, map: &mut BTreeMap<String, DefaultPathItemRaw>) {
        *map = mem::replace(&mut self.path_map, BTreeMap::new());
    }
}

/// Wrapper for [`actix_web::web::scope`](https://docs.rs/actix-web/*/actix_web/web/fn.scope.html).
pub fn scope(path: &str) -> Scope {
    Scope::new(path)
}

/* Route */

/// Wrapper for [`actix_web::Route`](https://docs.rs/actix-web/*/actix_web/struct.Route.html)
pub struct Route {
    method: Option<HttpMethod>,
    operation: Option<DefaultOperationRaw>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    inner: actix_web::Route,
}

impl ServiceFactory for Route {
    type Config = ();
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Service = <actix_web::Route as ServiceFactory>::Service;
    type Future = <actix_web::Route as ServiceFactory>::Future;

    fn new_service(&self, cfg: Self::Config) -> Self::Future {
        #[allow(clippy::unit_arg)]
        self.inner.new_service(cfg)
    }
}

impl Route {
    /// Wrapper for [`actix_web::Route::new`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.new)
    #[allow(clippy::new_without_default)]
    pub fn new() -> Route {
        Route {
            method: None,
            operation: None,
            definitions: BTreeMap::new(),
            inner: actix_web::Route::new(),
        }
    }

    /// Wrapper for [`actix_web::Route::method`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.method)
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(HttpMethod::from(&method));
        self.inner = self.inner.method(method);
        self
    }

    /// Proxy for [`actix_web::Route::guard`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.guard).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn guard<G: Guard + 'static>(mut self, guard: G) -> Self {
        self.inner = self.inner.guard(guard);
        self
    }

    /// Wrapper for [`actix_web::Route::to`](https://docs.rs/actix-web/*/actix_web/struct.Route.html#method.to)
    pub fn to<F, I, R, U>(mut self, handler: F) -> Self
    where
        F: Apiv2Operation<I, U> + Factory<I, R, U>,
        I: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.operation = Some(F::operation());
        self.definitions = F::definitions();
        self.inner = self.inner.to(handler);
        self
    }
}

/// Wrapper for [`actix_web::web::method`](https://docs.rs/actix-web/*/actix_web/web/fn.method.html).
pub fn method(method: Method) -> Route {
    Route::new().method(method)
}

/// Wrapper for [`actix_web::web::get`](https://docs.rs/actix-web/*/actix_web/web/fn.get.html).
pub fn get() -> Route {
    method(Method::GET)
}

/// Wrapper for [`actix_web::web::put`](https://docs.rs/actix-web/*/actix_web/web/fn.put.html).
pub fn put() -> Route {
    method(Method::PUT)
}

/// Wrapper for [`actix_web::web::post`](https://docs.rs/actix-web/*/actix_web/web/fn.post.html).
pub fn post() -> Route {
    method(Method::POST)
}

/// Wrapper for [`actix_web::web::patch`](https://docs.rs/actix-web/*/actix_web/web/fn.patch.html).
pub fn patch() -> Route {
    method(Method::PATCH)
}

/// Wrapper for [`actix_web::web::delete`](https://docs.rs/actix-web/*/actix_web/web/fn.delete.html).
pub fn delete() -> Route {
    method(Method::DELETE)
}

/// Wrapper for [`actix_web::web::options`](https://docs.rs/actix-web/*/actix_web/web/fn.options.html).
pub fn options() -> Route {
    method(Method::OPTIONS)
}

/// Wrapper for [`actix_web::web::head`](https://docs.rs/actix-web/*/actix_web/web/fn.head.html).
pub fn head() -> Route {
    method(Method::HEAD)
}

/// Workaround for issue #17. In actix-web, a method in a route is a guard for that route.
/// Whenever we call `App::route`, `Scope::route` or `ServiceConfig::route`, actix-web
/// creates a new resource with a route (by calling `Resource::new(path).route(route)`),
/// but then it also internally moves the guards to the new entity (manually). This forces
/// us to call the `.route` method on that entity rather than creating a resource with a
/// route. This wrapper is `Mountable` and can be used by `App`, `Scope`, etc. when calling
/// the `.route()` method.
pub(crate) struct RouteWrapper<S> {
    path: S,
    pub(crate) operations: BTreeMap<HttpMethod, DefaultOperationRaw>,
    pub(crate) definitions: BTreeMap<String, DefaultSchemaRaw>,
    pub(crate) inner: actix_web::Route,
}

impl<S> RouteWrapper<S>
where
    S: AsRef<str>,
{
    pub(crate) fn from(path: S, route: Route) -> Self {
        let mut operations = BTreeMap::new();
        if let Some(mut op) = route.operation {
            op.set_parameter_names_from_path_template(path.as_ref());

            if let Some(meth) = route.method {
                operations.insert(meth, op);
            } else {
                for method in METHODS {
                    operations.insert(method.into(), op.clone());
                }
            }
        }

        RouteWrapper {
            path,
            operations,
            definitions: route.definitions,
            inner: route.inner,
        }
    }
}

impl<S> Mountable for RouteWrapper<S>
where
    S: AsRef<str>,
{
    fn path(&self) -> &str {
        self.path.as_ref()
    }

    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw> {
        mem::replace(&mut self.operations, BTreeMap::new())
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
    }
}

/* Service config */

/// Wrapper for [`actix_web::web::ServiceConfig`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html).
pub struct ServiceConfig<S = actix_web::Scope> {
    path_map: BTreeMap<String, DefaultPathItemRaw>,
    definitions: BTreeMap<String, DefaultSchemaRaw>,
    scope: Option<S>,
}

impl<T> Mountable for ServiceConfig<T> {
    fn path(&self) -> &str {
        unimplemented!("ServiceConfig has multiple paths. Use `update_operations` object instead.");
    }

    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw> {
        unimplemented!(
            "ServiceConfig has multiple operation maps. Use `update_operations` object instead."
        )
    }

    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw> {
        mem::replace(&mut self.definitions, BTreeMap::new())
    }

    fn update_operations(&mut self, map: &mut BTreeMap<String, DefaultPathItemRaw>) {
        *map = mem::replace(&mut self.path_map, BTreeMap::new());
    }
}

impl<T> ServiceConfig<actix_web::Scope<T>>
where
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
{
    /// Wrapper for [`actix_web::web::ServiceConfig::route`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.route).
    pub fn route(&mut self, path: &str, route: Route) -> &mut Self {
        let mut w = RouteWrapper::from(path, route);
        self.definitions.extend(w.definitions().into_iter());
        w.update_operations(&mut self.path_map);
        self.scope = self.scope.take().map(|s| s.route(path, w.inner));
        self
    }

    /// Wrapper for [`actix_web::web::ServiceConfig::service`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.service).
    pub fn service<F>(&mut self, mut factory: F) -> &mut Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.definitions.extend(factory.definitions().into_iter());
        factory.update_operations(&mut self.path_map);
        self.scope = self.scope.take().map(|s| s.service(factory));
        self
    }

    /// Proxy for [`actix_web::web::ServiceConfig::external_resource`](https://docs.rs/actix-web/*/actix_web/web/struct.ServiceConfig.html#method.external_resource).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn external_resource<N, U>(&mut self, name: N, url: U) -> &mut Self
    where
        N: AsRef<str>,
        U: AsRef<str>,
    {
        self.scope = self.scope.take().map(|s| {
            s.configure(|cfg| {
                cfg.external_resource(name, url);
            })
        });
        self
    }
}
