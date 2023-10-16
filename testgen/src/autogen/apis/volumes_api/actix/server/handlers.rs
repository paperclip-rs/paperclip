#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        volumes_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Volumes resource
pub fn configure<T: server::Volumes + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/volumes")
                .name("get_volumes")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volumes::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("get_volume")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volume::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("put_volume")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}")
                .name("del_volume")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_volume::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/replica_count/{replica_count}")
                .name("put_volume_replica_count")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume_replica_count::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/target")
                .name("put_volume_target")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume_target::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/target")
                .name("del_volume_target")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_volume_target::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/shutdown_targets")
                .name("del_volume_shutdown_targets")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_volume_shutdown_targets::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/share/{protocol}")
                .name("put_volume_share")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume_share::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes{volume_id}/share")
                .name("del_share")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_share::<T, A>))
       );


}




#[derive(serde::Deserialize)]
struct get_volumesQueryParams {

    
    #[serde(rename = "volume_id", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<uuid::Uuid>,

    
    #[serde(rename = "max_entries")]
    pub max_entries: isize,

    
    #[serde(rename = "starting_token", default, skip_serializing_if = "Option::is_none")]
    pub starting_token: Option<isize>,

}













#[derive(serde::Deserialize)]
struct del_volume_targetQueryParams {

    
    #[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

}





#[derive(serde::Deserialize)]
struct put_volume_shareQueryParams {

    
    #[serde(rename = "frontend_host", default, skip_serializing_if = "Option::is_none")]
    pub frontend_host: Option<String>,

}










async fn get_volumes<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, query: Query<get_volumesQueryParams>) -> Result<Json<crate::models::Volumes>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::get_volumes(crate::apis::actix_server::Query(query.volume_id, query.max_entries, query.starting_token)).await.map(Json)
}




async fn get_volume<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<crate::models::Volume>, RestError<crate::models::RestJsonError>> {
    T::get_volume(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_volume<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<crate::models::Volume>, RestError<crate::models::RestJsonError>> {
    T::put_volume(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_volume<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_volume(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_volume_replica_count<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, u8>) -> Result<Json<crate::models::Volume>, RestError<crate::models::RestJsonError>> {
    T::put_volume_replica_count(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_volume_target<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<crate::models::Volume>, RestError<crate::models::RestJsonError>> {
    T::put_volume_target(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_volume_target<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>, query: Query<del_volume_targetQueryParams>) -> Result<Json<crate::models::Volume>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::del_volume_target(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.force)).await.map(Json)
}




async fn del_volume_shutdown_targets<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_volume_shutdown_targets(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_volume_share<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, String>, query: Query<put_volume_shareQueryParams>) -> Result<Json<String>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::put_volume_share(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.frontend_host)).await.map(Json)
}




async fn del_share<T: server::Volumes + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_share(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


