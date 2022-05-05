[![API docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://paperclip-rs.github.io/paperclip/paperclip)
[![Crates.io](https://img.shields.io/crates/v/paperclip.svg)](https://crates.io/crates/paperclip)

# Paperclip

Paperclip is a OpenAPI code generator for efficient type-safe compile-time checked HTTP APIs in Rust.

It's currently under active development and may not be ready for production use just yet.

### Features

- Paperclip CLI can generate API client library (which checks some usage at [compile-time](compile-checks.md)) or a console for your API (which checks usage at [runtime](cli.md#runtime-checks)).
- API client code can also be generated using [build scripts](build-script.md) which will then check parameters usage in your library at compile time.
- [Acix-web plugin](actix-plugin.md) can be used to host the API spec for your `actix-web` application.
  - Actix Web 4 support has been added through actix4 feature flag.

### Design

TODO
