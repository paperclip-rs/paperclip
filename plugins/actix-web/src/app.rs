#![allow(clippy::return_self_not_must_use)]

extern crate actix_service2 as actix_service;
extern crate actix_web4 as actix_web;

#[cfg(feature = "rapidoc")]
use super::RAPIDOC;
#[cfg(feature = "swagger-ui")]
use super::SWAGGER_DIST;
use super::{
    web::{Route, RouteWrapper, ServiceConfig},
    Mountable,
};
use actix_service::ServiceFactory;
#[cfg(any(feature = "swagger-ui"))]
use actix_web::HttpRequest;
use actix_web::{
    body::MessageBody,
    dev::{HttpServiceFactory, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok as fut_ok, Ready};
use paperclip_core::v2::models::{DefaultApiRaw, SecurityScheme};
#[cfg(feature = "rapidoc")]
use tinytemplate::TinyTemplate;

use std::{
    collections::BTreeMap,
    fmt::Debug,
    future::Future,
    sync::{Arc, RwLock},
};

/// Wrapper for [`actix_web::App`](https://docs.rs/actix-web/*/actix_web/struct.App.html).
pub struct App<T> {
    spec: Arc<RwLock<DefaultApiRaw>>,
    #[cfg(feature = "v3")]
    spec_v3: Option<Arc<RwLock<openapiv3::OpenAPI>>>,
    #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
    spec_path: Option<String>,
    inner: Option<actix_web::App<T>>,
}

/// Extension trait for actix-web applications.
pub trait OpenApiExt<T> {
    type Wrapper;

    /// Consumes this app and produces its wrapper to start tracking
    /// paths and their corresponding operations.
    fn wrap_api(self) -> Self::Wrapper;

    /// Same as `wrap_api` initializing with provided specification
    /// defaults. Useful for defining Api properties outside of definitions and
    /// paths.
    fn wrap_api_with_spec(self, spec: DefaultApiRaw) -> Self::Wrapper;
}

impl<T> OpenApiExt<T> for actix_web::App<T> {
    type Wrapper = App<T>;

    fn wrap_api(self) -> Self::Wrapper {
        App {
            spec: Arc::new(RwLock::new(DefaultApiRaw::default())),
            #[cfg(feature = "v3")]
            spec_v3: None,
            #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
            spec_path: None,
            inner: Some(self),
        }
    }

    fn wrap_api_with_spec(self, spec: DefaultApiRaw) -> Self::Wrapper {
        App {
            spec: Arc::new(RwLock::new(spec)),
            #[cfg(feature = "v3")]
            spec_v3: None,
            #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
            spec_path: None,
            inner: Some(self),
        }
    }
}

impl<T> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
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
        self.inner = self.inner.take().map(|a| a.data_factory(data));
        self
    }

    /// Proxy for [`actix_web::App::app_data`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.app_data).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn app_data<U: 'static>(mut self, data: U) -> Self {
        self.inner = self.inner.take().map(|a| a.app_data(data));
        self
    }

    /// Wrapper for [`actix_web::App::configure`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.configure).
    pub fn configure<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ServiceConfig),
    {
        self.inner = self.inner.take().map(|s| {
            s.configure(|c| {
                let mut cfg = ServiceConfig::from(c);
                f(&mut cfg);
                self.update_from_mountable(&mut cfg);
            })
        });
        self
    }

    /// Wrapper for [`actix_web::App::route`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.route).
    pub fn route(mut self, path: &str, route: Route) -> Self {
        let mut w = RouteWrapper::from(path, route);
        self.update_from_mountable(&mut w);
        self.inner = self.inner.take().map(|a| a.route(path, w.inner));
        self
    }

    /// Wrapper for [`actix_web::App::service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.service).
    pub fn service<F>(mut self, mut factory: F) -> Self
    where
        F: Mountable + HttpServiceFactory + 'static,
    {
        self.update_from_mountable(&mut factory);
        self.inner = self.inner.take().map(|a| a.service(factory));
        self
    }

    /// Proxy for [`actix_web::App::default_service`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.default_service).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn default_service<F, U>(mut self, f: F) -> Self
    where
        F: actix_service::IntoServiceFactory<U, ServiceRequest>,
        U: ServiceFactory<
                ServiceRequest,
                Config = (),
                Response = ServiceResponse,
                Error = Error,
                InitError = (),
            > + 'static,
        U::InitError: Debug,
    {
        self.inner = self.inner.take().map(|a| a.default_service(f));
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
        self.inner = self.inner.take().map(|a| a.external_resource(name, url));
        self
    }

    /// Proxy for [`actix_web::web::App::wrap`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap<M, B>(
        mut self,
        mw: M,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        >,
    >
    where
        M: Transform<
                T::Service,
                ServiceRequest,
                Response = ServiceResponse<B>,
                Error = Error,
                InitError = (),
            > + 'static,
        B: MessageBody,
    {
        App {
            spec: self.spec,
            #[cfg(feature = "v3")]
            spec_v3: self.spec_v3,
            #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
            spec_path: None,
            inner: self.inner.take().map(|a| a.wrap(mw)),
        }
    }

    /// Proxy for [`actix_web::web::App::wrap_fn`](https://docs.rs/actix-web/*/actix_web/struct.App.html#method.wrap_fn).
    ///
    /// **NOTE:** This doesn't affect spec generation.
    pub fn wrap_fn<F, R, B>(
        mut self,
        mw: F,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        >,
    >
    where
        F: Fn(ServiceRequest, &T::Service) -> R + Clone + 'static,
        R: Future<Output = Result<ServiceResponse<B>, Error>>,
        B: MessageBody,
    {
        App {
            spec: self.spec,
            #[cfg(feature = "v3")]
            spec_v3: self.spec_v3,
            #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
            spec_path: None,
            inner: self.inner.take().map(|a| a.wrap_fn(mw)),
        }
    }

    /// Mounts the specification for all operations and definitions
    /// recorded by the wrapper and serves them in the given path
    /// as a JSON.
    pub fn with_json_spec_at(mut self, path: &str) -> Self {
        #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
        {
            self.spec_path = Some(path.to_owned());
        }

        self.inner = self.inner.take().map(|a| {
            a.service(
                actix_web::web::resource(path)
                    .route(actix_web::web::get().to(SpecHandler(self.spec.clone()))),
            )
        });
        self
    }

    #[cfg(feature = "v3")]
    /// Converts the generated v2 specification to v3 and then
    /// mounts the v3 specification for all operations and definitions
    /// recorded by the wrapper and serves them in the given path
    /// as a JSON.
    pub fn with_json_spec_v3_at(mut self, path: &str) -> Self {
        #[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
        {
            self.spec_path = Some(path.to_owned());
        }

        let spec_v3 = if let Some(spec_v3) = &self.spec_v3 {
            spec_v3.clone()
        } else {
            let spec_v3 = Arc::new(RwLock::new(openapiv3::OpenAPI::default()));
            self.spec_v3 = Some(spec_v3.clone());
            spec_v3
        };
        self.inner = self.inner.take().map(|a| {
            a.service(
                actix_web::web::resource(path)
                    .route(actix_web::web::get().to(SpecHandlerV3(spec_v3.clone()))),
            )
        });
        self
    }

    /// Calls the given function with `App` and JSON `Value` representing your API
    /// specification **built until now**.
    ///
    /// **NOTE:** Unlike `with_json_spec_at`, this only has the API spec built until
    /// this function call. Any route handler added after this call won't affect the
    /// spec. So, it's important to call this function after adding all route handlers.
    pub fn with_raw_json_spec<F>(self, mut call: F) -> Self
    where
        F: FnMut(Self, serde_json::Value) -> Self,
    {
        let spec = serde_json::to_value(&*self.spec.read().unwrap()).expect("generating json spec");
        call(self, spec)
    }

    #[cfg(feature = "v3")]
    /// Calls the given function with `App` and JSON `Value` representing your API
    /// v2 specification **built until now** which is converted to v3.
    ///
    /// **NOTE:** Unlike `with_json_spec_at`, this only has the API spec built until
    /// this function call. Any route handler added after this call won't affect the
    /// spec. So, it's important to call this function after adding all route handlers.
    pub fn with_raw_json_spec_v3<F>(self, mut call: F) -> Self
    where
        F: FnMut(Self, serde_json::Value) -> Self,
    {
        let v3 = paperclip_core::v3::openapiv2_to_v3(self.spec.read().unwrap().clone());
        let spec = serde_json::to_value(v3).expect("generating json spec");
        call(self, spec)
    }

    /// Exposes the previously built JSON specification with Swagger UI at the given path
    ///
    /// **NOTE:** you **MUST** call with_json_spec_at before calling this function
    #[cfg(feature = "swagger-ui")]
    pub fn with_swagger_ui_at(mut self, path: &str) -> Self {
        let spec_path = self.spec_path.clone().expect(
            "Specification not set, be sure to call `with_json_spec_at` before this function",
        );

        let path: String = path.into();
        // Grab any file request from the documentation UI path and fetch it from SWAGGER_DIST
        // E.g: js, html, svg and etc.
        let regex_path = format!("{}/{{filename:.*}}", path);

        self.inner = self.inner.take().map(|a| {
            a.service(
                actix_web::web::resource([regex_path.to_owned(), path.clone()]).route(
                    actix_web::web::get().to(move |request: HttpRequest| {
                        let path = path.clone();
                        let spec_path = spec_path.clone();
                        async move {
                            let filename = request.match_info().query("filename");
                            if filename.is_empty() && request.query_string().is_empty() {
                                let redirect_url = format!("{}/index.html?url={}", path, spec_path);
                                HttpResponse::PermanentRedirect()
                                    .append_header(("Location", redirect_url))
                                    .finish()
                            } else {
                                let mut response = HttpResponse::Ok().body(
                                    SWAGGER_DIST
                                        .get_file(filename)
                                        .unwrap_or_else(|| {
                                            panic!("Failed to get file {}", filename)
                                        })
                                        .contents(),
                                );
                                if let Some(guess_result) = mime_guess::from_path(filename).first()
                                {
                                    if let Ok(header) =
                                        actix_web::http::header::HeaderValue::from_str(
                                            guess_result.essence_str(),
                                        )
                                    {
                                        response
                                            .headers_mut()
                                            .insert(actix_web::http::header::CONTENT_TYPE, header);
                                    }
                                }
                                response
                            }
                        }
                    }),
                ),
            )
        });
        self
    }

    /// Exposes the previously built JSON specification with RapiDoc at the given path
    ///
    /// **NOTE:** you **MUST** call with_json_spec_at before calling this function
    #[cfg(feature = "rapidoc")]
    pub fn with_rapidoc_at(mut self, path: &str) -> Self {
        let spec_path = self.spec_path.clone().expect(
            "Specification not set, be sure to call `with_json_spec_at` before this function",
        );

        let path: String = path.trim_end_matches('/').into();

        let rapidoc = RAPIDOC
            .get_file("index.html")
            .and_then(|file| file.contents_utf8())
            .unwrap_or_else(|| panic!("Failed to get file RapiDoc UI"));
        let mut tt = TinyTemplate::new();
        tt.add_template("index.html", rapidoc).unwrap();

        async fn rapidoc_handler(
            data: actix_web::web::Data<(TinyTemplate<'_>, String)>,
        ) -> Result<HttpResponse, Error> {
            let data = data.into_inner();
            let (tmpl, spec_path) = data.as_ref();
            let ctx = serde_json::json!({ "spec_url": spec_path });
            let s = tmpl.render("index.html", &ctx).map_err(|_| {
                actix_web::error::ErrorInternalServerError("Error rendering RapiDoc documentation")
            })?;
            Ok(HttpResponse::Ok().content_type("text/html").body(s))
        }

        self.inner = self.inner.take().map(|a| {
            a.app_data(actix_web::web::Data::new((tt, spec_path)))
                .service(
                    actix_web::web::resource(format!("{}/index.html", path))
                        .route(actix_web::web::get().to(rapidoc_handler)),
                )
                .service(
                    actix_web::web::resource(path).route(actix_web::web::get().to(rapidoc_handler)),
                )
        });
        self
    }

    /// Builds and returns the `actix_web::App`.
    pub fn build(self) -> actix_web::App<T> {
        #[cfg(feature = "v3")]
        if let Some(v3) = self.spec_v3 {
            let mut v3 = v3.write().unwrap();
            *v3 = paperclip_core::v3::openapiv2_to_v3(self.spec.read().unwrap().clone());
        }
        self.inner.expect("missing app?")
    }

    /// Trim's the Api base path from the start of all method paths.
    /// **NOTE:** much like `with_raw_json_spec` this only has the API spec built until
    /// this function call. Any route handler added after this call won't have the base path trimmed.
    /// So, it's important to call this function after adding all route handlers.
    pub fn trim_base_path(self) -> Self {
        {
            let mut spec = self.spec.write().unwrap();
            let base_path = spec.base_path.clone().unwrap_or_default();
            spec.paths = spec.paths.iter().fold(BTreeMap::new(), |mut i, (k, v)| {
                i.insert(
                    k.trim_start_matches(base_path.as_str()).to_string(),
                    v.clone(),
                );
                i
            });
        }
        self
    }

    /// Updates the underlying spec with definitions and operations from the given factory.
    fn update_from_mountable<F>(&mut self, factory: &mut F)
    where
        F: Mountable,
    {
        let mut api = self.spec.write().unwrap();
        api.definitions.extend(factory.definitions().into_iter());
        SecurityScheme::append_map(
            factory.security_definitions(),
            &mut api.security_definitions,
        );
        factory.update_operations(&mut api.paths);
        if cfg!(feature = "normalize") {
            for map in api.paths.values_mut() {
                map.normalize();
            }
        }
    }
}

#[derive(Clone)]
struct SpecHandler(Arc<RwLock<DefaultApiRaw>>);

impl actix_web::dev::Handler<()> for SpecHandler {
    type Output = Result<HttpResponse, Error>;
    type Future = Ready<Self::Output>;

    fn call(&self, _: ()) -> Self::Future {
        fut_ok(HttpResponse::Ok().json(&*self.0.read().unwrap()))
    }
}

#[cfg(feature = "v3")]
#[derive(Clone)]
struct SpecHandlerV3(Arc<RwLock<openapiv3::OpenAPI>>);

#[cfg(feature = "v3")]
impl actix_web::dev::Handler<()> for SpecHandlerV3 {
    type Output = Result<HttpResponse, Error>;
    type Future = Ready<Self::Output>;

    fn call(&self, _: ()) -> Self::Future {
        fut_ok(HttpResponse::Ok().json(&*self.0.read().unwrap()))
    }
}
