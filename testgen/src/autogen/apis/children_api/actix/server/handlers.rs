#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        children_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Children resource
pub fn configure<T: server::Children + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nexuses/{nexus_id}/children")
                .name("get_nexus_children")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_nexus_children::<T, A>))
       )

       .service(
            actix_web::web::resource("/nexuses/{nexus_id}/children/{child_id}")
                .name("get_nexus_child")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_nexus_child::<T, A>))
       )

       .service(
            actix_web::web::resource("/nexuses/{nexus_id}/children/{child_id}")
                .name("put_nexus_child")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_nexus_child::<T, A>))
       )

       .service(
            actix_web::web::resource("/nexuses/{nexus_id}/children/{child_id}")
                .name("del_nexus_child")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_nexus_child::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/children")
                .name("get_node_nexus_children")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_nexus_children::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}")
                .name("get_node_nexus_child")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_nexus_child::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}")
                .name("put_node_nexus_child")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_nexus_child::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}")
                .name("del_node_nexus_child")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_nexus_child::<T, A>))
       );


}

























async fn get_nexus_children<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<Vec<crate::models::Child>>, RestError<crate::models::RestJsonError>> {
    T::get_nexus_children(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, String>) -> Result<Json<crate::models::Child>, RestError<crate::models::RestJsonError>> {
    T::get_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, String>) -> Result<Json<crate::models::Child>, RestError<crate::models::RestJsonError>> {
    T::put_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_node_nexus_children<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid>) -> Result<Json<Vec<crate::models::Child>>, RestError<crate::models::RestJsonError>> {
    T::get_node_nexus_children(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_node_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid, String>) -> Result<Json<crate::models::Child>, RestError<crate::models::RestJsonError>> {
    T::get_node_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid, String>) -> Result<Json<crate::models::Child>, RestError<crate::models::RestJsonError>> {
    T::put_node_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_node_nexus_child<T: server::Children + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, uuid::Uuid, String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_node_nexus_child(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


