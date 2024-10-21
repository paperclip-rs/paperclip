#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        json_grpc_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the JsonGrpc resource
pub fn configure<T: server::JsonGrpc + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nodes/{node}/jsongrpc/{method}")
                .name("put_node_jsongrpc")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_jsongrpc::<T, A>))
       );


}











async fn put_node_jsongrpc<T: server::JsonGrpc + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<serde_json::Value>, RestError<crate::models::RestJsonError>> {
    T::put_node_jsongrpc(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


