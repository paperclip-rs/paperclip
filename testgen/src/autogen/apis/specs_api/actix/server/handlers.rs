#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        specs_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Specs resource
pub fn configure<T: server::Specs + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/specs")
                .name("get_specs")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_specs::<T, A>))
       );


}











async fn get_specs<T: server::Specs + 'static, A: FromRequest + 'static>(_token: A) -> Result<Json<crate::models::Specs>, RestError<crate::models::RestJsonError>> {
    T::get_specs().await.map(Json)
}


