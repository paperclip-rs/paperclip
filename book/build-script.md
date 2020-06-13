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
thiserror = "1.0.19"
futures = "0.1"
parking_lot = "0.8"
reqwest = "0.9"
serde = "1.0"
# Other crates I'm using for this example.
futures-preview = { version = "0.3.0-alpha.16", features = ["compat"], package = "futures-preview" }
runtime = "0.3.0-alpha.7"
runtime-tokio = "0.3.0-alpha.6"

[build-dependencies]
paperclip = { version = "0.4", features = ["v2", "codegen"] }
```

- Add `my-spec.yaml` to the project root with contents from [this file](https://raw.githubusercontent.com/wafflespeanut/paperclip/master/tests/pet-v2.yaml).

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

use thiserror::Error;
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

Some things to note:

- I'm using async/await only to demonstrate the usage of the generated code. The generated client code uses the old [futures 0.1](https://docs.rs/futures/0.1.28/futures/) and won't switch to the new syntax until it's stablilized.
- The names of associated functions for each [operation](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operationObject) (such as `list_pets`) is obtained from `operationId` fields. But since it's optional and if the user has ignored it in their spec, then we use HTTP methods and number them if there are more than one.
- The emitter tries to bind each operation to some model (based on `body` parameters and `2xx` responses). If it cannot bind it, then they're ignored (at this point).

## Compile-time checks?

API calls often *require* some parameters. Should we miss those parameters when performing a request, either the client will produce a runtime error or the server will reject our request. Our generated client code on the other hand, uses markers to avoid this problem at compile-time.

For example, in order to fetch a pet, [`petId` parameter](https://github.com/wafflespeanut/paperclip/blob/fa95b023aaf8b6e396c899a93a9eda6fd791505c/openapi/tests/pet-v2.yaml#L42-L47) is required. Let's change the main function in the above example to fetch a pet without its ID.

```rust
let pet = Pet::get_pet_by_id().send(&client).compat().await?;
```

If we try and compile the program, then we'll get the following error:

```
error[E0599]: no method named `send` found for type
`codegen::pet::PetGetBuilder1<codegen::generics::MissingPetId>`
in the current scope
```

Note that the struct `PetGetBuilder1` has been marked with `MissingPetId`. And, `send` is implemented only when the builder has `PetIdExists` marker.

Hence the fix would be to set the required parameter using the relevant method call (which transforms the builder struct).

```rust
let pet = Pet::get_pet_by_id()
    .pet_id(25)
    .send(&client)
    .compat().await?;
```

... and the code will compile.

The same applies to using API objects (with required fields). For example, the [`addPet` operation](https://github.com/wafflespeanut/paperclip/blob/98a2c053c283ebbbef9b17f7e0ac6ddb0e64f77f/tests/pet-v2.yaml#L125-L148) requires `Pet` object to be present in the HTTP body, but then `Pet` object itself requires [`id` and `name` fields](https://github.com/wafflespeanut/paperclip/blob/98a2c053c283ebbbef9b17f7e0ac6ddb0e64f77f/tests/pet-v2.yaml#L44-L46).

So, if we did this:

```rust
let pet = Pet::add_pet().send(&client).compat().await?;
```

... we'd get an error during compilation:

```
no method named `send` found for type `codegen::pet::PetPostBuilder<
    codegen::generics::MissingId,
    codegen::generics::MissingName
>` in the current scope
```

As we can see, the builder struct has been marked with `MissingId` and `MissingName`, but again `send` is implemented only if the struct had `IdExists` and `NameExists` markers.

Now, we change the code to:

```rust
let pet = Pet::add_pet()
    .id(25)
    .name("Milo")
    .send(&client)
    .compat().await?;
```

... and the code will compile.

> **NOTE:** The types of arguments are also enforced.
