#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

use actix_rt::System;
use actix_service::NewService;
use actix_web::dev::{MessageBody, Payload, ServiceRequest, ServiceResponse};
use actix_web::{App, Error, FromRequest, HttpRequest, HttpServer, Responder};
use futures::Future;
use paperclip::actix::{api_v2_operation, api_v2_schema, web, OpenApiExt};
use parking_lot::Mutex;

use std::collections::{BTreeMap, HashSet};
use std::sync::mpsc;
use std::thread;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref PORTS: Mutex<HashSet<u16>> = Mutex::new(HashSet::new());
}

#[api_v2_schema]
#[derive(Deserialize, Serialize)]
enum PetClass {
    Dog,
    Cat,
    EverythingElse,
}

#[api_v2_schema]
#[derive(Deserialize, Serialize)]
struct Pet {
    name: String,
    class: PetClass,
    id: Option<u64>,
}

#[test]
fn test_simple_app() {
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
    fn some_pet(_data: web::Data<String>) -> web::Json<Pet> {
        unimplemented!();
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
                        "class": {
                          "enum": ["Dog", "Cat", "EverythingElse"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "type": "string"
                        }
                      },
                      "required":["class", "name"]
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
    fn get_resource_2(_p: web::Path<String>) -> String {
        unimplemented!();
    }

    #[api_v2_operation]
    fn get_known_badge_1(_p: web::Path<KnownResourceBadge>, _q: web::Query<BadgeParams>) -> String {
        unimplemented!();
    }

    #[api_v2_operation]
    fn get_known_badge_2(_p: web::Path<(String, String)>, _q: web::Query<BadgeParams>) -> String {
        unimplemented!();
    }

    #[api_v2_operation]
    fn post_badge_1(
        _p: web::Path<KnownResourceBadge>,
        _q: web::Query<BadgeParams>,
        _f: web::Form<BadgeBody>,
    ) -> String {
        unimplemented!();
    }

    #[api_v2_operation]
    fn post_badge_2(_p: web::Path<(String, String)>, _b: web::Json<BadgeBody>) -> String {
        unimplemented!();
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
                        )
                        .service(
                            web::resource("/v2/{resource}").route(web::get().to(get_resource_2)),
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
                    "/api/v2/{resource}": {
                      "get": {
                        "responses": {}
                      },
                      "parameters": [{
                        "in": "path",
                        "name": "resource",
                        "required": true,
                        "type": "string"
                      }]
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

#[test]
fn test_map_in_out() {
    #[api_v2_schema]
    #[derive(Serialize)]
    struct Image {
        data: String,
    }

    #[api_v2_operation]
    fn some_images() -> web::Json<BTreeMap<String, Image>> {
        unimplemented!();
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/images").route(web::get().to(some_images)))
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
                    "Image": {
                      "properties": {
                        "data": {
                          "type": "string"
                        }
                      },
                      "required": ["data"]
                    }
                  },
                  "paths": {
                    "/images": {
                      "get": {
                        "responses": {
                          "200": {
                            "schema": {
                              "additionalProperties": {
                                "$ref": "#/definitions/Image"
                              },
                              "type": "object"
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
fn test_list_in_out() {
    #[api_v2_schema]
    #[derive(Serialize, Deserialize)]
    enum Sort {
        Asc,
        Desc,
    }

    #[api_v2_schema]
    #[derive(Serialize, Deserialize)]
    struct Params {
        sort: Option<Sort>,
        limit: Option<u16>,
    }

    #[api_v2_operation]
    fn get_pets(_q: web::Query<Params>) -> web::Json<Vec<Pet>> {
        unimplemented!();
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/pets").route(web::get().to(get_pets)))
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
                        "class": {
                          "enum": ["Dog", "Cat", "EverythingElse"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "type": "string"
                        }
                      },
                      "required":["class", "name"]
                    }
                  },
                  "paths": {
                    "/pets": {
                      "get": {
                        "responses": {
                          "200": {
                            "schema": {
                              "type": "array",
                              "items": {
                                "$ref": "#/definitions/Pet"
                              }
                            }
                          }
                        }
                      },
                      "parameters": [{
                        "format": "int32",
                        "in": "query",
                        "name": "limit",
                        "required": false,
                        "type": "integer"
                      }, {
                        "enum": ["Asc", "Desc"],
                        "in": "query",
                        "name": "sort",
                        "required": false,
                        "type": "string"
                      }],
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
#[allow(unreachable_code)]
fn test_impl_traits() {
    #[api_v2_operation]
    fn index() -> impl Responder {
        unimplemented!();
    }

    #[api_v2_schema]
    #[derive(Serialize, Deserialize)]
    struct Params {
        limit: Option<u16>,
    }

    #[api_v2_operation]
    fn get_pets(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Item = web::Json<Vec<Pet>>, Error = ()> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(());
        }

        futures::future::err(())
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/").route(web::get().to(index)))
                .service(web::resource("/pets").route(web::get().to_async(get_pets)))
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
                        "class": {
                          "enum": ["Dog", "Cat", "EverythingElse"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "type": "string"
                        }
                      },
                      "required":["class", "name"]
                    }
                  },
                  "paths": {
                    "/": {
                      "get": {
                        "responses": {}
                      }
                    },
                    "/pets": {
                      "get": {
                        "responses": {
                          "200": {
                            "schema": {
                              "type": "array",
                              "items": {
                                "$ref": "#/definitions/Pet"
                              }
                            }
                          }
                        }
                      },
                      "parameters": [{
                        "format": "int32",
                        "in": "query",
                        "name": "limit",
                        "required": false,
                        "type": "integer"
                      }]
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test] // issue #71
fn test_multiple_method_routes() {
    #[api_v2_operation]
    fn test_get() -> String {
        "get".into()
    }

    #[api_v2_operation]
    fn test_post() -> String {
        "post".into()
    }

    fn test_app<F, T, B>(f: F)
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
    {
        run_and_check_app(f, |addr| {
            let mut resp = CLIENT
                .get(&format!("http://{}/foo", addr))
                .send()
                .expect("request failed?");
            assert_eq!(resp.status().as_u16(), 200);
            assert_eq!(resp.text().unwrap(), "get");

            let mut resp = CLIENT
                .post(&format!("http://{}/foo", addr))
                .send()
                .expect("request failed?");
            assert_eq!(resp.status().as_u16(), 200);
            assert_eq!(resp.text().unwrap(), "post");

            let mut resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                &mut resp,
                json!({
                  "definitions": {},
                  "paths": {
                    "/foo": {
                      "get": {
                        "responses": {},
                      },
                      "post": {
                        "responses": {},
                      },
                    }
                  },
                  "swagger": "2.0",
                }),
            );
        });
    }

    test_app(|| {
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .route("/foo", web::get().to(test_get))
            .route("/foo", web::post().to(test_post))
            .build()
    });

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.route("/foo", web::get().to(test_get))
            .route("/foo", web::post().to(test_post));
    }

    test_app(|| {
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .service(web::scope("").configure(config))
            .build()
    });

    test_app(|| {
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .configure(config)
            .build()
    });
}

#[test]
fn test_custom_extractor_empty_schema() {
    #[api_v2_schema(empty)]
    struct SomeUselessThing<T>(T);

    impl FromRequest for SomeUselessThing<String> {
        type Error = Error;
        type Future = Result<Self, Self::Error>;
        type Config = ();

        fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
            Ok(SomeUselessThing(String::from("booya")))
        }
    }

    #[api_v2_operation]
    fn index(_req: HttpRequest, _payload: String, _thing: SomeUselessThing<String>) -> () {
        unimplemented!();
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/").route(web::get().to(index)))
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
                    "/": {
                      "get": {
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
            if !PORTS.lock().insert(port) {
                continue;
            }

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
