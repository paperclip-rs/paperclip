# Generate client code using build script

This example generates code using a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

- Create a new project using cargo: `cargo new my_awesome_app`

- Modify your `Cargo.toml` to look like this:

```toml
[package]
name = "my_awesome_app"
version = "0.1.0"
authors = ["Me <me@example.com>"]
edition = "2018"
build = "build.rs"

[dependencies]
# Crates required by the generated code!
failure = "0.1"
futures = "0.1"
reqwest = "0.9"
serde = "1.0"
# Other crates I'm using for this example.
futures-preview = { version = "0.3.0-alpha.16", features = ["compat"], package = "futures-preview" }
runtime = { git = "https://github.com/rustasync/runtime" }
runtime-tokio = { git = "https://github.com/rustasync/runtime" }

[build-dependencies]
paperclip = "0"
```

- Add `my-spec.yaml` to the project root with contents from [this file](https://raw.githubusercontent.com/wafflespeanut/paperclip/master/openapi/tests/pet-v2.yaml).

- Now, add `build.rs` to the project root with the following:

```rust
use paperclip::v2::{
    self,
    codegen::{DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};

use std::env;
use std::fs::File;

fn main() {
    let fd = File::open("my-spec.yaml").expect("schema?");
    let raw: Api<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
    let schema = raw.resolve().expect("resolution");

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut state = EmitterState::default();
    // prefix because we've isolated generated code (see main.rs).
    state.mod_prefix = "crate::codegen::";
    state.working_dir = out_dir.into();

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&schema).expect("codegen");
}
```

- Now, you can modify `src/main.rs` to make use of the generated code:

> **NOTE:** I'm using async/await only to demonstrate the usage of the generated code. The generated client code uses the old futures 0.1 and won't switch to the new syntax until it's stablilized.

```rust
#![feature(async_await)]

#[macro_use] extern crate failure;
#[macro_use] extern crate serde;

use failure::Error;
use futures_preview::compat::Future01CompatExt;
use reqwest::r#async::Client;

mod codegen {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use self::codegen::client::Sendable;
use self::codegen::pet::Pet;

#[runtime::main(runtime_tokio::Tokio)]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let pets = Pet::list_pets().send(&client).compat().await?;
    Ok(())
}
```
