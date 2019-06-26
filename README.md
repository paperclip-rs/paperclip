# Paperclip

[![Build Status](https://api.travis-ci.org/wafflespeanut/paperclip.svg?branch=master)](https://travis-ci.org/wafflespeanut/paperclip)
[![API docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://paperclip.waffles.space/paperclip)
[![Crates.io](https://img.shields.io/crates/v/paperclip.svg)](https://crates.io/crates/paperclip)

Paperclip is a WIP OpenAPI code generator for efficient type-safe compile-time checked HTTP APIs in Rust.

It's currently under active development and may not be ready for production use just yet.

You may be interested in:

 - [An overview](https://paperclip.waffles.space/)
 - [Supported features](https://paperclip.waffles.space/features.html).
 - [Features being worked on](https://github.com/wafflespeanut/paperclip/projects).
 - [Examples](https://paperclip.waffles.space/examples.html).
 - [API documentation](https://paperclip.waffles.space/paperclip).

## Developing locally

 - Make sure you have [`rustup`](https://rustup.rs/) installed. `cd` into this repository and run `make prepare` to setup your environment.
 - Now run `make` to build and run the tests.

## Contributing

This project welcomes all kinds of contributions. No contribution is too small!

If you want to contribute to this project but don't know how to begin or if you need help with something related to this project, feel free to send me an email or ping me in Discord (same handle).

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

This doesn't mean you can't generate APIs in compile-time. The only difference is that you'll be using [build scripts](#build-script-example) instead and `include!` the relevant code. That said, [we're using proc-macros](./macros) for other things.

> The error thrown at compile-time doesn't look like it's very useful. Isn't there a better way to do this?

None that I can think of, sadly.

**New ideas here needed.**
