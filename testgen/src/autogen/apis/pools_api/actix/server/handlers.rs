#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        pools_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Pools resource
pub fn configure<T: server::Pools + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nodes/{id}/pools")
                .name("get_node_pools")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_pools::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
                .name("get_node_pool")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_pool::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
                .name("put_node_pool")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_node_pool::<T, A>))
       )

       .service(
            actix_web::web::resource("/nodes/{node_id}/pools/{pool_id}")
                .name("del_node_pool")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_node_pool::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools")
                .name("get_pools")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_pools::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}")
                .name("get_pool")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_pool::<T, A>))
       )

       .service(
            actix_web::web::resource("/pools/{pool_id}")
                .name("del_pool")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_pool::<T, A>))
       );


}























async fn get_node_pools<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<Vec<crate::models::Pool>>, RestError<crate::models::RestJsonError>> {
    T::get_node_pools(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_node_pool<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Pool>, RestError<crate::models::RestJsonError>> {
    T::get_node_pool(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_node_pool<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<crate::models::Pool>, RestError<crate::models::RestJsonError>> {
    T::put_node_pool(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_node_pool<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String, String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_node_pool(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_pools<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A) -> Result<Json<Vec<crate::models::Pool>>, RestError<crate::models::RestJsonError>> {
    T::get_pools().await.map(Json)
}




async fn get_pool<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<crate::models::Pool>, RestError<crate::models::RestJsonError>> {
    T::get_pool(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_pool<T: server::Pools + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_pool(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


