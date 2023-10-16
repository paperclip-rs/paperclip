

pub mod block_devices_api;

pub mod children_api;

pub mod json_grpc_api;

pub mod nexuses_api;

pub mod nodes_api;

pub mod pools_api;

pub mod replicas_api;

pub mod snapshots_api;

pub mod specs_api;

pub mod volumes_api;

pub mod watches_api;



/// Actix server.
#[cfg(feature = "actix-server")]
pub mod actix_server;

#[cfg(feature = "tower-hyper")]
pub use hyper::http::StatusCode;

#[cfg(not(feature = "tower-hyper"))]
#[cfg(feature = "actix")]
pub use actix_web::http::StatusCode;

/// Url.
pub use url::Url;
/// Uuid.
pub use uuid::Uuid;

/// Encode string to use in a URL.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// Helper to convert from Vec<F> into Vec<T>.
pub trait IntoVec<T>: Sized {
    /// Performs the conversion.
    fn into_vec(self) -> Vec<T>;
}

impl<F: Into<T>, T> IntoVec<T> for Vec<F> {
    fn into_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}

/// Helper to convert from Vec<F> or Option<Vec<F>> into Option<Vec<T>>.
pub trait IntoOptVec<T>: Sized {
    /// Performs the conversion.
    fn into_opt_vec(self) -> Option<Vec<T>>;
}

impl<F: Into<T>, T> IntoOptVec<T> for Vec<F> {
    fn into_opt_vec(self) -> Option<Vec<T>> {
        Some(self.into_iter().map(Into::into).collect())
    }
}
impl<F: Into<T>, T> IntoOptVec<T> for Option<Vec<F>> {
    fn into_opt_vec(self) -> Option<Vec<T>> {
        self.map(|s| s.into_iter().map(Into::into).collect())
    }
}