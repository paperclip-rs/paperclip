#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

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
#[derive(Default, Deserialize, Serialize)]
pub struct Counter {
    name: String,
    count: u32,
}

#[test]
fn test_app() {
    #[api_v2_operation]
    fn echo_counter(body: web::Json<Counter>) -> web::Json<Counter> {
        body
    }

    #[api_v2_operation]
    fn some_counter() -> web::Json<Counter> {
        web::Json(Counter::default())
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                // .service(web::resource("/all-methods-echo").to(echo_counter))
                .service(web::resource("/post-echo").route(web::post().to(echo_counter)))
                .service(web::resource("/get-counter").route(web::get().to(some_counter)))
                .build()
        },
        |addr| {
            let mut resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                &mut resp,
                json!(
                {
                  "definitions": {
                    "Counter": {
                      "properties": {
                        "count": {
                          "type": "integer",
                          "format": "int32"
                        },
                        "name": {
                          "type": "string"
                        }
                      }
                    }
                  },
                  "paths": {
                    "/get-counter": {
                      "get": {
                        "responses": {}
                      }
                    },
                    "/post-echo": {
                      "post": {
                        "parameters": [{
                          "in": "body",
                          "name": "body",
                          "required": true,
                          "schema": {
                            "$ref": "#/definitions/Counter"
                          }
                        }],
                        "responses": {}
                      }
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
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

fn check_json(resp: &mut reqwest::Response, expected: serde_json::Value) {
    assert_eq!(resp.status().as_u16(), 200);
    let json = resp.json::<serde_json::Value>().expect("json error");

    if json != expected {
        panic!(
            "assertion failed:
  left: {}

 right: {}
",
            json.to_string(),
            expected.to_string()
        )
    }
}
