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

## Motivation

While [Serde](https://serde.rs/) makes it amazingly easy to write API objects, only the official codegen [supports generating proper APIs](https://github.com/swagger-api/swagger-codegen/tree/dedb5ce36d54495365da9a7d88d1e6e056cfe29f/samples/client/petstore/rust) and leverages the builder pattern for building API requests. I think it should be really easy to build type-safe APIs from OpenAPI specifications using pure Rust.

## Building

 - Make sure you have [`rustup`](https://rustup.rs/) installed. `cd` into this repository and run `make prepare` to setup your environment.
 - Now run `make` to build and run the tests.

## Contributing

This project welcomes all kinds of contributions. No contribution is too small!

If you really wish to contribute to this project but don't know how to begin or if you need help with something related to this project, then feel free to send me an email or ping me in Discord (same handle).

### Code of Conduct

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
