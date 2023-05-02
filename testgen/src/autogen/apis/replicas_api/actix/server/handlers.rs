#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        replicas_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Replicas resource
pub fn configure<T: server::Replicas + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nodes/{id}/replicas")
                .name("get_node_replicas")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_replicas::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas")
                .name("get_node_pool_replicas")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_pool_replicas::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}")
                .name("get_node_pool_replica")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_pool_replica::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}")
                .name("put_node_pool_replica")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_pool_replica::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}")
                .name("del_node_pool_replica")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_pool_replica::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share")
                .name("del_node_pool_replica_share")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_pool_replica_share::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share/nvmf")
                .name("put_node_pool_replica_share")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_pool_replica_share::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}/replicas/{replica_id}")
                .name("put_pool_replica")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_pool_replica::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}/replicas/{replica_id}")
                .name("del_pool_replica")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_pool_replica::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}/replicas/{replica_id}/share")
                .name("del_pool_replica_share")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_pool_replica_share::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}/replicas/{replica_id}/share/nvmf")
                .name("put_pool_replica_share")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_pool_replica_share::<T, A>))
       )

       .service(
            actix_web::web::resource("/replicas")
                .name("get_replicas")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_replicas::<T, A>))
       )

       .service(
            actix_web::web::resource("/replicas/{id}")
                .name("get_replica")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_replica::<T, A>))
       );


}
















#[derive(serde::Deserialize)]
struct put_node_pool_replica_shareQueryParams {

    
    #[serde(rename = "allowed-hosts", default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_option_stringified_list")]
    pub allowed_hosts: Option<Vec<String>>,

}









#[derive(serde::Deserialize)]
struct put_pool_replica_shareQueryParams {

    
    #[serde(rename = "allowed-hosts", default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_option_stringified_list")]
    pub allowed_hosts: Option<Vec<String>>,

}












async fn get_node_replicas<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<Vec<crate::models::Replica>>, RestError<crate::models::RestJsonError>> {
    T::get_node_replicas(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_node_pool_replicas<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<Vec<crate::models::Replica>>, RestError<crate::models::RestJsonError>> {
    T::get_node_pool_replicas(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_node_pool_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, uuid::Uuid>) -> Result<Json<crate::models::Replica>, RestError<crate::models::RestJsonError>> {
    T::get_node_pool_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_pool_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, uuid::Uuid>) -> Result<Json<crate::models::Replica>, RestError<crate::models::RestJsonError>> {
    T::put_node_pool_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_node_pool_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_node_pool_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_node_pool_replica_share<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_node_pool_replica_share(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_pool_replica_share<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String, uuid::Uuid>, query: Query<put_node_pool_replica_shareQueryParams>) -> Result<Json<String>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::put_node_pool_replica_share(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.allowed_hosts)).await.map(Json)
}




async fn put_pool_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid>) -> Result<Json<crate::models::Replica>, RestError<crate::models::RestJsonError>> {
    T::put_pool_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_pool_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_pool_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_pool_replica_share<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_pool_replica_share(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_pool_replica_share<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid>, query: Query<put_pool_replica_shareQueryParams>) -> Result<Json<String>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::put_pool_replica_share(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.allowed_hosts)).await.map(Json)
}




async fn get_replicas<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A) -> Result<Json<Vec<crate::models::Replica>>, RestError<crate::models::RestJsonError>> {
    T::get_replicas().await.map(Json)
}




async fn get_replica<T: server::Replicas + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<crate::models::Replica>, RestError<crate::models::RestJsonError>> {
    T::get_replica(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


