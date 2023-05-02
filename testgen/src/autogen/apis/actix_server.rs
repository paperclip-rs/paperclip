use actix_web::http::StatusCode;
use actix_web::{web::ServiceConfig, FromRequest, HttpResponse, ResponseError};
use serde::Serialize;
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops,
};












pub use crate::apis::app_nodes_api::actix::server::AppNodes;







pub use crate::apis::block_devices_api::actix::server::BlockDevices;





















pub use crate::apis::children_api::actix::server::Children;







pub use crate::apis::json_grpc_api::actix::server::JsonGrpc;























pub use crate::apis::nexuses_api::actix::server::Nexuses;



















pub use crate::apis::nodes_api::actix::server::Nodes;



















pub use crate::apis::pools_api::actix::server::Pools;































pub use crate::apis::replicas_api::actix::server::Replicas;



















pub use crate::apis::snapshots_api::actix::server::Snapshots;







pub use crate::apis::specs_api::actix::server::Specs;

































pub use crate::apis::volumes_api::actix::server::Volumes;











pub use crate::apis::watches_api::actix::server::Watches;






/// Rest Error wrapper with a status code and a JSON error
/// Note: Only a single error type for each handler is supported at the moment
pub struct RestError<T: Debug + Serialize> {
    status_code: StatusCode,
    error_response: T,
}

impl<T: Debug + Serialize> RestError<T> {
    pub fn new(status_code: StatusCode, error_response: T) -> Self {
        Self {
            status_code,
            error_response
        }
    }
}

impl<T: Debug + Serialize> Debug for RestError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RestError")
            .field("status_code", &self.status_code)
            .field("error_response", &self.error_response)
            .finish()
    }
}

impl<T: Debug + Serialize> Display for RestError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<T: Debug + Serialize> ResponseError for RestError<T> {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code).json(&self.error_response)
    }
}

/// 204 Response with no content
#[derive(Default)]
pub(crate) struct NoContent;

impl From<actix_web::web::Json<()>> for NoContent {
    fn from(_: actix_web::web::Json<()>) -> Self {
        NoContent {}
    }
}
impl From<()> for NoContent {
    fn from(_: ()) -> Self {
        NoContent {}
    }
}
impl actix_web::Responder for NoContent {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        actix_web::HttpResponse::NoContent().finish()
    }
}

/// Wrapper type used as tag to easily distinguish the 3 different parameter types:
/// 1. Path 2. Query 3. Body
/// Example usage:
/// fn delete_resource(Path((p1, p2)): Path<(String, u64)>) { ... }
pub struct Path<T>(pub T);

impl<T> Path<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Path<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Path<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Wrapper type used as tag to easily distinguish the 3 different parameter types:
/// 1. Path 2. Query 3. Body
/// Example usage:
/// fn delete_resource(Path((p1, p2)): Path<(String, u64)>) { ... }
pub struct Query<T>(pub T);

impl<T> Query<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Query<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Query<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Wrapper type used as tag to easily distinguish the 3 different parameter types:
/// 1. Path 2. Query 3. Body
/// Example usage:
/// fn delete_resource(Path((p1, p2)): Path<(String, u64)>) { ... }
pub struct Body<T>(pub T);

impl<T> Body<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> AsRef<T> for Body<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::Deref for Body<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for Body<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Configure all actix server handlers
pub fn configure<T: AppNodes + BlockDevices + Children + JsonGrpc + Nexuses + Nodes + Pools + Replicas + Snapshots + Specs + Volumes + Watches + 'static>(cfg: &mut ServiceConfig) {











    crate::apis::app_nodes_api::actix::server::handlers::configure::<T, A>(cfg);







    crate::apis::block_devices_api::actix::server::handlers::configure::<T, A>(cfg);





















    crate::apis::children_api::actix::server::handlers::configure::<T, A>(cfg);







    crate::apis::json_grpc_api::actix::server::handlers::configure::<T, A>(cfg);























    crate::apis::nexuses_api::actix::server::handlers::configure::<T, A>(cfg);



















    crate::apis::nodes_api::actix::server::handlers::configure::<T, A>(cfg);



















    crate::apis::pools_api::actix::server::handlers::configure::<T, A>(cfg);































    crate::apis::replicas_api::actix::server::handlers::configure::<T, A>(cfg);



















    crate::apis::snapshots_api::actix::server::handlers::configure::<T, A>(cfg);







    crate::apis::specs_api::actix::server::handlers::configure::<T, A>(cfg);

































    crate::apis::volumes_api::actix::server::handlers::configure::<T, A>(cfg);











    crate::apis::watches_api::actix::server::handlers::configure::<T, A>(cfg);





}

/// Used with Query to deserialize into Vec<I>.
#[allow(dead_code)]
pub(crate) fn deserialize_stringified_list<'de, D, I>(
    deserializer: D,
) -> std::result::Result<Vec<I>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    I: serde::de::DeserializeOwned,
{
    struct StringVecVisitor<I>(std::marker::PhantomData<I>);

    impl<'de, I> serde::de::Visitor<'de> for StringVecVisitor<I>
    where
        I: serde::de::DeserializeOwned,
    {
        type Value = Vec<I>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let mut list = Vec::new();
            if !v.is_empty() {
                for item in v.split(',') {
                    let item = I::deserialize(serde::de::IntoDeserializer::into_deserializer(item))?;
                    list.push(item);
                }
            }
            Ok(list)
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<I>))
}

/// Used with Query to deserialize into Option<Vec<I>>.
#[allow(dead_code)]
pub(crate) fn deserialize_option_stringified_list<'de, D, I>(
    deserializer: D,
) -> std::result::Result<Option<Vec<I>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    I: serde::de::DeserializeOwned,
{
    let list = deserialize_stringified_list(deserializer)?;
    match list.is_empty() {
        true => Ok(None),
        false => Ok(Some(list)),
    }
}