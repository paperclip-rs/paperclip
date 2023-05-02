#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        watches_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Watches resource
pub fn configure<T: server::Watches + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("get_watch_volume")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_watch_volume::<T, A>))
       )

       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("put_watch_volume")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_watch_volume::<T, A>))
       )

       .service(
            actix_web::web::resource("/watches/volumes/{volume_id}")
                .name("del_watch_volume")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_watch_volume::<T, A>))
       );


}






#[derive(serde::Deserialize)]
struct put_watch_volumeQueryParams {

    
    #[serde(rename = "callback")]
    pub callback: String,

}



#[derive(serde::Deserialize)]
struct del_watch_volumeQueryParams {

    
    #[serde(rename = "callback")]
    pub callback: String,

}








async fn get_watch_volume<T: server::Watches + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<Vec<crate::models::RestWatch>>, RestError<crate::models::RestJsonError>> {
    T::get_watch_volume(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_watch_volume<T: server::Watches + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>, query: Query<put_watch_volumeQueryParams>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::put_watch_volume(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.callback)).await.map(Json)
}




async fn del_watch_volume<T: server::Watches + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>, query: Query<del_watch_volumeQueryParams>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::del_watch_volume(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.callback)).await.map(Json)
}


