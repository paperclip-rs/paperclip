use paperclip::actix::web;
use paperclip::actix::{OpenApiExt, Apiv2Schema};
use paperclip::actix::api_v2_operation;
use actix_web::{App, Error};
use std::future::Future;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
/// Pets are awesome!
pub struct Pet {
    /// Pick a good one.
    name: String,
    id: Option<u64>,
}

#[api_v2_operation]
async fn some_pet(data: web::Data<String>, pet: web::Json<Pet>) -> Result<web::Json<Pet>, Error> {
    #[allow(unreachable_code)]
    async { unimplemented!() }.await
}

#[api_v2_operation]
async fn abstract_pet<T: 'static>(data: web::Data<T>, pet: web::Json<Pet>) -> Result<web::Json<Pet>, Error> {
    #[allow(unreachable_code)]
    async { unimplemented!() }.await
}

#[actix_rt::main]
async fn main() {
    let app =
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .service(web::resource("/random")
                .route(web::post().operation(some_pet))
                .route(web::get().operation(abstract_pet::<String>))
            )
            .build();
}