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
pub struct Pet {
    name: String,
    id: u64,
}

#[test]
fn test_simple_app() {
    #[api_v2_operation]
    fn echo_pet(body: web::Json<Pet>) -> web::Json<Pet> {
        body
    }

    #[api_v2_operation]
    fn some_pet() -> web::Json<Pet> {
        web::Json(Pet::default())
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                // .service(web::resource("/all-methods-test").to(test))
                .service(web::resource("/echo").route(web::post().to(echo_pet)))
                .service(web::resource("/random").route(web::get().to(some_pet)))
                .build()
        },
        |addr| {
            let mut resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                &mut resp,
                json!({
                  "definitions": {
                    "Pet": {
                      "properties": {
                        "id": {
                          "type": "integer",
                          "format": "int64"
                        },
                        "name": {
                          "type": "string"
                        }
                      }
                    }
                  },
                  "paths": {
                    "/random": {
                      "get": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/echo": {
                      "post": {
                        "parameters": [{
                          "in": "body",
                          "name": "body",
                          "required": true,
                          "schema": {
                            "$ref": "#/definitions/Pet"
                          }
                        }],
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_path_param_struct() {
    #[api_v2_schema]
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct KnownResourceBadge {
        resource: String,
        name: String,
    }

    #[api_v2_operation]
    fn get_known_badge(_p: web::Path<KnownResourceBadge>) -> String {
        String::from("some base64 data")
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(
                    web::resource("/{resource}/v/{name}").route(web::get().to(get_known_badge)),
                )
                .build()
        },
        |addr| {
            let mut resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                &mut resp,
                json!({
                  "definitions": {},
                  "paths": {
                    "/{resource}/v/{name}": {
                      "get": {
                        "parameters": [{
                          "in": "path",
                          "name": "name",
                          "required": true,
                          "type": "string"
                        }, {
                          "in": "path",
                          "name": "resource",
                          "required": true,
                          "type": "string"
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
