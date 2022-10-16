# Generate client code using build script

This example generates API client code code using a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

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
# Crates required by the generated code
async-trait = "0.1"
bytes = "0.5"
thiserror = "1.0"
futures = "0.3"
http = "0.2"
lazy_static = "1.4"
log = "0.4"
mime = { git = "https://github.com/hyperium/mime" }
mime_guess = "2.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.8"
tokio-util = { version = "0.4", features = ["codec"] }
url = "2.1"
tokio = { version = "0.3", features = ["fs", "io-util", "macros", "rt-multi-thread"] }
reqwest = { version = "0.10", features = ["stream", "json"] }

[build-dependencies]
paperclip = { version = "0.5", features = ["v2", "codegen"] }
```

- Add `my-spec.yaml` to the project root with contents from [this file](https://raw.githubusercontent.com/paperclip-rs/paperclip/master/tests/pet-v2.yaml).

- Now, add `build.rs` to the project root with the following:

```rust
use paperclip::v2::{
    self,
    codegen::{DefaultEmitter, Emitter, EmitterState},
    models::{ResolvableApi, DefaultSchema},
};

use std::env;
use std::fs::File;

fn main() {
    let fd = File::open("my-spec.yaml").expect("schema?");
    let raw: ResolvableApi<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
    let schema = raw.resolve().expect("resolution");

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut state = EmitterState::default();
    // set prefix for using generated code inside `codegen` module (see main.rs).
    state.mod_prefix = "crate::codegen::";
    state.working_dir = out_dir.into();

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&schema).expect("codegen");
}
```

- Now, you can modify `src/main.rs` to make use of the generated code:

```rust
#[macro_use] extern crate serde;

use reqwest::Client;

mod codegen {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use self::codegen::client::Sendable;
use self::codegen::pet::Pet;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let _pets = Pet::<()>::list_pets().send(&client).await.unwrap();
}
```

Some things to note:

- The names of associated functions for each [operation](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operationObject) (such as `list_pets`) is obtained from `operationId` fields. But since it's optional and if the user has ignored it in their spec, then we use HTTP methods and number them if there are more than one.
- The emitter tries to bind each operation to some model (based on `body` parameters and `2xx` responses). If it cannot bind it, then they're ignored (at this point).
