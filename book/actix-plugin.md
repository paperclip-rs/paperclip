# Host OpenAPI v2 spec through actix-web

With `actix` feature enabled, paperclip exports an **experimental** plugin for [actix-web](https://github.com/actix/actix-web) framework to host OpenAPI v2 spec for your APIs *automatically*. While it's not feature complete, you can rely on it to not break your actix-web flow.

Let's start with a simple actix-web application. It has `actix-web` and `serde` for JSON'ifying your APIs. Let's also add `paperclip` with `actix` feature.

```toml
# [package] ignored for brevity

[dependencies]
# actix-web 2.0 is also supported
actix-web = "3.0"
# The "actix-nightly" feature can be specified if you're using nightly compiler. Even though
# this plugin works smoothly with the nightly compiler, it also works in stable
# channel (replace "actix-nightly" feature with "actix" in that case). There maybe compilation errors,
# but those can be fixed.
paperclip = { version = "0.4", features = ["actix-nightly"] }
serde = "1.0"
```

Our `main.rs` looks like this:

```rust
use actix_web::{App, HttpServer, web::{self, Json}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    id: Option<i64>,
}

async fn echo_pet(body: Json<Pet>) -> Result<Json<Pet>, ()> {
    Ok(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(
            web::resource("/pets")
                .route(web::post().to(echo_pet))
        )
    ).bind("127.0.0.1:8080")?
    .run().await
}
```

Now, let's modify it to use the plugin!

```rust
use actix_web::{App, HttpServer};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, Apiv2Schema, api_v2_operation,
    // use this instead of actix_web::web
    web::{self, Json},
};
use serde::{Serialize, Deserialize};

// Mark containers (body, query, parameter, etc.) like so...
#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Pet {
    name: String,
    id: Option<i64>,
}

// Mark operations like so...
#[api_v2_operation]
async fn echo_pet(body: Json<Pet>) -> Result<Json<Pet>, ()> {
    Ok(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        // Record services and routes from this line.
        .wrap_api()
        // Add routes like you normally do...
        .service(
            web::resource("/pets")
                .route(web::post().to(echo_pet))
        )
        // Mount the JSON spec at this path.
        .with_json_spec_at("/api/spec")

        // ... or if you wish to build the spec by yourself...

        // .with_raw_json_spec(|app, spec| {
        //     app.route("/api/spec", web::get().to(move || {
        //         actix_web::HttpResponse::Ok().json(&spec)
        //     }))
        // })

        // IMPORTANT: Build the app!
        .build()
    ).bind("127.0.0.1:8080")?
    .run().await
}
```

We have:

 - Imported `OpenApiExt` extension trait for `actix_web::App` along with `Apiv2Schema` derive macro and `api_v2_operation` proc macro attributes.
 - Switched from `actix_web::web` to `paperclip::actix::web`.
 - Marked our `Pet` struct and `add_pet` function as OpenAPI-compatible schema and operation using proc macro attributes.
 - Transformed our `actix_web::App` to a wrapper using `.wrap_api()`.
 - Mounted the JSON spec at a relative path using `.with_json_spec_at("/api/spec")`.
 - Built (using `.build()`) and passed the original `App` back to actix-web.

Note that we never touched the service, resources, routes or anything else! This means that our original actix-web flow is unchanged.

Now you can check the API with the following **cURL** command:

```
curl -X POST http://localhost:8080/pets -H "Content-Type: application/json" -d '{"id":1,"name":"Felix"}'
```

And see the specs with this:

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
            "description": "OK",
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
  },
  "info": {
    "version": "",
    "title": ""
  }
}
```

Similarly, if we were to use other extractors like `web::Query<T>`, `web::Form<T>` or `web::Path`, the plugin will emit the corresponding specification as expected.

#### Operation metadata

By default, the first doc comment (if any) is taken for the `summary` field and the rest of the following doc comments
(if any) will be taken as `description` for that operation.

```rust
/// Default
/// multiline
/// summary
///
/// Default
/// multiline
/// description
async fn my_handler() -> Json<Foo> { /* */ }
```

This can be overridden by explicitly specifying `summary` and `description` in the proc-macro attribute like so:

```rust
#[api_v2_operation(
  summary = "My awesome handler",
  description = "It creates a pretty JSON object",
  /// A few other parameters are also supported
  operation_id = "my_handler",
  consumes = "application/yaml, application/json",
  produces = "application/yaml, application/json",
)]
async fn my_handler() -> Json<Foo> { /* */ }
```

#### Manually defining additional response codes

There is a macro `api_v2_errors` which helps to manually add error (non-2xx) response codes.

```rust
use paperclip::actix::api_v2_errors;

#[api_v2_errors(
    code=400,
    code=401, description="Unauthorized: Can't read session from header",
    code=500,
)]
pub enum MyError {
    /* ... */
}
```

You can now use this error in handlers and they'll emit operations containing those response codes.

```rust
#[api_v2_operation]
async fn my_handler() -> Result<(), MyError> {
    /* ... */
}
```

#### Defining security

Use `Apiv2Security` derive macro for struct used as handler parameter to have this handler marked as requiring authorization.

```rust
use paperclip::actix::Apiv2Security;

#[derive(Apiv2Security)]
#[openapi(
  apiKey,
  in = "header",
  name = "Authorization",
  description = "Use format 'Bearer TOKEN'"
)]
pub struct AccessToken;

impl FromRequest for Accesstoken { /*...*/ }

#[api_v2_operation]
async fn my_handler(access_token: AccessToken) -> Result<String, MyError> {
    /*...*/
}
```

First parameter is the type of security, currently supported types are "apiKey" and "oauth2". Possible parameters are `alias`, `description`, `name`, `in`, `flow`, `auth_url`, `token_url` or `parent`.

Use `alias` parameter if you need to have two different security definitions of the same type.

If you need to define scopes for `oauth2`, use `parent` attribute:

```rust
#[derive(Apiv2Security, Deserialize)]
#[openapi(
  oauth2,
  auth_url = "http://example.com/",
  token_url = "http://example.com/token",
  flow = "password"
)]
struct OAuth2Access;

#[derive(Apiv2Security, Deserialize)]
#[openapi(parent = "OAuth2Access", scopes("pets.read", "pets.write"))]
struct PetScopeAccess;
```

#### Known limitations

- **Enums:** OpenAPI (v2) itself supports using simple enums (i.e., with unit variants), but Rust and serde has support for variants with fields and tuples. I still haven't looked deep enough either to say whether this can/cannot be done in OpenAPI or find an elegant way to represent this in OpenAPI.
- **Functions returning abstractions:** The plugin has no way to obtain any useful information from functions returning abstractions such as `HttpResponse`, `impl Responder` or containers such as `Result<T, E>` containing those abstractions. So currently, the plugin silently ignores these types, which results in an empty value in your hosted specification.

#### Missing features

At the time of this writing, this plugin didn't support a few OpenAPI v2 features:

Affected entity | Missing feature(s)
--------------- | ---------------
[Parameter](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object) | Non-body parameters allowing validations like `allowEmptyValue`, `collectionFormat`, `items`, etc.
[Parameter](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object) | Headers as parameters.

#### Performance implications?

Even though we use some wrappers and generate schema structs for building the spec, we do this only once i.e., until the `.build()` function call. At runtime, it's basically a pointer read, which is quite fast!

We also add wrappers to blocks in functions tagged with `#[api_v2_operation]`, but those wrappers follow the [Newtype pattern](https://doc.rust-lang.org/stable/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) and the code eventually gets optimized away anyway.
