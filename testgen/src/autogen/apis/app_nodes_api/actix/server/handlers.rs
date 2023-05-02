#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        app_nodes_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the AppNodes resource
pub fn configure<T: server::AppNodes + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/app-nodes")
                .name("get_app_nodes")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_app_nodes::<T, A>))
       )

       .service(
            actix_web::web::resource("/app-nodes/{app_node_id}")
                .name("get_app_node")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_app_node::<T, A>))
       )

       .service(
            actix_web::web::resource("/app-nodes/{app_node_id}")
                .name("register_app_node")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(register_app_node::<T, A>))
       )

       .service(
            actix_web::web::resource("/app-nodes/{app_node_id}")
                .name("deregister_app_node")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(deregister_app_node::<T, A>))
       );


}




#[derive(serde::Deserialize)]
struct get_app_nodesQueryParams {

    
    #[serde(rename = "max_entries")]
    pub max_entries: isize,

    
    #[serde(rename = "starting_token", default, skip_serializing_if = "Option::is_none")]
    pub starting_token: Option<isize>,

}














async fn get_app_nodes<T: server::AppNodes + 'static, A: FromRequest + 'static>(_token: A, query: Query<get_app_nodesQueryParams>) -> Result<Json<crate::models::AppNodes>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::get_app_nodes(crate::apis::actix_server::Query(query.max_entries, query.starting_token)).await.map(Json)
}




async fn get_app_node<T: server::AppNodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<crate::models::AppNode>, RestError<crate::models::RestJsonError>> {
    T::get_app_node(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn register_app_node<T: server::AppNodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::register_app_node(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn deregister_app_node<T: server::AppNodes + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::deregister_app_node(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}


