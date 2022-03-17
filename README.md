# Paperclip

![Build Status](https://github.com/paperclip-rs/paperclip/actions/workflows/cicd.yml/badge.svg)
![Linter Status](https://github.com/paperclip-rs/paperclip/actions/workflows/linter.yml/badge.svg)
[![Usage docs](https://img.shields.io/badge/quickstart-blue.svg)](https://paperclip-rs.github.io/paperclip)
[![API docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://paperclip-rs.github.io/paperclip/paperclip)
[![Crates.io](https://img.shields.io/crates/v/paperclip.svg)](https://crates.io/crates/paperclip)

Paperclip offers tooling for the [OpenAPI specification](https://github.com/OAI/OpenAPI-Specification/). Once complete, it will provide:

- Code generation for efficient, type-safe, compile-time checked HTTP APIs (server, client and CLI) in Rust.
- Support for processing, validating and hosting OpenAPI spec.
- Customization for spec and code generation.

It's currently under active development and may not be ready for production use just yet.

You may be interested in:

 - [Examples and Usage](https://paperclip-rs.github.io/paperclip).
 - [Features being worked on](https://github.com/paperclip-rs/paperclip/projects).
 - [API documentation](https://paperclip-rs.github.io/paperclip/paperclip).

## Developing locally

 - Make sure you have [`rustup`](https://rustup.rs/) installed. `cd` into this repository and run `make prepare` to setup your environment.
 - Now run `make` to build and run the tests.

## Contributing

This project welcomes all kinds of contributions. No contribution is too small!

If you want to contribute to this project but don't know how to begin or if you need help with something related to this project, feel free to send me an email (in Github profile) or join the [Discord server](https://discord.gg/PPu4Dhj).

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Sponsors

Folks who have sponsored for the development of this project:

<table>
  <tr>
    <td><a href="https://offscale.io"><img src="https://avatars1.githubusercontent.com/u/11748352" width="100"></a>
  </tr>
</table>

## FAQ

> Why is this generating raw Rust code instead of leveraging [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html) for compile-time codegen?

I don't think proc macros are the right way to go for REST APIs. We need to be able to **see** the generated code somehow to identify names, fields, supported methods, etc. With proc macros, you sorta have to guess.

This doesn't mean you can't generate APIs in compile-time. The only difference is that you'll be using [build scripts](https://paperclip-rs.github.io/paperclip/build-script.html) instead and `include!` the relevant code. That said, [we're using proc-macros](./macros) for other things.

> The error thrown at compile-time doesn't look like it's very useful. Isn't there a better way to do this?

None that I can think of, sadly.

**New ideas are here needed.**
