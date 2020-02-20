pub mod web;

pub use self::web::{Resource, Route, Scope};
pub use paperclip_macros::{api_v2_operation, api_v2_schema};

use self::web::{RouteWrapper, ServiceConfig};
use actix_service::ServiceFactory;
use actix_web::dev::{HttpServiceFactory, MessageBody, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web::HttpResponse, Error};
use futures::future::{ok as fut_ok, Ready};
use paperclip_core::v2::models::{
    DefaultApiRaw, DefaultOperationRaw, DefaultPathItemRaw, DefaultSchemaRaw, HttpMethod,
};
use parking_lot::RwLock;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;

/// Wrapper for [`actix_web::App`](https://docs.rs/actix-web/*/actix_web/struct.App.html).
pub struct App<T, B> {
    spec: Arc<RwLock<DefaultApiRaw>>,
    inner: actix_web::App<T, B>,
}

/// Extension trait for actix-web applications.
pub trait OpenApiExt<T, B> {
    type Wrapper;

    /// Consumes this app and produces its wrapper to start tracking
    /// paths and their corresponding operations.
    fn wrap_api(self) -> Self::Wrapper;
}

impl<T, B> OpenApiExt<T, B> for actix_web::App<T, B> {
    type Wrapper = App<T, B>;

    fn wrap_api(self) -> Self::Wrapper {
        App {
            spec: Arc::new(RwLock::new(DefaultApiRaw::default())),
            inner: self,
        }
    }
}

/// Indicates that this thingmabob has a path and a bunch of definitions and operations.
pub trait Mountable {
    /// Where this thing gets mounted.
    fn path(&self) -> &str;

    /// Map of HTTP methods and the associated API operations.
    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw>;

    /// The definitions recorded by this object.
    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw>;

    /// Updates the given map of operations with operations tracked by this object.
    ///
    /// **NOTE:** Overriding implementations must ensure that the `PathItem`
    /// is normalized before updating the input map.
    fn update_operations(&mut self, map: &mut BTreeMap<String, DefaultPathItemRaw>) {
        let op_map = map
            .entry(self.path().into())
            .or_insert_with(Default::default);
        op_map.methods.extend(self.operations().into_iter());
        op_map.normalize();
    }
}

impl<T, B> App<T, B>
where
    B: MessageBody,
    T: ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
        InitError = (),
    >,
{
    /// Proxy for [`actix_web::App::data`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn data<U: 'static>(mut self, data: U) -> Self {
        self.inner = self.inner.data(data);
        self
    }

    /// Proxy for [`actix_web::App::data_factory`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.data_factory).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn data_factory<F, Out, D, E>(mut self, data: F) -> Self
    where
        F: Fn() -> Out + 'static,
        Out: Future<Output = Result<D, E>> + 'static,
        D: 'static,
        E: Debug,
    {
        self.inner = self.inner.data_factory(data);
        self
    }

    /// Proxy for [`actix_web::App::app_data`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.app_data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn app_data<U: 'static>(mut self, data: U) -> Self {
        self.inner = self.inner.app_data(data);
        self
    }

    /// Wrapper for [`actix_web::App::configure`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.configure).
    pub fn configure<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ServiceConfig),
    {
        self.service(Scope::new("").configure(f))
    }

    /// Wrapper for [`actix_web::App::route`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.route).
    pub fn route(mut self, path: &str, route: Route) -> Self {
        let mut w = RouteWrapper::from(path, route);
        self.update_from_mountable(&mut w);
        self.inner = self.inner.route(path, w.inner);
        self
    }

    /// Wrapper for [`actix_web::App::service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.service).
    pub fn service<F>(mut self, mut factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.update_from_mountable(&mut factory);
        self.inner = self.inner.service(factory);
        self
    }

    /// Proxy for [`actix_web::App::default_service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.default_service).
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

    /// Proxy for [`actix_web::App::external_resource`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.external_resource).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn external_resource<N, U>(mut self, name: N, url: U) -> Self
    where
        N: AsRef<str>,
        U: AsRef<str>,
    {
        self.inner = self.inner.external_resource(name, url);
        self
    }

    /// Proxy for [`actix_web::web::App::wrap`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap<M, B1>(
        self,
        mw: M,
    ) -> App<
        impl ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse<B1>,
            Error = Error,
            InitError = (),
        >,
        B1,
    >
    where
        M: Transform<
            T::Service,
            Request = ServiceRequest,
            Response = ServiceResponse<B1>,
            Error = Error,
            InitError = (),
        >,
        B1: MessageBody,
    {
        App {
            spec: self.spec,
            inner: self.inner.wrap(mw),
        }
    }

    /// Proxy for [`actix_web::web::App::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap_fn).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap_fn<B1, F, R>(
        self,
        mw: F,
    ) -> App<
        impl ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse<B1>,
            Error = Error,
            InitError = (),
        >,
        B1,
    >
    where
        B1: MessageBody,
        F: FnMut(ServiceRequest, &mut T::Service) -> R + Clone,
        R: Future<Output = Result<ServiceResponse<B1>, Error>>,
    {
        App {
            spec: self.spec,
            inner: self.inner.wrap_fn(mw),
        }
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

    /// Updates the underlying spec with definitions and operations from the given factory.
    fn update_from_mountable<F>(&mut self, factory: &mut F)
    where
        F: Mountable,
    {
        let mut api = self.spec.write();
        api.definitions.extend(factory.definitions().into_iter());
        factory.update_operations(&mut api.paths);
    }
}

#[derive(Clone)]
struct SpecHandler(Arc<RwLock<DefaultApiRaw>>);

impl actix_web::dev::Factory<(), Ready<Result<HttpResponse, Error>>, Result<HttpResponse, Error>>
    for SpecHandler
{
    fn call(&self, _: ()) -> Ready<Result<HttpResponse, Error>> {
        fut_ok(HttpResponse::Ok().json(&*self.0.read()))
    }
}
