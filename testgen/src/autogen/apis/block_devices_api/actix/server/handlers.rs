#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        block_devices_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the BlockDevices resource
pub fn configure<T: server::BlockDevices + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/nodes/{node}/block_devices")
                .name("get_node_block_devices")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_node_block_devices::<T, A>))
       );


}




#[derive(serde::Deserialize)]
struct get_node_block_devicesQueryParams {

    
    #[serde(rename = "all", default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,

}








async fn get_node_block_devices<T: server::BlockDevices + 'static, A: FromRequest + 'static>(_token: A, path: Path<String>, query: Query<get_node_block_devicesQueryParams>) -> Result<Json<Vec<crate::models::BlockDevice>>, RestError<crate::models::RestJsonError>> {
    let query = query.into_inner();
    T::get_node_block_devices(crate::apis::actix_server::Path(path.into_inner()), crate::apis::actix_server::Query(query.all)).await.map(Json)
}


