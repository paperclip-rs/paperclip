# Paperclip

[![Build Status](https://api.travis-ci.org/wafflespeanut/paperclip.svg?branch=master)](https://travis-ci.org/wafflespeanut/paperclip)
[![API docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://paperclip.waffles.space/paperclip_openapi)

WIP OpenAPI code generator for type-safe compile-time checked HTTP APIs in Rust.

## Features

The following features are supported at the moment:

 - Generates API objects from schemas in an OpenAPI v2 spec.
 - Generates builder structs for the API objects and HTTP operations.
 - Fulfilled builder structs send API calls and return response futures (only `application/json` is supported as of now).

See the [projects](https://github.com/wafflespeanut/paperclip/projects) for tracking the features in queue.

## Build script example

Assuming you already have an OpenAPI v2 spec, let's generate code through a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

### `Cargo.toml`

```toml
[package]
name = "my_awesome_app"
version = "0.1.0"
authors = ["Me <me@example.com>"]
edition = "2018"
build = "build.rs"

[dependencies]
# NOTE: These are the crates required by the generated code!
failure = "0.1"
failure_derive = "0.1"
futures = "0.1"
reqwest = "0.9"
serde = "1.0"
serde_derive = "1.0"
# Other crates I need...
tokio = "0.1"

[build-dependencies]
paperclip = "0.1.0"
```

### `build.rs`

> Here, I'm using the [kubernetes spec I already have in tree](./openapi/tests/k8s-v1.16.0-alpha.0-openapi-v2.json).

```rust
use paperclip::v2::{
    self,
    codegen::{DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};

use std::env;
use std::fs::File;

fn main() {
    // Your spec path here.
    let fd = File::open("../paperclip/openapi/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("schema?");
    let raw: Api<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
    let schema = raw.resolve().expect("resolution");

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut state = EmitterState::default();
    state.working_dir = out_dir.into();

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&schema).expect("codegen");
}
```

### `src/main.rs`

```rust
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

mod io {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/io/mod.rs"));
}

use self::io::client::Sendable;
use self::io::k8s::api::core::v1::node_list::NodeList;

use futures::Future;
use reqwest::r#async::Client;

fn main() {
    let client = Client::new();
    let f = NodeList::get()
        .limit(10)
        .send(&client);

    // NOTE: For Kubernetes, this works only if TLS is disabled!
    tokio::run(f.map(|list| {
        println!("{:?}", list);
    }).map_err(|e| {
        println!("{:?}", e);
    }));
}
```

## Motivation

While [Serde](https://serde.rs/) makes it amazingly easy to write API objects, only the official codegen [supports generating proper APIs](https://github.com/swagger-api/swagger-codegen/tree/dedb5ce36d54495365da9a7d88d1e6e056cfe29f/samples/client/petstore/rust) and leverages the builder pattern for building API requests. I think it should be really easy to build type-safe APIs from OpenAPI specifications using pure Rust.

## Developing locally

 - Make sure you have [`rustup`](https://rustup.rs/) installed. `cd` into this repository and run `make prepare` to setup your environment.
 - Now run `make` to build and run the tests.

## Contributing

This project welcomes all kinds of contributions. No contribution is too small!

If you really wish to contribute to this project but don't know how to begin or if you need help with something related to this project, then feel free to send me an email or ping me in Discord (same handle).

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## FAQ

> Why is this generating raw Rust code instead of leveraging [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) for compile-time codegen?

I don't think proc macros are the right way to go for REST APIs. We need to be able to **see** the generated code somehow to identify names, fields, supported methods, etc. ([like this](https://paperclip.waffles.space/tests/test_k8s/api/)). With proc macros, you sorta have to guess.

This doesn't mean you can't generate APIs in compile-time. The only difference is that you'll be using [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) instead and `include!` the relevant code. That said, [we're using proc-macros](./macros) for other things.
