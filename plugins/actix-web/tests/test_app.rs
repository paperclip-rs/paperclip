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

#[test]
fn test_simple_app() {
    #[api_v2_schema]
    #[derive(Default, Deserialize, Serialize)]
    struct Pet {
        name: String,
        id: u64,
    }

    #[api_v2_schema]
    #[derive(Deserialize, Serialize)]
    struct SomeResource {
        name: String,
    }

    #[api_v2_operation]
    fn echo_pet(body: web::Json<Pet>) -> web::Json<Pet> {
        body
    }

    #[api_v2_operation]
    fn some_pet() -> web::Json<Pet> {
        web::Json(Pet::default())
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/echo").route(web::post().to(echo_pet)))
            .service(web::resource("/random").to(some_pet));
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::scope("/api").configure(config))
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
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "type": "string"
                        }
                      },
                      "required":["id","name"]
                    }
                  },
                  "paths": {
                    "/api/echo": {
                      "parameters": [{
                        "in": "body",
                        "name": "body",
                        "required": true,
                        "schema": {
                          "$ref": "#/definitions/Pet"
                        }
                      }],
                      "post": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/random": {
                      "delete": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "get": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "head": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "options": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "patch": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "post": {
                        "responses": {
                          "200": {
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "put": {
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
#[allow(dead_code)]
fn test_params() {
    #[api_v2_schema]
    #[derive(Deserialize)]
    struct KnownResourceBadge {
        resource: String,
        name: String,
    }

    #[api_v2_schema]
    #[derive(Deserialize)]
    struct BadgeParams {
        res: Option<u16>,
        color: String,
    }

    #[api_v2_schema]
    #[derive(Deserialize)]
    struct BadgeBody {
        data: String,
    }

    #[api_v2_operation]
    fn get_known_badge_1(_p: web::Path<KnownResourceBadge>, _q: web::Query<BadgeParams>) -> String {
        String::new()
    }

    #[api_v2_operation]
    fn get_known_badge_2(_p: web::Path<(String, String)>, _q: web::Query<BadgeParams>) -> String {
        String::new()
    }

    #[api_v2_operation]
    fn post_badge_1(
        _p: web::Path<KnownResourceBadge>,
        _q: web::Query<BadgeParams>,
        _f: web::Form<BadgeBody>,
    ) -> String {
        String::new()
    }

    #[api_v2_operation]
    fn post_badge_2(_p: web::Path<(String, String)>, _b: web::Json<BadgeBody>) -> String {
        String::new()
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(
                    web::scope("/api")
                        .service(
                            web::resource("/v1/{resource}/v/{name}")
                                .route(web::Route::new().to(get_known_badge_1))
                                .route(web::post().to(post_badge_1)),
                        )
                        .service(
                            web::resource("/v2/{resource}/v/{name}")
                                .route(web::get().to(get_known_badge_2))
                                .route(web::post().to(post_badge_2)),
                        ),
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
                  "definitions": {
                    "BadgeBody": {
                      "properties": {
                        "data": {
                          "type": "string"
                        }
                      },
                      "required":["data"]
                    }
                  },
                  "paths": {
                    "/api/v1/{resource}/v/{name}": {
                      "delete": {
                        "responses": {}
                      },
                      "get": {
                        "responses": {}
                      },
                      "head": {
                        "responses": {}
                      },
                      "options": {
                        "responses": {}
                      },
                      "parameters": [{
                        "in": "query",
                        "name": "color",
                        "required": true,
                        "type": "string"
                      }, {
                        "in": "path",
                        "name": "name",
                        "required": true,
                        "type": "string"
                      }, {
                        "format": "int32",
                        "in": "query",
                        "name": "res",
                        "required": false,
                        "type": "integer"
                      }, {
                        "in": "path",
                        "name": "resource",
                        "required": true,
                        "type": "string"
                      }],
                      "patch": {
                        "responses": {}
                      },
                      "post": {
                        "parameters": [{
                          "in": "formData",
                          "name": "data",
                          "required": true,
                          "type": "string"
                        }],
                        "responses": {}
                      },
                      "put": {
                        "responses": {}
                      }
                    },
                    "/api/v2/{resource}/v/{name}": {
                      "get": {
                        "parameters": [{
                          "in": "query",
                          "name": "color",
                          "required": true,
                          "type": "string"
                        }, {
                          "format": "int32",
                          "in": "query",
                          "name": "res",
                          "required": false,
                          "type": "integer"
                        }],
                        "responses": {}
                      },
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
                      "post": {
                        "parameters": [{
                          "in": "body",
                          "name": "body",
                          "required": true,
                          "schema": {
                            "$ref": "#/definitions/BadgeBody"
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
            let server = match HttpServer::new(factory.clone()).bind(&addr) {
                Ok(srv) => {
                    println!("Bound to {}", addr);
                    srv
                }
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
