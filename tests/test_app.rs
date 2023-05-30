#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

#[cfg(feature = "actix3-validator")]
extern crate actix_web_validator2 as actix_web_validator;
#[cfg(feature = "actix4-validator")]
extern crate actix_web_validator3 as actix_web_validator;

#[cfg(feature = "actix3-validator")]
extern crate validator12 as validator;
#[cfg(feature = "actix4-validator")]
extern crate validator14 as validator;

#[cfg(not(feature = "actix4"))]
extern crate actix_service1 as actix_service;
#[cfg(feature = "actix4")]
extern crate actix_service2 as actix_service;

#[cfg(feature = "actix2")]
extern crate actix_web2 as actix_web;
#[cfg(feature = "actix3")]
extern crate actix_web3 as actix_web;
#[cfg(feature = "actix4")]
extern crate actix_web4 as actix_web;

#[cfg(feature = "actix4")]
use actix_web::body::{BoxBody, MessageBody};
#[cfg(not(feature = "actix4"))]
use actix_web::dev::MessageBody;

#[cfg(feature = "actix2")]
use actix_rt1::System;
use actix_service::ServiceFactory;
#[cfg(feature = "actix4")]
use actix_web::middleware::{DefaultHeaders, Logger};
#[cfg(not(feature = "actix2"))]
use actix_web::rt::System;
use actix_web::{
    dev::{Payload, ServiceRequest, ServiceResponse},
    App, Error, FromRequest, HttpRequest, HttpServer, Responder,
};
#[cfg(any(feature = "actix3-validator", feature = "actix4-validator"))]
use actix_web_validator::{Json as ValidatedJson, Path as ValidatedPath, Query as ValidatedQuery};
use futures::future::{ok as fut_ok, ready, Future, Ready};
use once_cell::sync::Lazy;
use paperclip::{
    actix::{
        api_v2_errors, api_v2_errors_overlay, api_v2_operation, delete, get, patch, post, put, web,
        Apiv2Header, Apiv2Schema, Apiv2Security, CreatedJson, NoContent, OpenApiExt,
    },
    v2::models::{DefaultApiRaw, Info, Tag},
};
use std::sync::Mutex;
#[cfg(any(feature = "actix3-validator", feature = "actix4-validator"))]
use validator::Validate;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::PathBuf,
    sync::mpsc,
    thread,
};

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(|| reqwest::blocking::Client::new());
static PORTS: Lazy<Mutex<HashSet<u16>>> = Lazy::new(|| Mutex::new(HashSet::new()));

use uuid0_dev::Uuid;

type OptionalUuid0 = Option<uuid0_dev::Uuid>;
type OptionalUuid1 = Option<uuid1_dev::Uuid>;

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "lowercase")]
enum PetClass {
    Dog,
    Cat,
    #[serde(rename = "other")]
    EverythingElse,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
/// Pets are awesome!
struct Pet {
    /// Pick a good one.
    name: String,
    class: PetClass,
    id: Option<u64>,
    birthday: chrono_dev::NaiveDate,
    updated_on: Option<chrono_dev::NaiveDateTime>,
    #[serde(rename = "uuid0")]
    uid0: OptionalUuid0,
    #[serde(rename = "uuid1")]
    uid1: OptionalUuid1,
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            class: PetClass::EverythingElse,
            birthday: chrono_dev::NaiveDate::from_ymd_opt(2012, 3, 10)
                .expect("invalid or out-of-range date"),
            id: None,
            updated_on: None,
            uid0: None,
            uid1: None,
        }
    }
}

#[cfg(feature = "actix4")]
impl Responder for Pet {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[cfg(not(feature = "actix4"))]
impl Responder for Pet {
    type Error = Error;
    type Future = Ready<Result<actix_web::HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[test]
fn test_simple_app() {
    #[api_v2_operation]
    fn echo_pet(body: web::Json<Pet>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        fut_ok(body)
    }

    #[api_v2_operation]
    async fn echo_pet_async(body: web::Json<Pet>) -> Result<web::Json<Pet>, actix_web::Error> {
        Ok(body)
    }

    async fn inner_async_func(body: web::Json<Pet>) -> Pet {
        body.into_inner()
    }

    #[api_v2_operation]
    async fn echo_pet_async_2(body: web::Json<Pet>) -> Result<web::Json<Pet>, actix_web::Error> {
        let pet = inner_async_func(body).await;
        Ok(web::Json(pet))
    }

    #[api_v2_operation]
    fn some_pet(_data: web::Data<String>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        #[allow(unreachable_code)]
        fut_ok(unimplemented!())
    }

    #[api_v2_operation]
    async fn adopt_pet() -> Result<CreatedJson<Pet>, Error> {
        let pet: Pet = Pet::default();
        Ok(CreatedJson(pet))
    }

    #[api_v2_operation]
    async fn nothing() -> NoContent {
        NoContent
    }

    #[api_v2_operation]
    #[paperclip::actix::post("no-slash")]
    async fn path_without_slash() -> NoContent {
        NoContent
    }

    #[api_v2_operation]
    async fn path_with_param_without_slash(_p: web::Path<u32>) -> NoContent {
        NoContent
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/echo").route(web::post().to(echo_pet)))
            .service(web::resource("/async_echo").route(web::post().to(echo_pet_async)))
            .service(web::resource("/async_echo_2").route(web::post().to(echo_pet_async_2)))
            .service(web::resource("/adopt").route(web::post().to(adopt_pet)))
            .service(web::resource("/nothing").route(web::get().to(nothing)))
            .service(web::resource("/random").to(some_pet))
            .service(path_without_slash)
            .service(web::scope("/test").service(
                web::resource("{id}").route(web::get().to(path_with_param_without_slash)),
            ));
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
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "class": {
                          "enum": [
                            "dog",
                            "cat",
                            "other"
                          ],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required": [
                        "birthday",
                        "class",
                        "name"
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/api/adopt": {
                      "post": {
                        "responses": {
                          "201": {
                            "description": "Created",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/async_echo": {
                      "post": {
                        "parameters": [
                          {
                            "in": "body",
                            "name": "body",
                            "required": true,
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/async_echo_2": {
                      "post": {
                        "parameters": [
                          {
                            "in": "body",
                            "name": "body",
                            "required": true,
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/echo": {
                      "post": {
                        "parameters": [
                          {
                            "in": "body",
                            "name": "body",
                            "required": true,
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/no-slash": {
                      "post": {
                        "responses": {
                          "204": {
                            "description": "No Content"
                          }
                        }
                      }
                    },
                    "/api/nothing": {
                      "get": {
                        "responses": {
                          "204": {
                            "description": "No Content"
                          }
                        }
                      }
                    },
                    "/api/random": {
                      "delete": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "head": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "options": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "patch": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "post": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      },
                      "put": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        }
                      }
                    },
                    "/api/test/{id}": {
                      "get": {
                        "parameters": [
                          {
                            "format": "int32",
                            "in": "path",
                            "name": "id",
                            "required": true,
                            "type": "integer"
                          }
                        ],
                        "responses": {
                          "204": {
                            "description": "No Content"
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

#[cfg(feature = "actix4")]
#[test]
fn test_non_boxed_body_middleware() {
    #[api_v2_operation]
    fn echo_pet(body: web::Json<Pet>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        fut_ok(body)
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/test")
                .service(web::resource("/echo").route(web::post().to(echo_pet)))
                .wrap(Logger::default())
                .wrap(DefaultHeaders::default().add(("X-Test", "Value"))),
        );
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
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                "definitions":{
                  "Pet":{
                    "description":"Pets are awesome!",
                    "properties":{
                      "birthday":{
                        "format":"date",
                        "type":"string"
                      },
                      "class":{
                        "enum":[
                          "dog",
                          "cat",
                          "other"
                          ],
                          "type":"string"
                        },
                        "id":{
                          "format":"int64",
                          "type":"integer"
                        },
                        "name":{
                          "description":"Pick a good one.",
                          "type":"string"
                        },
                        "updatedOn":{
                          "format":"date-time",
                          "type":"string"
                        },
                        "uuid0":{
                          "format":"uuid",
                          "type":"string"
                        },
                        "uuid1":{
                          "format":"uuid",
                          "type":"string"
                        }
                      },
                      "required":[
                        "birthday",
                        "class",
                        "name"
                        ],
                        "type":"object"
                      }
                    },
                    "info":{
                      "title":"",
                      "version":""
                    },
                    "paths":{
                      "/api/test/echo":{
                        "post":{
                          "parameters":[{
                            "in":"body",
                            "name":"body",
                            "required":true,
                            "schema":{
                              "$ref":"#/definitions/Pet"
                            }
                          }],
                          "responses":{
                            "200":{
                              "description":"OK",
                              "schema":{
                                "$ref":"#/definitions/Pet"
                              }
                            }
                          }
                        }
                      }
                    },
                    "swagger":"2.0"
                  }
                ),
            );
        },
    );
}

#[test]
#[allow(dead_code)]
fn test_params() {
    #[derive(Deserialize, Apiv2Schema)]
    #[cfg_attr(
        any(feature = "actix3-validator", feature = "actix4-validator"),
        derive(Validate)
    )]
    struct KnownResourceBadge {
        resource: String,
        name: String,
    }

    /// KnownBadge Id Doc
    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct KnownBadgeId(String);

    /// KnownBadge Id2 Doc
    #[derive(Deserialize, Apiv2Schema)]
    struct KnownBadgeId2(u64);

    /// KnownBadge Id3 Doc
    #[derive(Deserialize, Apiv2Schema)]
    struct KnownBadgeId3(
        /// Number Doc
        pub u64,
        /// String Doc
        pub String,
    );

    /// KnownBadge Id4 Doc
    #[derive(Serialize, Deserialize, Apiv2Schema)]
    #[cfg_attr(
        any(feature = "actix3-validator", feature = "actix4-validator"),
        derive(Validate)
    )]
    struct KnownBadgeId4 {
        id: String,
    }

    #[derive(Deserialize, Apiv2Schema)]
    #[cfg_attr(
        any(feature = "actix3-validator", feature = "actix4-validator"),
        derive(Validate)
    )]
    struct BadgeParams {
        res: Option<u16>,
        colors: Vec<String>,
    }

    #[derive(Deserialize, Apiv2Schema)]
    #[cfg_attr(
        any(feature = "actix3-validator", feature = "actix4-validator"),
        derive(Validate)
    )]
    struct BadgeBody {
        /// JSON value
        json: Option<serde_json::Value>,
        yaml: Option<serde_yaml::Value>,
    }

    #[derive(Deserialize, Apiv2Schema)]
    struct BadgeBodyPatch {
        /// JSON value
        json: Option<serde_json::Value>,
    }

    #[derive(Deserialize, Apiv2Schema)]
    struct BadgeForm {
        data: String,
    }

    #[derive(Deserialize, Apiv2Schema)]
    struct AppState {
        data: String,
    }

    async fn is_data_empty(p: &AppState) -> bool {
        p.data.is_empty()
    }

    // issue: https://github.com/paperclip-rs/paperclip/issues/216
    #[cfg(not(feature = "actix2"))]
    #[api_v2_operation]
    async fn check_data_ref_async(
        app: web::Data<AppState>,
        _req_data: Option<web::ReqData<bool>>, // this should compile and change nothing
    ) -> web::Json<bool> {
        web::Json(is_data_empty(app.get_ref()).await)
    }

    // Use dumb check_data_ref_async function for actix2 instead of real one
    #[cfg(feature = "actix2")]
    #[api_v2_operation]
    async fn check_data_ref_async(app: web::Data<AppState>) -> web::Json<bool> {
        web::Json(is_data_empty(app.get_ref()).await)
    }

    #[api_v2_operation]
    fn get_resource_2(_p: web::Path<u32>) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn get_known_badge_1(
        _p: web::Path<KnownResourceBadge>,
        _q: web::Query<BadgeParams>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn get_known_badge_2(
        _p: web::Path<(u32, String)>,
        _q: web::Query<BadgeParams>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn get_known_badge_3(
        _p: web::Path<KnownBadgeId>,
    ) -> impl Future<Output = Result<web::Json<KnownBadgeId>, Error>> {
        futures::future::ok(web::Json(KnownBadgeId("id".into())))
    }

    #[api_v2_operation]
    fn get_known_badge_4(
        _p1: web::Path<KnownBadgeId>,
        _p2: web::Path<(KnownBadgeId2, KnownBadgeId3)>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(any(feature = "actix3-validator", feature = "actix4-validator"))]
    #[api_v2_operation]
    fn get_known_badge_5(_p1: ValidatedPath<KnownBadgeId4>) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(not(any(feature = "actix3-validator", feature = "actix4-validator")))]
    #[api_v2_operation]
    fn get_known_badge_5(_p1: web::Path<KnownBadgeId4>) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn post_badge_1(
        _p: web::Path<KnownResourceBadge>,
        _q: web::Query<BadgeParams>,
        _f: web::Form<BadgeForm>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn post_badge_2(
        _p: web::Path<(u32, String)>,
        _b: web::Json<BadgeBody>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn post_badge_3(
        _p: web::Path<u32>,
        _b: web::Json<BadgeBody>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[api_v2_operation]
    fn patch_badge_3(
        _p: web::Path<u32>,
        _b: web::Json<BadgeBodyPatch>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(any(feature = "actix3-validator", feature = "actix4-validator"))]
    #[api_v2_operation]
    fn post_badge_4(
        _p: web::Path<u32>,
        _b: ValidatedJson<BadgeBody>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(not(any(feature = "actix3-validator", feature = "actix4-validator")))]
    #[api_v2_operation]
    fn post_badge_4(
        _p: web::Path<u32>,
        _b: web::Json<BadgeBody>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(any(feature = "actix3-validator", feature = "actix4-validator"))]
    #[api_v2_operation]
    fn patch_badge_4(
        _p: ValidatedPath<KnownResourceBadge>,
        _q: ValidatedQuery<BadgeParams>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    #[cfg(not(any(feature = "actix3-validator", feature = "actix4-validator")))]
    #[api_v2_operation]
    fn patch_badge_4(
        _p: web::Path<KnownResourceBadge>,
        _q: web::Query<BadgeParams>,
    ) -> impl Future<Output = &'static str> {
        ready("")
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
                            // Test that we can also have parameters in scopes
                            web::scope("/v2/{resource}")
                                .service(
                                    web::resource("/v/{name}")
                                        .route(web::get().to(get_known_badge_2))
                                        .route(web::post().to(post_badge_2)),
                                )
                                .service(
                                    web::resource("/v/{id}")
                                        .route(web::get().to(get_known_badge_3)),
                                )
                                .service(
                                    web::resource("/v/{id}/{id2}/{id3}/{id4}")
                                        .route(web::get().to(get_known_badge_4)),
                                )
                                .service(
                                    web::resource("/v/{id}/{id2}/{id3}/{id4}/{id5}")
                                        .route(web::get().to(get_known_badge_5)),
                                )
                                .service(
                                    web::resource("/v")
                                        .route(web::post().to(post_badge_3))
                                        .route(web::patch().to(patch_badge_3)),
                                )
                                .service(
                                    web::resource("/v_")
                                        .route(web::post().to(post_badge_4))
                                        .route(web::patch().to(patch_badge_4)),
                                )
                                .service(
                                    web::resource("/foo").route(web::get().to(get_resource_2)),
                                ),
                        )
                        .service(
                            web::resource("/v2/check_data")
                                .route(web::get().to(check_data_ref_async)),
                        ),
                )
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "BadgeBody": {
                            "properties": {
                                "json": {
                                    "description": "JSON value",
                                },
                                "yaml": {
                                }
                            },
                            "type":"object"
                        },
                        "BadgeBodyPatch": {
                            "properties": {
                                "json": {
                                    "description": "JSON value",
                                }
                            },
                            "type":"object"
                        },
                        "KnownBadgeId": {
                            "description": "KnownBadge Id Doc",
                            "type": "string"
                        },
                    },
                    "info": {
                        "title": "",
                        "version": ""
                    },
                    "paths": {
                        "/api/v1/{resource}/v/{name}": {
                            "delete": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "get": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "head": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "options": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "patch": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    },
                                    {
                                        "in": "formData",
                                        "name": "data",
                                        "required": true,
                                        "type": "string"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "put": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/foo": {
                            "get": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/v": {
                            "patch": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/BadgeBodyPatch"
                                        }
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/BadgeBody"
                                        }
                                    }
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/v/{id}": {
                            "get": {
                                "parameters": [
                                    {
                                        "description": "KnownBadge Id Doc",
                                        "in": "path",
                                        "name": "id",
                                        "required": true,
                                        "type": "string"
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/KnownBadgeId"
                                        }
                                    }
                                }
                            }
                        },
                        "/api/v2/{resource}/v/{id}/{id2}/{id3}/{id4}": {
                            "get": {
                                "parameters": [
                                    {
                                        "description": "KnownBadge Id Doc",
                                        "in": "path",
                                        "name": "id",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "description": "KnownBadge Id2 Doc",
                                        "format": "int64",
                                        "in": "path",
                                        "name": "id2",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "description": "Number Doc",
                                        "format": "int64",
                                        "in": "path",
                                        "name": "id3",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "description": "String Doc",
                                        "in": "path",
                                        "name": "id4",
                                        "required": true,
                                        "type": "string"
                                    },
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/v/{id}/{id2}/{id3}/{id4}/{id5}": {
                            "get": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "id5",
                                        "required": true,
                                        "type": "string"
                                    },
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/v/{name}": {
                            "get": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/BadgeBody"
                                        }
                                    }
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/{resource}/v_": {
                            "patch": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "query",
                                        "items": {
                                            "type": "string"
                                        },
                                        "name": "colors",
                                        "required": true,
                                        "type": "array"
                                    },
                                    {
                                        "format": "int32",
                                        "in": "query",
                                        "name": "res",
                                        "type": "integer"
                                    }
                                ],
                                "responses": {
                                }
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "format": "int32",
                                        "in": "path",
                                        "name": "resource",
                                        "required": true,
                                        "type": "integer"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/BadgeBody"
                                        }
                                    }
                                ],
                                "responses": {
                                }
                            }
                        },
                        "/api/v2/check_data": {
                            "get": {
                                "responses": {
                                    "200":{
                                        "description": "OK",
                                        "schema":{
                                           "type": "boolean"
                                        }
                                     }
                                }
                            }
                        },
                    },
                    "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_map_in_out() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct ImageId(u64);

    #[derive(Serialize, Apiv2Schema)]
    struct Image {
        data: String,
        id: ImageId,
    }

    #[derive(Deserialize, Apiv2Schema)]
    struct Filter {
        #[allow(dead_code)]
        pub folders: HashMap<String, Vec<ImageId>>,
    }

    #[derive(Serialize, Apiv2Schema)]
    struct Catalogue {
        pub folders: HashMap<Uuid, Vec<Image>>,
    }

    #[api_v2_operation]
    fn some_images() -> impl Future<Output = web::Json<BTreeMap<String, Image>>> {
        #[allow(unreachable_code)]
        ready(unimplemented!())
    }

    #[api_v2_operation]
    fn catalogue(_filter: web::Json<Filter>) -> impl Future<Output = web::Json<Catalogue>> {
        #[allow(unreachable_code)]
        ready(unimplemented!())
    }

    run_and_check_app(
        || {
            let app = App::new().wrap_api().with_json_spec_at("/api/spec");

            #[cfg(feature = "swagger-ui")]
            let app = app.with_swagger_ui_at("/swagger");

            #[cfg(feature = "rapidoc")]
            let app = app.with_rapidoc_at("/rapidoc");

            app.service(web::resource("/images").route(web::get().to(some_images)))
                .service(web::resource("/catalogue").route(web::post().to(catalogue)))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions":{
                        "Catalogue":{
                           "properties":{
                              "folders":{
                                 "additionalProperties":{
                                    "items":{
                                       "properties":{
                                          "data":{
                                             "type":"string"
                                          },
                                          "id":{
                                             "format":"int64",
                                             "type":"integer"
                                          }
                                       },
                                       "required":[
                                          "data",
                                          "id"
                                       ],
                                       "type":"object"
                                    },
                                    "type":"array"
                                 },
                                 "type":"object"
                              }
                           },
                           "required":[
                              "folders"
                           ],
                           "type":"object"
                        },
                        "Filter":{
                           "properties":{
                              "folders":{
                                 "additionalProperties":{
                                    "items":{
                                       "format":"int64",
                                       "type":"integer"
                                    },
                                    "type":"array"
                                 },
                                 "type":"object"
                              }
                           },
                           "required":[
                              "folders"
                           ],
                           "type":"object"
                        },
                        "Image":{
                           "properties":{
                              "data":{
                                 "type":"string"
                              },
                              "id":{
                                 "format":"int64",
                                 "type":"integer"
                              }
                           },
                           "required":[
                              "data",
                              "id"
                           ],
                           "type":"object"
                        }
                     },
                     "info":{
                        "title":"",
                        "version":""
                     },
                     "paths":{
                        "/catalogue":{
                           "post":{
                              "parameters":[
                                 {
                                    "in":"body",
                                    "name":"body",
                                    "required":true,
                                    "schema":{
                                       "$ref":"#/definitions/Filter"
                                    }
                                 }
                              ],
                              "responses":{
                                 "200":{
                                    "description":"OK",
                                    "schema":{
                                       "$ref":"#/definitions/Catalogue"
                                    }
                                 }
                              }
                           }
                        },
                        "/images":{
                           "get":{
                              "responses":{
                                 "200":{
                                    "description":"OK",
                                    "schema":{
                                       "additionalProperties":{
                                          "$ref":"#/definitions/Image"
                                       },
                                       "type":"object"
                                    }
                                 }
                              }
                           }
                        }
                     },
                     "swagger":"2.0"
                }),
            );

            #[cfg(feature = "swagger-ui")]
            {
                let resp = CLIENT
                    .get(&format!("http://{}/swagger/swagger-ui.css", addr))
                    .send()
                    .expect("request failed?");

                assert_eq!(resp.status().as_u16(), 200);
                assert_eq!(
                    resp.headers()
                        .get(actix_web::http::header::CONTENT_TYPE)
                        .expect("Could not get Content-Type header"),
                    "text/css"
                );
            }

            #[cfg(feature = "rapidoc")]
            {
                let resp = CLIENT
                    .get(&format!("http://{}/rapidoc", addr))
                    .send()
                    .expect("request failed?");

                assert_eq!(resp.status().as_u16(), 200);
            }
        },
    );
}

#[test]
fn test_serde_flatten() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct PagedQuery {
        /// First image number to return
        offset: Option<i32>,
        /// Return number of images
        size: Option<i32>,
    }

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct Paging {
        /// Starting image number
        offset: i32,
        /// Total images found
        total: i32,
        /// Page size
        size: i32,
    }

    #[derive(Serialize, Apiv2Schema)]
    struct Image {
        data: String,
        id: Uuid,
        time: chrono_dev::DateTime<chrono_dev::Utc>,
    }

    /// Images response with paging information embedded
    #[derive(Serialize, Apiv2Schema)]
    struct Images {
        data: Vec<Image>,
        #[serde(flatten)]
        paging: Paging,
    }

    /// Query images from library by name
    #[derive(Deserialize, Apiv2Schema)]
    struct ImagesQuery {
        #[serde(flatten)]
        paging: PagedQuery,
        name: Option<String>,
    }

    /// Image author info
    #[derive(Deserialize, Apiv2Schema)]
    #[allow(dead_code)]
    struct Author {
        name: String,
        address: Option<String>,
        age: Option<u8>,
    }

    /// Image to persist
    #[derive(Deserialize, Apiv2Schema)]
    #[allow(dead_code)]
    struct ImagePayload {
        data: String,
        id: Uuid,
        #[serde(flatten)]
        author: Author,
    }

    /// Article to persist
    #[derive(Deserialize, Apiv2Schema)]
    #[allow(dead_code)]
    struct Article {
        description: String,
        id: Uuid,
        #[serde(flatten)]
        author: Option<Author>,
    }

    #[api_v2_operation]
    async fn some_images(_filter: web::Query<ImagesQuery>) -> Result<web::Json<Images>, Error> {
        #[allow(unreachable_code)]
        if _filter.paging.offset.is_some() && _filter.name.is_some() {
            unimplemented!()
        }
        unimplemented!()
    }

    #[api_v2_operation]
    async fn add_images(_content: web::Json<ImagePayload>) -> Result<NoContent, Error> {
        #[allow(unreachable_code)]
        if _content.author.address.is_some() {
            unimplemented!()
        }
        unimplemented!()
    }

    #[api_v2_operation]
    async fn add_article(_content: web::Json<Article>) -> Result<NoContent, Error> {
        #[allow(unreachable_code)]
        if _content.author.is_some() {
            unimplemented!()
        }
        unimplemented!()
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(
                    web::resource("/images")
                        .route(web::get().to(some_images))
                        .route(web::post().to(add_images)),
                )
                .service(web::resource("/article").route(web::get().to(add_article)))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                      "definitions": {
                        "Article": {
                          "description": "Article to persist",
                          "properties": {
                            "address": {
                              "type": "string"
                            },
                            "age": {
                              "format": "int32",
                              "type": "integer"
                            },
                            "description": {
                              "type": "string"
                            },
                            "id": {
                              "format": "uuid",
                              "type": "string"
                            },
                            "name": {
                              "type": "string"
                            }
                          },
                          "required": [
                            "description",
                            "id"
                          ],
                          "type": "object"
                        },
                        "ImagePayload": {
                          "description": "Image to persist",
                          "properties": {
                            "address": {
                              "type": "string"
                            },
                            "age": {
                              "format": "int32",
                              "type": "integer"
                            },
                            "data": {
                              "type": "string"
                            },
                            "id": {
                              "format": "uuid",
                              "type": "string"
                            },
                            "name": {
                              "type": "string"
                            }
                          },
                          "required": [
                            "data",
                            "id",
                            "name"
                          ],
                          "type": "object"
                        },
                        "Images": {
                          "description": "Images response with paging information embedded",
                          "properties": {
                            "data": {
                              "items": {
                                "properties": {
                                  "data": {
                                    "type": "string"
                                  },
                                  "id": {
                                    "format": "uuid",
                                    "type": "string"
                                  },
                                  "time": {
                                    "format": "date-time",
                                    "type": "string"
                                  }
                                },
                                "required": [
                                  "data",
                                  "id",
                                  "time"
                                ],
                                "type": "object"
                              },
                              "type": "array"
                            },
                            "offset": {
                              "description": "Starting image number",
                              "format": "int32",
                              "type": "integer"
                            },
                            "size": {
                              "description": "Page size",
                              "format": "int32",
                              "type": "integer"
                            },
                            "total": {
                              "description": "Total images found",
                              "format": "int32",
                              "type": "integer"
                            }
                          },
                          "required": [
                            "data",
                            "offset",
                            "size",
                            "total"
                          ],
                          "type": "object"
                        }
                      },
                      "info": {
                        "title": "",
                        "version": ""
                      },
                      "paths": {
                        "/article": {
                          "get": {
                            "parameters": [
                              {
                                "in": "body",
                                "name": "body",
                                "required": true,
                                "schema": {
                                  "$ref": "#/definitions/Article"
                                }
                              }
                            ],
                            "responses": {
                              "204": {
                                "description": "No Content"
                              }
                            }
                          }
                        },
                        "/images": {
                          "get": {
                            "parameters": [
                              {
                                "in": "query",
                                "name": "name",
                                "type": "string"
                              },
                              {
                                "description": "First image number to return",
                                "format": "int32",
                                "in": "query",
                                "name": "offset",
                                "type": "integer"
                              },
                              {
                                "description": "Return number of images",
                                "format": "int32",
                                "in": "query",
                                "name": "size",
                                "type": "integer"
                              }
                            ],
                            "responses": {
                              "200": {
                                "description": "OK",
                                "schema": {
                                  "$ref": "#/definitions/Images"
                                }
                              }
                            }
                          },
                          "post": {
                            "parameters": [
                              {
                                "in": "body",
                                "name": "body",
                                "required": true,
                                "schema": {
                                  "$ref": "#/definitions/ImagePayload"
                                }
                              }
                            ],
                            "responses": {
                              "204": {
                                "description": "No Content"
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
fn test_serde_skip() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[serde(rename_all = "camelCase")]
    /// Pets are awesome!
    struct Pet {
        class: PetClass,
        #[serde(skip)]
        #[allow(dead_code)]
        skip_it: Option<chrono_dev::NaiveDateTime>,
        un: PetUnnamed,
    }

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[serde(rename_all = "lowercase")]
    enum PetClass {
        Dog,
        Cat,
        #[serde(rename = "other")]
        EverythingElse,
        #[serde(skip)]
        #[allow(dead_code)]
        Another,
    }

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct PetUnnamed(#[serde(skip)] bool, bool);

    #[post("/v0/pets")]
    #[api_v2_operation]
    fn post_pet(pet: web::Json<Pet>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(post_pet)
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "Pet": {
                            "description": "Pets are awesome!",
                            "properties": {
                                "class": {
                                "enum": ["dog", "cat", "other"],
                                    "type":"string"
                                },
                                "un": {
                                    "properties": {
                                        "1": {
                                            "type": "boolean"
                                        }
                                    },
                                    "required": ["1"],
                                    "type": "object"
                                },
                            },
                            "required":[
                                "class",
                                "un"
                            ],
                            "type":"object"
                        }
                    },
                    "info": {
                        "title":"",
                        "version":""
                    },
                    "paths": {
                        "/v0/pets": {
                            "post": {
                                "parameters": [
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
                            }
                        },
                    },
                    "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_list_in_out() {
    #[derive(Serialize, Deserialize, Apiv2Schema)]
    enum Sort {
        Asc,
        Desc,
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct Params {
        sort: Option<Sort>,
        limit: Option<u16>,
    }

    #[api_v2_operation]
    fn get_pets(_q: web::Query<Params>) -> impl Future<Output = web::Json<Vec<Pet>>> {
        #[allow(unreachable_code)]
        ready(unimplemented!())
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
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "info":{"title":"","version":""},
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "class": {
                          "enum": ["dog", "cat", "other"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required":["birthday", "class", "name"],
                      "type":"object"
                    }
                  },
                  "paths": {
                    "/pets": {
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "type": "array",
                              "items": {
                                "$ref": "#/definitions/Pet"
                              }
                            }
                          }
                        },
                        "parameters": [{
                            "format": "int32",
                            "in": "query",
                            "name": "limit",
                            "type": "integer"
                          }, {
                            "enum": ["Asc", "Desc"],
                            "in": "query",
                            "name": "sort",
                            "type": "string"
                        }],
                      },
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_tags() {
    #[derive(Serialize, Apiv2Schema)]
    struct Image {
        data: String,
        id: u64,
    }

    #[api_v2_operation(tags(Cats, Dogs))]
    fn some_pets_images() -> impl Future<Output = web::Json<Vec<Image>>> {
        ready(web::Json(Vec::new()))
    }

    #[api_v2_operation(tags(Cats, "Nice cars"))]
    fn some_cats_cars_images() -> impl Future<Output = web::Json<Vec<Image>>> {
        ready(web::Json(Vec::new()))
    }

    run_and_check_app(
        || {
            let mut spec = DefaultApiRaw::default();
            spec.tags = vec![
                Tag {
                    name: "Dogs".to_string(),
                    description: Some("Images of dogs".to_string()),
                    external_docs: None,
                },
                Tag {
                    name: "Cats".to_string(),
                    description: Some("Images of cats".to_string()),
                    external_docs: None,
                },
                Tag {
                    name: "Nice cars".to_string(),
                    description: Some("Images of nice cars".to_string()),
                    external_docs: None,
                },
            ];
            let mut extensions = BTreeMap::new();
            extensions.insert("x-my-attr".to_string(), serde_json::Value::Bool(true));
            spec.info = Info {
                version: "0.1".into(),
                title: "Image server".into(),
                extensions,
                ..Default::default()
            };

            let mut root_extensions = BTreeMap::new();
            root_extensions.insert(
                "x-root-level-extension".to_string(),
                serde_json::Value::Bool(false),
            );
            spec.extensions = root_extensions;

            App::new()
                .wrap_api_with_spec(spec)
                .with_json_spec_at("/api/spec")
                .service(web::resource("/images/pets").route(web::get().to(some_pets_images)))
                .service(
                    web::resource("/images/cats/cars").route(web::get().to(some_cats_cars_images)),
                )
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions":{
                        "Image":{
                            "properties":{
                                "data":{
                                "type":"string"
                                },
                                "id":{
                                "format":"int64",
                                "type":"integer"
                                }
                            },
                            "required":[
                                "data",
                                "id"
                            ],
                            "type":"object"
                        }
                    },
                    "info":{
                        "title":"Image server",
                        "version":"0.1",
                        "x-my-attr":true
                    },
                    "x-root-level-extension": false,
                    "paths":{
                        "/images/pets":{
                            "get":{
                                "responses":{
                                "200":{
                                    "description":"OK",
                                    "schema":{
                                        "items":{
                                            "$ref":"#/definitions/Image"
                                        },
                                        "type":"array"
                                    }
                                }
                                },
                                "tags":[ "Cats", "Dogs" ]
                            }
                        },
                        "/images/cats/cars":{
                            "get":{
                                "responses":{
                                "200":{
                                    "description":"OK",
                                    "schema":{
                                        "items":{
                                            "$ref":"#/definitions/Image"
                                        },
                                        "type":"array"
                                    }
                                }
                                },
                                "tags":[ "Cats", "Nice cars" ]
                            }
                        }
                    },
                    "swagger":"2.0",
                    "tags":[
                        {
                            "description":"Images of dogs",
                            "name":"Dogs"
                        },
                        {
                            "description":"Images of cats",
                            "name":"Cats"
                        },
                        {
                            "description":"Images of nice cars",
                            "name":"Nice cars"
                        }
                    ]
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
        ""
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct Params {
        limit: Option<u16>,
    }

    #[api_v2_operation]
    fn get_pets(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(actix_web::error::ErrorInternalServerError(""));
        }

        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    #[api_v2_operation]
    async fn get_pet_async() -> impl Responder {
        Pet::default()
    }

    #[api_v2_operation]
    fn get_pet() -> impl Responder {
        Pet::default()
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/").route(web::get().to(index)))
                .service(web::resource("/pets").route(web::get().to(get_pets)))
                .service(web::resource("/pet").route(web::get().to(get_pet)))
                .service(web::resource("/pet_async").route(web::get().to(get_pet_async)))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "info":{"title":"","version":""},
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "class": {
                          "enum": ["dog", "cat", "other"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required":["birthday", "class", "name"],
                      "type":"object"
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
                            "description": "OK",
                            "schema": {
                              "type": "array",
                              "items": {
                                "$ref": "#/definitions/Pet"
                              }
                            }
                          }
                        },
                        "parameters": [{
                            "format": "int32",
                            "in": "query",
                            "name": "limit",
                            "type": "integer"
                        }]
                      },
                    },
                    "/pet": {
                      "get": {
                        "responses": {}
                      }
                    },
                    "/pet_async": {
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

#[test]
#[allow(unreachable_code)]
fn test_operation_with_generics() {
    #[api_v2_operation]
    fn get_pet_by_id<I: paperclip::v2::schema::Apiv2Schema>(
        _path: web::Path<I>,
    ) -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        futures::future::ok(web::Json(vec![Pet::default()]))
    }

    #[api_v2_operation]
    async fn get_pet_by_name<S: paperclip::v2::schema::Apiv2Schema + ToString>(
        _path: web::Path<S>,
    ) -> Result<web::Json<Vec<Pet>>, Error> {
        Ok(web::Json(vec![Pet::default()]))
    }

    #[api_v2_operation]
    async fn get_pet_by_type<S>(_path: web::Path<S>) -> Result<web::Json<Vec<Pet>>, Error>
    where
        S: paperclip::v2::schema::Apiv2Schema + ToString,
    {
        Ok(web::Json(vec![Pet::default()]))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/pet/id/{id}").route(web::get().to(get_pet_by_id::<u64>)))
                .service(
                    web::resource("/pet/name/{name}")
                        .route(web::get().to(get_pet_by_name::<String>)),
                )
                .service(
                    web::resource("/pet/type/{type}")
                        .route(web::get().to(get_pet_by_type::<String>)),
                )
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions":{
                        "Pet":{
                           "description":"Pets are awesome!",
                           "properties":{
                              "class":{
                                 "enum":[
                                    "dog",
                                    "cat",
                                    "other"
                                 ],
                                 "type":"string"
                              },
                              "id":{
                                 "format":"int64",
                                 "type":"integer"
                              },
                              "name":{
                                 "description":"Pick a good one.",
                                 "type":"string"
                              },
                              "birthday": {
                                "format": "date",
                                "type": "string"
                              },
                              "updatedOn":{
                                 "format":"date-time",
                                 "type":"string"
                              },
                              "uuid0":{
                                "format":"uuid",
                                "type":"string"
                              },
                              "uuid1":{
                                "format":"uuid",
                                "type":"string"
                              }
                           },
                           "required":[
                             "birthday",
                              "class",
                              "name"
                           ],
                           "type":"object"
                        }
                     },
                     "info":{
                        "title":"",
                        "version":""
                     },
                     "paths":{
                        "/pet/id/{id}":{
                           "get":{
                              "responses":{
                                 "200":{
                                    "description":"OK",
                                    "schema":{
                                       "items":{
                                          "$ref":"#/definitions/Pet"
                                       },
                                       "type":"array"
                                    }
                                 }
                              },
                              "parameters":[
                                {
                                   "format":"int64",
                                   "in":"path",
                                   "name":"id",
                                   "required":true,
                                   "type":"integer"
                                }
                             ]
                          },
                        },
                        "/pet/name/{name}":{
                           "get":{
                              "responses":{
                                 "200":{
                                    "description":"OK",
                                    "schema":{
                                       "items":{
                                          "$ref":"#/definitions/Pet"
                                       },
                                       "type":"array"
                                    }
                                 }
                              },
                              "parameters":[
                                {
                                   "in":"path",
                                   "name":"name",
                                   "required":true,
                                   "type":"string"
                                }
                             ]
                          },
                        },
                        "/pet/type/{type}":{
                           "get":{
                              "responses":{
                                 "200":{
                                    "description":"OK",
                                    "schema":{
                                       "items":{
                                          "$ref":"#/definitions/Pet"
                                       },
                                       "type":"array"
                                    }
                                 }
                              },
                              "parameters":[
                                {
                                   "in":"path",
                                   "name":"type",
                                   "required":true,
                                   "type":"string"
                                }
                             ]
                          },
                        }
                     },
                     "swagger":"2.0"
                }),
            );
        },
    );
}

#[test]
#[allow(unreachable_code)]
fn test_operations_documentation() {
    /// Index call
    #[api_v2_operation]
    fn index() -> impl Responder {
        ""
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct Params {
        limit: Option<u16>,
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct Dog {
        name: String,
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct InvisibleDog {
        name: String,
    }

    /// List all pets
    ///
    /// Will provide list of all pets available for sale
    #[api_v2_operation]
    fn get_pets(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(actix_web::error::ErrorInternalServerError(""));
        }

        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    /// Get pet info
    ///
    /// Will provide details on a pet
    #[api_v2_operation]
    async fn get_pet_async() -> impl Responder {
        Pet::default()
    }

    /// Get pet info
    /// sync version
    #[api_v2_operation]
    fn get_pet() -> impl Responder {
        Pet::default()
    }

    #[api_v2_operation(description = "An invisible dog handler", skip)]
    fn get_dogs() -> impl Future<Output = Result<web::Json<Vec<InvisibleDog>>, Error>> {
        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    #[api_v2_operation(description = "A visible dog handler")]
    fn get_dog() -> impl Future<Output = Result<web::Json<Dog>, Error>> {
        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/").route(web::get().to(index)))
                .service(web::resource("/pets").route(web::get().to(get_pets)))
                .service(web::resource("/pet").route(web::get().to(get_pet)))
                .service(web::resource("/dog").route(web::get().to(get_dogs)))
                .service(
                    web::scope("/dogs")
                        .service(web::resource("").route(web::get().to(get_dogs)))
                        .service(web::resource("{id}").route(web::get().to(get_dog)))
                        .service(web::resource("{id}/another").to(get_dogs))
                        .service(web::resource("{id}/another-visible").to(get_dog)),
                )
                .service(web::resource("/pet_async").route(web::get().to(get_pet_async)))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Dog": {
                      "properties": {
                        "name": {
                          "type": "string"
                        }
                      },
                      "required": [
                        "name"
                      ],
                      "type": "object"
                    },
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "class": {
                          "enum": [
                            "dog",
                            "cat",
                            "other"
                          ],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required": [
                        "birthday",
                        "class",
                        "name"
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/": {
                      "get": {
                        "responses": {},
                        "summary": "Index call"
                      }
                    },
                    "/dogs/{id}": {
                      "get": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      }
                    },
                    "/dogs/{id}/another-visible": {
                      "delete": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "get": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "head": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "options": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "patch": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "post": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      },
                      "put": {
                        "description": "A visible dog handler",
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Dog"
                            }
                          }
                        }
                      }
                    },
                    "/pet": {
                      "get": {
                        "responses": {},
                        "summary": "Get pet info sync version"
                      }
                    },
                    "/pet_async": {
                      "get": {
                        "description": "Will provide details on a pet",
                        "responses": {},
                        "summary": "Get pet info"
                      }
                    },
                    "/pets": {
                      "get": {
                        "description": "Will provide list of all pets available for sale",
                        "parameters": [
                          {
                            "format": "int32",
                            "in": "query",
                            "name": "limit",
                            "type": "integer"
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Pet"
                              },
                              "type": "array"
                            }
                          }
                        },
                        "summary": "List all pets"
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
#[allow(unreachable_code)]
fn test_operations_macro_attributes() {
    /// Index operation
    ///
    /// This doc comment will be overriden by macro attrs
    #[api_v2_operation(
        summary = "Root",
        description = "Provides an empty value in response",
        operation_id = "getIndex",
        consumes = "application/json, text/plain",
        produces = "text/plain",
        deprecated
    )]
    fn index() -> impl Responder {
        ""
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    struct Params {
        limit: Option<u16>,
    }

    /// List all pets (in summary)
    ///
    /// This doc comment will be used in description
    #[deprecated(since = "1.0")]
    #[api_v2_operation(operation_id = "getPets")]
    fn get_pets(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(actix_web::error::ErrorInternalServerError(""));
        }

        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    /// List all pets (in summary)
    ///
    /// This doc comment will be used in description
    #[api_v2_operation(operation_id = "getDogs")]
    #[deprecated]
    fn get_dogs(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(actix_web::error::ErrorInternalServerError(""));
        }

        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    #[derive(Serialize, Deserialize, Apiv2Schema)]
    pub struct Car {
        brand: String,
    }

    /// List all cars (in summary)
    ///
    /// This route will not appear in openapi.json
    #[api_v2_operation(skip)]
    fn get_cars(
        _data: web::Data<String>,
        _q: web::Query<Params>,
    ) -> impl Future<Output = Result<web::Json<Vec<Car>>, Error>> {
        if true {
            // test for return in wrapper blocks (#75)
            return futures::future::err(actix_web::error::ErrorInternalServerError(""));
        }

        futures::future::err(actix_web::error::ErrorInternalServerError(""))
    }

    run_and_check_app(
        || {
            #[allow(deprecated)]
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(web::resource("/").route(web::get().to(index)))
                .service(web::resource("/pets").route(web::get().to(get_pets)))
                .service(web::resource("/dogs").route(web::get().to(get_dogs)))
                .service(web::resource("/cars").route(web::get().to(get_cars)))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "class": {
                          "enum": [
                            "dog",
                            "cat",
                            "other"
                          ],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required": [
                        "birthday",
                        "class",
                        "name"
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/": {
                      "get": {
                        "consumes": [
                          "application/json",
                          "text/plain"
                        ],
                        "deprecated": true,
                        "description": "Provides an empty value in response",
                        "operationId": "getIndex",
                        "produces": [
                          "text/plain"
                        ],
                        "responses": {},
                        "summary": "Root"
                      }
                    },
                    "/dogs": {
                      "get": {
                        "deprecated": true,
                        "description": "This doc comment will be used in description",
                        "operationId": "getDogs",
                        "parameters": [
                          {
                            "format": "int32",
                            "in": "query",
                            "name": "limit",
                            "type": "integer"
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Pet"
                              },
                              "type": "array"
                            }
                          }
                        },
                        "summary": "List all pets (in summary)"
                      }
                    },
                    "/pets": {
                      "get": {
                        "deprecated": true,
                        "description": "This doc comment will be used in description",
                        "operationId": "getPets",
                        "parameters": [
                          {
                            "format": "int32",
                            "in": "query",
                            "name": "limit",
                            "type": "integer"
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Pet"
                              },
                              "type": "array"
                            }
                          }
                        },
                        "summary": "List all pets (in summary)"
                      }
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

// #[test] // issue #71
// fn test_multiple_method_routes() {
//     #[api_v2_operation]
//     fn test_get() -> impl Future<Output = String> {
//         ready("get".into())
//     }
//
//     #[api_v2_operation]
//     fn test_post() -> impl Future<Output = String> {
//         ready("post".into())
//     }
//
//     fn test_app<F, T, B>(f: F)
//     where
//         F: Fn() -> App<T, B> + Clone + Send + Sync + 'static,
//         B: MessageBody + 'static,
//         T: ServiceFactory<
//                 Config = (),
//                 Request = ServiceRequest,
//                 Response = ServiceResponse<B>,
//                 Error = Error,
//                 InitError = (),
//             > + 'static,
//     {
//         run_and_check_app(f, |addr| {
//             let resp = CLIENT
//                 .get(&format!("http://{}/v1/foo", addr))
//                 .send()
//                 .expect("request failed?");
//             assert_eq!(resp.status().as_u16(), 200);
//             assert_eq!(resp.text().unwrap(), "get");
//
//             let resp = CLIENT
//                 .post(&format!("http://{}/v1/foo", addr))
//                 .send()
//                 .expect("request failed?");
//             assert_eq!(resp.status().as_u16(), 200);
//             assert_eq!(resp.text().unwrap(), "post");
//
//             let resp = CLIENT
//                 .get(&format!("http://{}/api/spec", addr))
//                 .send()
//                 .expect("request failed?");
//
//             check_json(
//                 resp,
//                 json!({
//                   "info":{"title":"","version":""},
//                   "definitions": {},
//                   "paths": {
//                     "/v1/foo": {
//                       "get": {
//                         "responses": {},
//                       },
//                       "post": {
//                         "responses": {},
//                       },
//                     }
//                   },
//                   "swagger": "2.0",
//                 }),
//             );
//         });
//     }
//
//     test_app(|| {
//         App::new()
//             .wrap_api()
//             .with_json_spec_at("/api/spec")
//             .route("/v1/foo", web::get().to(test_get))
//             .route("/v1/foo", web::post().to(test_post))
//             .build()
//     });
//
//     fn config(cfg: &mut web::ServiceConfig) {
//         cfg.route("/foo", web::get().to(test_get))
//             .route("/foo", web::post().to(test_post));
//     }
//
//     test_app(|| {
//         App::new()
//             .wrap_api()
//             .with_json_spec_at("/api/spec")
//             .service(web::scope("/v1").configure(config))
//             .build()
//     });
//
//     fn config_1(cfg: &mut web::ServiceConfig) {
//         cfg.route("/v1/foo", web::get().to(test_get));
//     }
//
//     fn config_2(cfg: &mut web::ServiceConfig) {
//         cfg.route("/v1/foo", web::post().to(test_post));
//     }
//
//     test_app(|| {
//         App::new()
//             .wrap_api()
//             .with_json_spec_at("/api/spec")
//             .configure(config_1)
//             .configure(config_2)
//             .build()
//     });
// }

#[test]
fn test_custom_extractor_empty_schema() {
    #[derive(Apiv2Schema)]
    #[openapi(empty)]
    struct SomeUselessThing<T>(T);

    impl FromRequest for SomeUselessThing<String> {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
            fut_ok(SomeUselessThing(String::from("booya")))
        }
    }

    #[api_v2_operation]
    fn index(
        _req: HttpRequest,
        _payload: String,
        _thing: SomeUselessThing<String>,
    ) -> impl Future<Output = &'static str> {
        ready("")
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .service(web::resource("/").route(web::get().to(index)))
                .with_raw_json_spec(|app, spec| {
                    app.route(
                        "/api/spec",
                        web::get().to(move || {
                            #[cfg(feature = "actix4")]
                            {
                                let spec = spec.clone();
                                async move {
                                    paperclip::actix::HttpResponseWrapper(
                                        actix_web::HttpResponse::Ok().json(&spec),
                                    )
                                }
                            }

                            #[cfg(not(feature = "actix4"))]
                            actix_web::HttpResponse::Ok().json(&spec)
                        }),
                    )
                })
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "info":{"title":"","version":""},
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

#[test]
fn test_errors_app() {
    use actix_web::{
        error::{ErrorBadRequest, ResponseError},
        HttpResponse,
    };
    use std::fmt;

    #[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
    struct PetErrorScheme1 {}
    #[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
    struct PetErrorScheme2 {}
    #[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
    struct PetErrorScheme3 {}

    #[api_v2_errors(
        400,
        description = "Sorry, bad request",
        code = 401,
        code = 403,
        schema = "PetErrorScheme1",
        description = "Forbidden, go away",
        500,
        description = "Internal Server Error",
        schema = "PetErrorScheme2"
    )]
    #[derive(Debug)]
    struct PetError {}

    #[api_v2_errors(
        400,
        description = "Sorry, bad request",
        code = 401,
        code = 403,
        schema = "PetErrorScheme1",
        description = "Forbidden, go away",
        500,
        description = "Internal Server Error",
        default_schema = "PetErrorScheme2"
    )]
    #[derive(Debug)]
    struct PetError2 {}

    #[api_v2_errors_overlay(401)]
    #[derive(Debug)]
    struct PetErrorOverlay(pub PetError2);

    impl fmt::Display for PetError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Bad Request")
        }
    }
    impl fmt::Display for PetError2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Bad Request")
        }
    }

    impl ResponseError for PetError {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::from_error(ErrorBadRequest("Bad Request"))
        }
    }
    impl ResponseError for PetError2 {
        fn error_response(&self) -> HttpResponse {
            HttpResponse::from_error(ErrorBadRequest("Bad Request"))
        }
    }

    #[api_v2_operation]
    async fn echo_pet_with_errors(body: web::Json<Pet>) -> Result<web::Json<Pet>, PetError> {
        Ok(body)
    }

    #[api_v2_operation]
    async fn echo_pet_with_errors2(
        body: web::Json<Pet>,
    ) -> Result<web::Json<Pet>, PetErrorOverlay> {
        Ok(body)
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/echo").route(web::post().to(echo_pet_with_errors)));
        cfg.service(web::resource("/echo2").route(web::post().to(echo_pet_with_errors2)));
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
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "info":{"title":"","version":""},
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "class": {
                          "enum": ["dog", "cat", "other"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required":["birthday", "class", "name"],
                      "type":"object"
                    },
                    "PetErrorScheme1": {
                      "type": "object"
                    },
                    "PetErrorScheme2": {
                      "type": "object"
                    },
                  },
                  "paths": {
                    "/api/echo": {
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
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          },
                          "400": {
                            "description": "Sorry, bad request"
                          },
                          "401": {
                            "description": "Unauthorized"
                          },
                          "403":{
                            "description": "Forbidden, go away",
                            "schema": {
                              "$ref": "#/definitions/PetErrorScheme1"
                            }
                          },
                          "500": {
                            "description": "Internal Server Error",
                            "schema": {
                              "$ref": "#/definitions/PetErrorScheme2"
                            }
                          }
                        }
                      }
                    },
                    "/api/echo2": {
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
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          },
                          "400": {
                            "description": "Sorry, bad request",
                            "schema": {
                              "$ref": "#/definitions/PetErrorScheme2"
                            }
                          },
                          "403":{
                            "description": "Forbidden, go away",
                            "schema": {
                              "$ref": "#/definitions/PetErrorScheme1"
                            }
                          },
                          "500": {
                            "description": "Internal Server Error",
                            "schema": {
                              "$ref": "#/definitions/PetErrorScheme2"
                            }
                          }
                        }
                      }
                    },
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_security_app() {
    #[derive(Apiv2Security, Deserialize)]
    #[openapi(
        apiKey,
        alias = "JWT",
        in = "header",
        name = "Authorization",
        description = "Use format 'Bearer TOKEN'"
    )]
    struct AccessToken;

    impl FromRequest for AccessToken {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
            ready(Ok(Self {}))
        }
    }

    #[derive(Apiv2Security, Deserialize)]
    #[openapi(
        oauth2,
        alias = "MyOAuth2",
        auth_url = "http://example.com/",
        token_url = "http://example.com/token",
        flow = "password"
    )]
    struct OAuth2Access;

    impl FromRequest for OAuth2Access {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
            ready(Ok(Self {}))
        }
    }

    #[derive(Apiv2Security, Deserialize)]
    #[openapi(parent = "OAuth2Access", scopes("pets.read", "pets.write"))]
    struct PetScope;

    impl FromRequest for PetScope {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
            ready(Ok(Self {}))
        }
    }

    #[api_v2_operation]
    async fn echo_pet_with_jwt(_: AccessToken, body: web::Json<Pet>) -> web::Json<Pet> {
        body
    }

    #[api_v2_operation]
    async fn echo_pet_with_petstore(_: PetScope, body: web::Json<Pet>) -> web::Json<Pet> {
        body
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/echo1").route(web::post().to(echo_pet_with_jwt)))
            .service(web::resource("/echo2").route(web::post().to(echo_pet_with_petstore)));
    }

    run_and_check_app(
        move || {
            App::new()
                .wrap_api()
                .service(web::scope("/api").configure(config))
                .with_json_spec_at("/spec")
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "info":{"title":"","version":""},
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "class": {
                          "enum": ["dog", "cat", "other"],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required":["birthday", "class", "name"],
                      "type":"object"
                    }
                  },
                  "paths": {
                    "/api/echo1": {
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
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          },
                        },
                        "security": [
                          {
                            "JWT": []
                          }
                        ]
                      }
                    },
                    "/api/echo2": {
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
                            "description": "OK",
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          },
                        },
                        "security": [
                          {
                            "MyOAuth2": ["pets.read", "pets.write"]
                          }
                        ]
                      }
                    },
                  },
                  "securityDefinitions": {
                    "JWT": {
                        "description":"Use format 'Bearer TOKEN'",
                        "in": "header",
                        "name": "Authorization",
                        "type": "apiKey"
                    },
                    "MyOAuth2": {
                        "scopes": {
                          "pets.read": "pets.read",
                          "pets.write": "pets.write"
                        },
                        "type": "oauth2",
                        "authorizationUrl": "http://example.com/",
                        "tokenUrl": "http://example.com/token",
                        "flow": "password"
                    }
                  },
                  "swagger": "2.0"
                }),
            );
        },
    );
}

#[test]
fn test_header_parameter_app() {
    #[derive(Apiv2Header, Deserialize)]
    #[allow(dead_code)]
    struct RequestHeaders {
        #[openapi(name = "X-Request-ID", description = "Allow to track request")]
        request_id: Uuid,
        #[openapi(description = "User organization slug")]
        slug: String,
        #[openapi(description = "User ip", format = "ip")]
        request_ip: String,
        /// Origin of the request
        origin: String,
        #[openapi(skip)]
        another_field: String,
    }

    impl FromRequest for RequestHeaders {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
            ready(Ok(Self {
                request_id: Uuid::default(),
                slug: "abc".to_owned(),
                request_ip: "127.1".to_owned(),
                origin: "test.com".to_owned(),
                another_field: "".to_owned(),
            }))
        }
    }

    #[derive(Apiv2Header, Deserialize)]
    struct RefererHeader(#[openapi(name = "X-Referer-slug")] String);

    impl FromRequest for RefererHeader {
        type Error = Error;
        type Future = Ready<Result<Self, Self::Error>>;
        #[cfg(not(feature = "actix4"))]
        type Config = ();

        fn from_request(_: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
            ready(Ok(Self("www.paperclip.rs".to_owned())))
        }
    }

    #[api_v2_operation]
    async fn echo_pet_with_headers(
        _: RequestHeaders,
        _: RefererHeader,
        body: web::Json<Pet>,
    ) -> web::Json<Pet> {
        body
    }

    fn config(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/echo").route(web::post().to(echo_pet_with_headers)));
    }

    run_and_check_app(
        move || {
            App::new()
                .wrap_api()
                .service(web::scope("/api").configure(config))
                .with_json_spec_at("/spec")
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "birthday": {
                          "format": "date",
                          "type": "string"
                        },
                        "class": {
                          "enum": [
                            "dog",
                            "cat",
                            "other"
                          ],
                          "type": "string"
                        },
                        "id": {
                          "format": "int64",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        },
                        "updatedOn": {
                          "format": "date-time",
                          "type": "string"
                        },
                        "uuid0": {
                          "format": "uuid",
                          "type": "string"
                        },
                        "uuid1": {
                          "format": "uuid",
                          "type": "string"
                        }
                      },
                      "required": [
                        "birthday",
                        "class",
                        "name"
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/api/echo": {
                      "post": {
                        "parameters": [
                          {
                            "description": "Allow to track request",
                            "format": "uuid",
                            "in": "header",
                            "name": "X-Request-ID",
                            "required": true,
                            "type": "string"
                          },
                          {
                            "description": "User organization slug",
                            "in": "header",
                            "name": "slug",
                            "required": true,
                            "type": "string"
                          },
                          {
                            "description": "User ip",
                            "format": "ip",
                            "in": "header",
                            "name": "request_ip",
                            "required": true,
                            "type": "string"
                          },
                          {
                            "description": "Origin of the request",
                            "in": "header",
                            "name": "origin",
                            "required": true,
                            "type": "string"
                          },
                          {
                            "in": "header",
                            "name": "X-Referer-slug",
                            "required": true,
                            "type": "string"
                          },
                          {
                            "in": "body",
                            "name": "body",
                            "required": true,
                            "schema": {
                              "$ref": "#/definitions/Pet"
                            }
                          }
                        ],
                        "responses": {
                          "200": {
                            "description": "OK",
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
fn test_method_macro() {
    #[get("/v0/pets")]
    #[api_v2_operation]
    fn get_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        futures::future::ready(Ok(web::Json(Default::default())))
    }
    #[put("/v0/pets/{name}")]
    #[api_v2_operation]
    fn put_pet(
        _name: web::Path<String>,
        pet: web::Json<Pet>,
    ) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }
    #[patch("/v0/pets/{name}")]
    #[api_v2_operation]
    fn patch_pet(
        _name: web::Path<String>,
        pet: web::Json<Pet>,
    ) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }
    #[post("/v0/pets")]
    #[api_v2_operation]
    fn post_pet(pet: web::Json<Pet>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }
    #[delete("/v0/pets/{name}")]
    #[api_v2_operation]
    fn delete_pet(_name: web::Path<String>) -> impl Future<Output = Result<web::Json<()>, Error>> {
        futures::future::ready(Ok(web::Json(())))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(get_pets)
                .service(put_pet)
                .service(patch_pet)
                .service(post_pet)
                .service(delete_pet)
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "Pet": {
                            "description": "Pets are awesome!",
                            "properties": {
                                "class": {
                                "enum": ["dog", "cat", "other"],
                                    "type":"string"
                                },
                                "id": {
                                    "format": "int64",
                                    "type": "integer"
                                },
                                "name": {
                                    "description": "Pick a good one.",
                                    "type": "string"
                                },
                                "birthday": {
                                  "format": "date",
                                  "type": "string"
                                },
                                "updatedOn": {
                                    "format": "date-time",
                                    "type": "string"
                                },
                                "uuid0":{
                                  "format":"uuid",
                                  "type":"string"
                                },
                                "uuid1":{
                                  "format":"uuid",
                                  "type":"string"
                                }
                            },
                            "required":[
                                "birthday",
                                "class",
                                "name"
                            ],
                            "type":"object"
                        }
                    },
                    "info": {
                        "title":"",
                        "version":""
                    },
                    "paths": {
                        "/v0/pets": {
                            "get": {
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "items": {
                                                "$ref": "#/definitions/Pet"
                                            },
                                            "type": "array"
                                        }
                                    }
                                },
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
                            }
                        },
                        "/v0/pets/{name}": {
                            "delete": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                        }
                                    }
                                },
                            },
                            "put": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
                            },
                            "patch": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
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
fn test_method_macro_subscope() {
    #[get("")]
    #[api_v2_operation]
    fn get_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        futures::future::ready(Ok(web::Json(Default::default())))
    }
    #[put("/{name}")]
    #[api_v2_operation]
    fn put_pet(
        _name: web::Path<String>,
        pet: web::Json<Pet>,
    ) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }
    #[post("")]
    #[api_v2_operation]
    fn post_pet(pet: web::Json<Pet>) -> impl Future<Output = Result<web::Json<Pet>, Error>> {
        futures::future::ready(Ok(pet))
    }
    #[delete("/{name}")]
    #[api_v2_operation]
    fn delete_pet(_name: web::Path<String>) -> impl Future<Output = Result<web::Json<()>, Error>> {
        futures::future::ready(Ok(web::Json(())))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/api/spec")
                .service(
                    web::scope("/v0/pets")
                        .service(get_pets)
                        .service(put_pet)
                        .service(post_pet)
                        .service(delete_pet),
                )
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "Pet": {
                            "description": "Pets are awesome!",
                            "properties": {
                                "class": {
                                "enum": ["dog", "cat", "other"],
                                    "type":"string"
                                },
                                "id": {
                                    "format": "int64",
                                    "type": "integer"
                                },
                                "name": {
                                    "description": "Pick a good one.",
                                    "type": "string"
                                },
                                "birthday": {
                                  "format": "date",
                                  "type": "string"
                                },
                                "updatedOn": {
                                    "format": "date-time",
                                    "type": "string"
                                },
                                "uuid0":{
                                  "format":"uuid",
                                  "type":"string"
                                },
                                "uuid1":{
                                  "format":"uuid",
                                  "type":"string"
                                }
                            },
                            "required":[
                                "birthday",
                                "class",
                                "name"
                            ],
                            "type":"object"
                        }
                    },
                    "info": {
                        "title":"",
                        "version":""
                    },
                    "paths": {
                        "/v0/pets/": {
                            "get": {
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "items": {
                                                "$ref": "#/definitions/Pet"
                                            },
                                            "type": "array"
                                        }
                                    }
                                },
                            },
                            "post": {
                                "parameters": [
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
                            }
                        },
                        "/v0/pets/{name}": {
                            "delete": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                        }
                                    }
                                },
                            },
                            "put": {
                                "parameters": [
                                    {
                                        "in": "path",
                                        "name": "name",
                                        "required": true,
                                        "type": "string"
                                    },
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet"
                                        }
                                    }
                                },
                            }
                        }
                    },
                    "swagger": "2.0"
                }),
            );
        },
    );
}

#[cfg(feature = "actix4")]
fn run_and_check_app<F, G, T, B, U>(factory: F, check: G) -> U
where
    F: Fn() -> App<T> + Clone + Send + Sync + 'static,
    B: MessageBody + 'static,
    T: ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<B>,
            Error = Error,
            InitError = (),
        > + 'static,
    G: Fn(String) -> U,
{
    let (tx, rx) = mpsc::channel();

    let _ = thread::spawn(move || {
        for port in 3000..30000 {
            if !PORTS.lock().unwrap().insert(port) {
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

            tx.send(addr).unwrap();

            System::new().block_on(async move {
                let _ = server.run().await;
            });
            // break;
        }

        unreachable!("No ports???");
    });

    let addr = rx.recv().unwrap();
    let ret = check(addr);
    ret
}

#[cfg(not(feature = "actix4"))]
fn run_and_check_app<F, G, T, B, U>(factory: F, check: G) -> U
where
    F: Fn() -> App<T, B> + Clone + Send + Sync + 'static,
    B: MessageBody + 'static,
    T: ServiceFactory<
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
            if !PORTS.lock().unwrap().insert(port) {
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

            let s = server.run();
            tx.send((s, addr)).unwrap();
            sys.run().expect("system error?");
            return;
        }

        unreachable!("No ports???");
    });

    let (_server, addr) = rx.recv().unwrap();
    let ret = check(addr);
    let _ = _server.stop(true);
    ret
}

fn check_json(resp: reqwest::blocking::Response, expected: serde_json::Value) {
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

#[cfg(feature = "v3")]
#[test]
fn test_openapi3() {
    let spec = std::fs::File::open("tests/pet-v2.yaml").unwrap();
    let spec: DefaultApiRaw = serde_yaml::from_reader(spec).unwrap();
    let _spec_v3: openapiv3::OpenAPI = spec.into();
}

#[test]
fn test_rename() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[serde(rename_all = "camelCase")]
    #[openapi(rename = "PetRenamed")]
    /// Pets are awesome!
    struct Pet {
        /// Pick a good one.
        name: String,
    }

    #[get("/pets")]
    #[api_v2_operation]
    fn echo_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .service(echo_pets)
                .with_raw_json_spec(|app, spec| {
                    app.route(
                        "/api/spec",
                        web::get().to(move || {
                            #[cfg(feature = "actix4")]
                            {
                                let spec = spec.clone();
                                async move {
                                    paperclip::actix::HttpResponseWrapper(
                                        actix_web::HttpResponse::Ok().json(&spec),
                                    )
                                }
                            }

                            #[cfg(not(feature = "actix4"))]
                            actix_web::HttpResponse::Ok().json(&spec)
                        }),
                    )
                })
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "PetRenamed": {
                            "description": "Pets are awesome!",
                            "properties": {
                                "name": {
                                    "description": "Pick a good one.",
                                    "type": "string"
                                },
                            },
                            "required":[
                                "name"
                            ],
                            "type":"object"
                        }
                    },
                    "info": {
                        "title":"",
                        "version":""
                    },
                    "paths": {
                        "/pets": {
                            "get": {
                                "responses": {
                                "200": {
                                    "description": "OK",
                                    "schema": {
                                        "items": {
                                            "$ref": "#/definitions/PetRenamed"
                                        },
                                        "type": "array"
                                    }
                                }
                                },
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
fn test_example() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[openapi(example = r#"{ "name": "Rex", "age": 8 }"#)]
    /// Pets are awesome!
    struct Pet {
        /// Pick a good one.
        name: String,
        /// 7 time yours
        age: u8,
    }

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct Car {
        /// Pick a good one.
        #[openapi(example = "whatever")]
        name: String,
    }

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    struct ImageFile {
        /// filename.
        #[openapi(example = "batman.png")]
        path: PathBuf,
    }

    #[api_v2_operation]
    fn echo_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    #[api_v2_operation]
    fn echo_cars() -> impl Future<Output = Result<web::Json<Vec<Car>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    #[api_v2_operation]
    fn echo_files() -> impl Future<Output = Result<web::Json<Vec<ImageFile>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .route("/pets", web::get().to(echo_pets))
                .route("/cars", web::get().to(echo_cars))
                .route("/files", web::get().to(echo_files))
                .with_json_spec_at("/api/spec")
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Car": {
                      "properties": {
                        "name": {
                          "description": "Pick a good one.",
                          "example": "whatever",
                          "type": "string"
                        }
                      },
                      "required": [
                        "name"
                      ],
                      "type": "object"
                    },
                    "Pet": {
                      "description": "Pets are awesome!",
                      "example": {
                        "age": 8,
                        "name": "Rex"
                      },
                      "properties": {
                        "age": {
                          "description": "7 time yours",
                          "format": "int32",
                          "type": "integer"
                        },
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        }
                      },
                      "required": [
                        "age",
                        "name"
                      ],
                      "type": "object"
                    },
                    "ImageFile": {
                      "properties": {
                        "path": {
                          "description": "filename.",
                          "example": "batman.png",
                          "type": "string"
                        }
                      },
                      "required": [
                        "path",
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/cars": {
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Car"
                              },
                              "type": "array"
                            }
                          }
                        }
                      }
                    },
                    "/pets": {
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Pet"
                              },
                              "type": "array"
                            }
                          }
                        }
                      }
                    },
                    "/files": {
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/ImageFile"
                              },
                              "type": "array"
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

mod module_path_in_definition_name {
    pub mod foo {
        pub mod bar {
            #[derive(serde::Serialize, paperclip::actix::Apiv2Schema)]
            pub struct Baz {
                pub a: i32,
                pub b: i32,
            }
        }

        pub mod other_bar {
            use std::path::PathBuf;

            #[derive(serde::Serialize, paperclip::actix::Apiv2Schema)]
            pub struct Baz {
                pub a: String,
                pub b: bool,
                pub c: PathBuf,
            }
        }
    }
}

#[test]
fn test_schema_with_r_literals() {
    use paperclip::v2::schema::Apiv2Schema;
    #[derive(paperclip::actix::Apiv2Schema, Deserialize, Serialize)]
    pub(crate) struct Dog {
        /// The voice that we love and hate
        pub(crate) r#bark: String,
    }

    let dog = Dog::raw_schema();
    assert_eq!(
        "bark",
        dog.properties.iter().next().map(|(k, _v)| k).unwrap()
    );
}

#[test]
fn test_schema_with_generics() {
    /// Our non-human family member
    #[derive(Apiv2Schema, Deserialize, Serialize)]
    pub(crate) struct Pet<T> {
        /// Fluffy or Fido or...
        pub(crate) name: String,
        /// So we can find her/him when we need to
        pub(crate) id: Option<i64>,
        /// The attributes unique to this type of pet
        pub(crate) inner: T,
    }

    /// An affectionate (but noisy) best friend
    #[derive(Apiv2Schema, Deserialize, Serialize)]
    pub(crate) struct Dog {
        /// The voice that we love and hate
        pub(crate) bark: String,
    }

    /// A lovely cat who loves to eat!
    #[derive(Apiv2Schema, Deserialize, Serialize)]
    pub(crate) struct Cat {
        /// Mmmmmmmmmm
        pub(crate) food_pref: String,
    }

    #[post("/dogs")]
    #[api_v2_operation]
    pub(crate) async fn echo_dogs(body: web::Json<Pet<Dog>>) -> Result<web::Json<Pet<Dog>>, Error> {
        Ok(body)
    }

    #[post("/cats")]
    #[api_v2_operation]
    pub(crate) async fn echo_cats(body: web::Json<Pet<Cat>>) -> Result<web::Json<Pet<Cat>>, Error> {
        Ok(body)
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .service(echo_dogs)
                .service(echo_cats)
                .with_raw_json_spec(|app, spec| {
                    app.route(
                        "/api/spec",
                        web::get().to(move || {
                            #[cfg(feature = "actix4")]
                            {
                                let spec = spec.clone();
                                async move {
                                    paperclip::actix::HttpResponseWrapper(
                                        actix_web::HttpResponse::Ok().json(&spec),
                                    )
                                }
                            }

                            #[cfg(not(feature = "actix4"))]
                            actix_web::HttpResponse::Ok().json(&spec)
                        }),
                    )
                })
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "Pet<Cat>": {
                            "description": "Our non-human family member",
                            "properties": {
                                "id": {
                                    "description": "So we can find her/him when we need to",
                                    "format": "int64",
                                    "type": "integer",
                                },
                                "inner": {
                                    "description": "The attributes unique to this type of pet",
                                    "properties": {
                                        "food_pref": {
                                            "description": "Mmmmmmmmmm",
                                            "type": "string",
                                        },
                                    },
                                    "required": [
                                        "food_pref",
                                    ],
                                    "type": "object",
                                },
                                "name": {
                                    "description": "Fluffy or Fido or...",
                                    "type": "string",
                                },
                            },
                            "required": [
                                "inner",
                                "name",
                            ],
                            "type":"object",
                        },
                        "Pet<Dog>": {
                            "description": "Our non-human family member",
                            "properties": {
                                "id": {
                                    "description": "So we can find her/him when we need to",
                                    "format": "int64",
                                    "type": "integer",
                                },
                                "inner": {
                                    "description": "The attributes unique to this type of pet",
                                    "properties": {
                                        "bark": {
                                            "description": "The voice that we love and hate",
                                            "type": "string",
                                        },
                                    },
                                    "required": [
                                        "bark",
                                    ],
                                    "type": "object",
                                },
                                "name": {
                                    "description": "Fluffy or Fido or...",
                                    "type": "string",
                                },
                            },
                            "required": [
                                "inner",
                                "name",
                            ],
                            "type":"object",
                        },
                    },
                    "info": {
                        "title":"",
                        "version":"",
                    },
                    "paths": {
                        "/cats": {
                            "post": {
                                "parameters": [
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet%3CCat%3E",
                                        },
                                    },
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet%3CCat%3E",
                                        },
                                    },
                                },
                            },
                        },
                        "/dogs": {
                            "post": {
                                "parameters": [
                                    {
                                        "in": "body",
                                        "name": "body",
                                        "required": true,
                                        "schema": {
                                            "$ref": "#/definitions/Pet%3CDog%3E",
                                        },
                                    },
                                ],
                                "responses": {
                                    "200": {
                                        "description": "OK",
                                        "schema": {
                                            "$ref": "#/definitions/Pet%3CDog%3E",
                                        },
                                    },
                                },
                            },
                        },
                    },
                    "swagger": "2.0",
                }),
            );
        },
    );
}

#[test]
#[cfg(feature = "path-in-definition")]
fn test_module_path_in_definition_name() {
    use paperclip::actix::{api_v2_operation, web, OpenApiExt};

    #[api_v2_operation]
    fn a() -> web::Json<module_path_in_definition_name::foo::bar::Baz> {
        web::Json(module_path_in_definition_name::foo::bar::Baz { a: 10, b: 10 })
    }

    #[api_v2_operation]
    fn b() -> web::Json<module_path_in_definition_name::foo::other_bar::Baz> {
        web::Json(module_path_in_definition_name::foo::other_bar::Baz {
            a: String::default(),
            b: true,
            c: PathBuf::default(),
        })
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .with_json_spec_at("/spec")
                .route("/a", web::get().to(a))
                .route("/b", web::get().to(b))
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                      "module_path_in_definition_name_foo_bar_Baz": {
                        "properties": {
                          "a": {
                            "format": "int32",
                            "type": "integer"
                          },
                          "b": {
                            "format": "int32",
                            "type": "integer"
                          }
                        },
                        "required": [
                          "a",
                          "b"
                        ],
                        "type": "object"
                      },
                      "module_path_in_definition_name_foo_other_bar_Baz": {
                        "properties": {
                          "a": {
                            "type": "string"
                          },
                          "b": {
                            "type": "boolean"
                          },
                          "c": {
                            "type": "string"
                          }
                        },
                        "required": [
                          "a",
                          "b",
                          "c"
                        ],
                        "type": "object"
                      }
                    },
                    "info": {
                      "title": "",
                      "version": ""
                    },
                    "paths": {
                      "/a": {
                        "get": {
                          "responses": {
                            "200": {
                              "description": "OK",
                              "schema": {
                                "$ref": "#/definitions/module_path_in_definition_name_foo_bar_Baz"
                              }
                            }
                          }
                        }
                      },
                      "/b": {
                        "get": {
                          "responses": {
                            "200": {
                              "description": "OK",
                              "schema": {
                                "$ref": "#/definitions/module_path_in_definition_name_foo_other_bar_Baz"
                              }
                            }
                          }
                        }
                      }
                    },
                    "swagger": "2.0"
                }),
            )
        },
    )
}

#[test]
fn test_ipvx() {
    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[serde(rename_all = "camelCase")]
    /// Pets are awesome!
    struct Pet {
        /// Pick a good one.
        name: String,
        /// An Ip address.
        ip: std::net::IpAddr,
        /// An IpV4 address.
        ip_v4: std::net::Ipv4Addr,
        /// An IpV6 address.
        ip_v6: std::net::Ipv6Addr,
    }

    #[get("/pets")]
    #[api_v2_operation]
    fn echo_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .service(echo_pets)
                .with_raw_json_spec(|app, spec| {
                    app.route(
                        "/api/spec",
                        web::get().to(move || {
                            #[cfg(feature = "actix4")]
                            {
                                let spec = spec.clone();
                                async move {
                                    paperclip::actix::HttpResponseWrapper(
                                        actix_web::HttpResponse::Ok().json(&spec),
                                    )
                                }
                            }

                            #[cfg(not(feature = "actix4"))]
                            actix_web::HttpResponse::Ok().json(&spec)
                        }),
                    )
                })
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                    "definitions": {
                        "Pet": {
                            "description": "Pets are awesome!",
                            "properties": {
                                "name": {
                                    "description": "Pick a good one.",
                                    "type": "string"
                                },
                                "ip": {
                                    "description": "An Ip address.",
                                    "format": "ip",
                                    "type": "string"
                                },
                                "ipV4": {
                                    "description": "An IpV4 address.",
                                    "format": "ipv4",
                                    "type": "string"
                                },
                                "ipV6": {
                                    "description": "An IpV6 address.",
                                    "format": "ipv6",
                                    "type": "string"
                                }
                            },
                            "required": [
                                "ip",
                                "ipV4",
                                "ipV6",
                                "name"
                            ],
                            "type" : "object"
                        }
                    },
                    "info": {
                        "title":"",
                        "version":""
                    },
                    "paths": {
                        "/pets": {
                            "get": {
                                "responses": {
                                "200": {
                                    "description": "OK",
                                    "schema": {
                                        "items": {
                                            "$ref": "#/definitions/Pet"
                                        },
                                        "type": "array"
                                    }
                                }
                                },
                            }
                        }
                    },
                    "swagger": "2.0"
                }),
            );
        },
    );
}

#[cfg(any(feature = "actix3", feature = "actix4"))]
#[test]
fn test_wrap() {
    #[cfg(not(feature = "actix4"))]
    extern crate actix_web_httpauth3 as actix_web_httpauth;
    #[cfg(feature = "actix4")]
    extern crate actix_web_httpauth4 as actix_web_httpauth;

    #[derive(Deserialize, Serialize, Apiv2Schema)]
    #[serde(rename_all = "camelCase")]
    /// Pets are awesome!
    struct Pet {
        /// Pick a good one.
        name: String,
    }

    #[api_v2_operation]
    fn echo_pets() -> impl Future<Output = Result<web::Json<Vec<Pet>>, Error>> {
        fut_ok(web::Json(vec![]))
    }

    run_and_check_app(
        || {
            App::new()
                .wrap_api()
                .service(
                    web::resource("/pets")
                        .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                            |req, _credentials| async { Ok(req) },
                        ))
                        .route(web::get().to(echo_pets)),
                )
                .with_json_spec_at("/api/spec")
                .build()
        },
        |addr| {
            let resp = CLIENT
                .get(&format!("http://{}/api/spec", addr))
                .send()
                .expect("request failed?");

            check_json(
                resp,
                json!({
                  "definitions": {
                    "Pet": {
                      "description": "Pets are awesome!",
                      "properties": {
                        "name": {
                          "description": "Pick a good one.",
                          "type": "string"
                        }
                      },
                      "required": [
                        "name"
                      ],
                      "type": "object"
                    }
                  },
                  "info": {
                    "title": "",
                    "version": ""
                  },
                  "paths": {
                    "/pets": {
                      "get": {
                        "responses": {
                          "200": {
                            "description": "OK",
                            "schema": {
                              "items": {
                                "$ref": "#/definitions/Pet"
                              },
                              "type": "array"
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
