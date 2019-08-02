# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Actix plugin: Callers of `#[api_v2_schema]` can specify `empty` to implement empty schema for any type and ignore the warning.
- Empty impls for some actix-web types (like `Payload`, `Data<T>`, etc.).

## [0.3.0] - 2019-07-30
### Added
- Paperclip's `#[api_v2_schema]` derives raw schema structs (i.e., no smart pointers) along with actual schema structs for other users and plugins. An effect of this would be that now there's a `DefaultSchemaRaw` in addition to `DefaultSchema`.
- `TypedData`, `Apiv2Schema` and `Apiv2Operation` traits for deriving v2 spec for server plugins.
- `paperclip-core` crate for segregating core types and traits.
- `paperclip-actix` crate as an actix-web plugin for hosting v2 spec as a JSON by marking models and operations using proc macro attributes.
- `paperclip::actix` module for exporting `paperclip_actix::*` when `actix` feature is enabled.

### Changed
- Segregated dependencies using feature gates.
- `Api::<S>::resolve` now returns `ValidationError` instead of `failure::Error`.
- During serialization, optional schema fields are skipped when empty/null.

### Fixed
- Allowing ports in `host` field in v2 spec.

## [0.2.0] - 2019-07-03
### Added
- Gitbook for detailed documentation and walkthroughs.
- Changelog
- Root module (`mod.rs`, `lib.rs` or `main.rs`) generation for codegen (previously we were only generating children modules).
- Cargo manifest generation (gated by `"cli"` feature).
- `[bin]` target (CLI) for generating crates.
- CLI generation (fancy curl for your APIs) - generated app uses async/await and `runtime_tokio`.
- `ApiClient::make_request` for sending a request and fetching a response future.
- Support for operations returning array of objects.
- Codegen uses `basePath` and `host` fields (if they exist) to override default base URL (`https://example.com`).
- API relative paths are checked for uniqueness.

### Changed
- Codegen now writes the dependency traits, types and impls in the root module.

### Removed
- `ApiClient::base_url` in favor of `ApiClient::make_request`
- Redundant `Optional` trait for generated markers.

### Fixed
- Templated paths are now validated against parameters.
- Import prefixes support in emitter (for codegen).

## [0.1.0] - 2019-06-13
### Added
- Build script example in README.
- `impl Sendable` for fulfilled builders to send API requests and return response futures.
- Documentation for builders and methods from `description` fields.
- Generation of `#[repr(transparent)]` builders with phantom fields for enforcing required parameters.
- Generation of builders for API objects and operations.
- Generation of Rust structs with appropriate serde rules from definitions.
- `#[api_schema]` proc-macro attribute for implementing `Schema` trait for custom schemas.
- Resolution of `$ref` references in definitions and paths to objects in the same file.
- Loading OpenAPI v2 schema from JSON/YAML
- Workspace, README, LICENSE, Makefile, CI config, etc.

[Unreleased]: https://github.com/wafflespeanut/paperclip/compare/v0.3.0...HEAD
[0.1.0]: https://github.com/wafflespeanut/paperclip/releases/tag/v0.1.0
[0.2.0]: https://github.com/wafflespeanut/paperclip/releases/tag/v0.2.0
[0.3.0]: https://github.com/wafflespeanut/paperclip/releases/tag/v0.3.0
