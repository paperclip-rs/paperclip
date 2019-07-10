#[macro_use]
extern crate serde;

use actix_web::App;
use paperclip_actix::{api_v2_operation, api_v2_schema, web, OpenApiExt};

#[api_v2_schema]
#[derive(Deserialize, Serialize)]
pub struct Counter {
    #[serde(rename = "counterName")]
    name: String,
    count: u32,
}

#[test]
fn test_app() {
    #[api_v2_operation]
    fn test(body: web::Json<Counter>) -> web::Json<Counter> {
        body
    }

    let _app = App::new()
        .record_operations()
        .service(web::resource("/test1").to(test));
}
