# Host OpenAPI spec through actix-web

With `actix` feature enabled, paperclip exports an **experimental** plugin for [actix-web](https://github.com/actix/actix-web) framework to host OpenAPI spec for your APIs *automatically*. While it's not feature complete, you can rely on it to not break your actix-web flow.

Let's start with a simple actix-web application. It has `actix-web` and `serde` for JSON'ifying your APIs. Let's also add `paperclip` with `actix` feature.

```toml
# [package] ignored for brevity

[dependencies]
# NOTE: Requires `actix-web = "^1.0.4"`
actix-web = "1.0.4"
paperclip = { version = "0.3", features = ["actix"] }
serde = "1.0"
```

Our `main.rs` looks like this:

```rust
use actix_web::{App, web::{self, Json}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    id: Option<i64>,
}

fn add_pet(_body: Json<Pet>) -> Json<Pet> {
    unimplemented!();
}

fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new()
        .service(
            web::resource("/pets")
                .route(web::post().to(add_pet))
        )
    ).bind("127.0.0.1:8080")?
    .run()
}
```

Now, let's modify it to use the plugin!

```rust
use actix_web::{App, HttpServer};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, api_v2_schema, api_v2_operation,
    // use this instead of actix_web::web
    web::{self, Json},
};
use serde::{Serialize, Deserialize};

// Mark containers (body, query, parameter, etc.) like so...
#[api_v2_schema]
#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    id: Option<i64>,
}

// Mark operations like so...
#[api_v2_operation]
fn add_pet(_body: Json<Pet>) -> Json<Pet> {
    unimplemented!();
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        // Record services and routes from this line.
        .wrap_api()
        // Mount the JSON spec at this path.
        .with_json_spec_at("/api/spec")
        // Everything else is the same!
        .service(
            web::resource("/pets")
                .route(web::post().to(add_pet))
        )
        // Build the app!
        .build()
    ).bind("127.0.0.1:8080")?
    .run()
}
```

We have:

 - Imported `OpenApiExt` extension trait for `actix_web::App` along with `api_v2_schema` and `api_v2_operation` proc macro attributes.
 - Switched from `actix_web::web` to `paperclip::actix::web`.
 - Marked our `Pet` struct and `add_pet` function as OpenAPI-compatible schema and operation using proc macro attributes.
 - Transformed our `actix_web::App` to a wrapper using `.wrap_api()`.
 - Mounted the JSON spec at a relative path using `.with_json_spec_at("/api/spec")`.
 - Built (using `.build()`) and passed the original `App` back to actix-web.

Note that we never touched the service, resources, routes or anything else! This means that our original actix-web flow is unchanged.

Now, testing with **cURL**:

```
curl http://localhost:8080/api/spec
```

... we get the swagger spec as a JSON!

```js
// NOTE: Formatted for clarity
{
  "swagger": "2.0",
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
      },
      "required": ["name"]
    }
  },
  "paths": {
    "/pets": {
      "post": {
        "responses": {
          "200": {
            "schema": {
              "$ref": "#/definitions/Pet"
            }
          }
        }
      },
      "parameters": [{
        "in": "body",
        "name": "body",
        "required": true,
        "schema": {
          "$ref": "#/definitions/Pet"
        }
      }]
    }
  }
}
```

Similarly, if we were to use other extractors like `web::Query<T>`, `web::Form<T>` or `web::Path`, the plugin will emit the corresponding specification as expected.

#### Known Limitations

At the time of this writing, this plugin didn't support a number of OpenAPI features:

Affected entity | Missing feature(s)
--------------- | ---------------
[Parameter](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object) | Non-body parameters allowing validations like `allowEmptyValue`, `collectionFormat`, `items`, etc.
[Parameter](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object) | Headers as parameters.
[Responses](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responsesObject) | Response codes other than `200 OK` and `400 Bad Request`.
Security ([definitions](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityDefinitionsObject) and [requirements](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityRequirementObject)) | Authentication and Authorization.

#### Performance implications?

Even though we use a wrapper and generate schema structs for building the spec, we do this only once i.e., until the `.build()` function call. At runtime, it's basically an [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) deref and [`RwLock`](https://docs.rs/parking_lot/*/parking_lot/type.RwLock.html) read, which is quite fast!
