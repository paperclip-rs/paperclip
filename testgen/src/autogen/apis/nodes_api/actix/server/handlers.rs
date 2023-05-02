#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        nodes_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Nodes resource
pub fn configure<T: server::Nodes + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nodes")
                .name("get_nodes")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_nodes::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}")
                .name("get_node")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}/cordon/{label}")
                .name("put_node_cordon")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_cordon::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}/cordon/{label}")
                .name("delete_node_cordon")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(delete_node_cordon::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}/drain/{label}")
                .name("put_node_drain")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_drain::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}/label/{key}={value}")
                .name("put_node_label")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_label::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{id}/label/{key}")
                .name("delete_node_label")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(delete_node_label::<T, A>))
       );


}




#[derive(serde::Deserialize)]
struct get_nodesQueryParams {

    
    #[serde(rename = "node_id", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,

}











#[derive(serde::Deserialize)]
struct put_node_labelQueryParams {

    
    #[serde(rename = "overwrite", default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,

}










async fn get_nodes<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, query: Query<get_nodesQueryParams>) -> Result<Json<Vec<crate::models::Node>>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::get_nodes(crate::apis::actix_server::Query(query.node_id)).await.map(Json)
}




async fn get_node<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    T::get_node(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_cordon<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    T::put_node_cordon(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn delete_node_cordon<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    T::delete_node_cordon(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_drain<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    T::put_node_drain(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_label<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, String>, query: Query<put_node_labelQueryParams>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::put_node_label(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.overwrite)).await.map(Json)
}




async fn delete_node_label<T: server::Nodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Node>, RestError<crate::models::RestJsonError>> {
    T::delete_node_label(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


