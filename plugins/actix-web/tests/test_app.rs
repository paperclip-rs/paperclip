#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use actix_rt::System;
use actix_service::NewService;
use actix_web::dev::{MessageBody, ServiceRequest, ServiceResponse};
use actix_web::{App, Error, HttpServer};
use paperclip_actix::{api_v2_operation, api_v2_schema, web, OpenApiExt};

use std::sync::mpsc;
use std::thread;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

#[api_v2_schema]
#[derive(Deserialize, Serialize)]
pub struct Counter {
    #[serde(rename = "counterName")]
    name: String,
    count: u32,
}

fn run_and_check_app<F, G, T, B, U>(factory: F, check: G) -> U
where
    F: Fn() -> App<T, B> + Clone + Send + Sync + 'static,
    B: MessageBody + 'static,
    T: NewService<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        > + 'static,
    G: Fn(String) -> U,
{
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        let sys = System::new("test");
        for port in 3000..30000 {
            let addr = format!("127.0.0.1:{}", port);
            println!("Trying to bind to {}", addr);
            let server = match HttpServer::new(factory.clone()).bind(&addr) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let s = server.start();
            tx.send((s, addr)).unwrap();
            sys.run().expect("system error?");
            return;
        }

        unreachable!("No ports???");
    });

    let (server, addr) = rx.recv().unwrap();
    let ret = check(addr);
    server.stop(false);
    ret
}

#[test]
fn test_app() {
    #[api_v2_operation]
    fn test(body: web::Json<Counter>) -> web::Json<Counter> {
        body
    }

    run_and_check_app(
        || {
            App::new()
                .record_operations()
                .with_json_spec_at("/swagger.json")
                .service(web::resource("/test1").to(test))
                .build()
        },
        |addr| {
            let mut resp = CLIENT
                .get(&format!("http://{}/swagger.json", addr))
                .send()
                .expect("request failed?");
            assert_eq!(resp.status().as_u16(), 200);
            let json = resp.json::<serde_json::Value>().expect("json error");
            assert_eq!(json["swagger"], "2.0");
        },
    );
}
