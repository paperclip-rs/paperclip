# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.6] - 2025-06-18
### Added
- Enable actix "tail match" using vendor extension. [PR#555](https://github.com/paperclip-rs/paperclip/pull/555)

## [0.9.5] - 2025-03-10
### Added
- Support for camino. [PR#551](https://github.com/paperclip-rs/paperclip/pull/551)
- Support jiff 0.2. [PR#549](https://github.com/paperclip-rs/paperclip/pull/549)

## [0.9.4] - 2024-10-23
### Fixed
- Various cli-ng fixes. [PR#547](https://github.com/paperclip-rs/paperclip/pull/547)

## [0.9.3] - 2024-10-21
### Added
- Experimental openapiv3 cli codegen. [PR#506](https://github.com/paperclip-rs/paperclip/pull/506)

## [0.9.2] - 2024-10-13
### Fixed
- Switch to pro-macro-error2. [PR#545](https://github.com/paperclip-rs/paperclip/pull/545)
- Bumps paperclip-macros to 0.7.0 to fix breaking change in 0.6.4. [PR#546](https://github.com/paperclip-rs/paperclip/pull/546)

## [0.9.1] - 2024-09-10 [ YANKED ]
### Fixed
- Support array field type. [PR#531](https://github.com/paperclip-rs/paperclip/pull/531)

## [0.9.0] - 2024-09-07 [ YANKED ]
### Fixed
- Support latest openapiv3 and indexmap. [PR#507](https://github.com/paperclip-rs/paperclip/pull/507)
- Clippy useless vec lint. [PR#511](https://github.com/paperclip-rs/paperclip/pull/511)
- Add lint ignores to paperclip unit struct. [PR#514](https://github.com/paperclip-rs/paperclip/pull/514)
- Fix map conversion for openapiv3. [PR#529](https://github.com/paperclip-rs/paperclip/pull/529)

# Added
- Add TermsOfService to the openapi spec. [PR#522](https://github.com/paperclip-rs/paperclip/pull/522)
- Add max/min for integers. [PR#523](https://github.com/paperclip-rs/paperclip/pull/523)
- Parse max/min attributes on fields. [PR#524](https://github.com/paperclip-rs/paperclip/pull/524)
- Add support for jiff via feature flag. [PR#526](https://github.com/paperclip-rs/paperclip/pull/526)
- Support generic array size. [PR#527](https://github.com/paperclip-rs/paperclip/pull/527)

## [0.8.2] - 2023-09-27
### Fixed
- Pin openapiv3 to wa breaking change. [PR#508](https://github.com/paperclip-rs/paperclip/pull/508)

## [0.8.1] - 2023-08-20
### Added
- Add support for `PathBuf` type. [PR#502](https://github.com/paperclip-rs/paperclip/pull/502)
- Add support for `actix-identity` type. [PR#495](https://github.com/paperclip-rs/paperclip/pull/495)
- Add support for `head` method type. [PR#493](https://github.com/paperclip-rs/paperclip/pull/493)

### Fixed
- Correct parameter name ordering. [PR#504](https://github.com/paperclip-rs/paperclip/pull/504)
- No spec path for `json_spec_v3` and `swagger-ui` combination. [PR#498](https://github.com/paperclip-rs/paperclip/pull/498)

## [0.8.0] - 2023-01-14 :warning: Breaking Changes
### Added
- Support non-boxed bodies in scope middleware. [PR#457](https://github.com/paperclip-rs/paperclip/pull/457)
- Add `uuid0` and `uuid1` features. [PR#461](https://github.com/paperclip-rs/paperclip/pull/461)
- Add Content-Type Header to Swagger-UI Requests. [PR#467](https://github.com/paperclip-rs/paperclip/pull/467)

### Changed
- Updated copyrights to use "Paperclip Contributors". [PR#470](https://github.com/paperclip-rs/paperclip/pull/470)
- Switch from `parking_lot` to `std::sync`. [PR#473](https://github.com/paperclip-rs/paperclip/pull/473)
- Replaced dependency `pin-project` with `pin-project-lite`. [PR#472](https://github.com/paperclip-rs/paperclip/pull/472)

### Fixed
- Ensures that each chunk is written fully (code-gen). [PR#491](https://github.com/paperclip-rs/paperclip/pull/491)
- Strip template pattern from paths. [PR#486](https://github.com/paperclip-rs/paperclip/pull/486)
- Inconsistent behavior between `rapidoc` and `swagger_ui` (extra slash). [PR#460](https://github.com/paperclip-rs/paperclip/pull/460)
- Fixed header-based `SecuritySchema` conversion for `OpenAPI v3`. [PR#458](https://github.com/paperclip-rs/paperclip/pull/458)
- Respect host setting of v2 spec when converting to v3. [PR#463](https://github.com/paperclip-rs/paperclip/pull/463)
- Move root level extensions to root. [PR#464](https://github.com/paperclip-rs/paperclip/pull/464)
- `Apiv2Header` link in documentation. [PR#468](https://github.com/paperclip-rs/paperclip/pull/468)

## [0.7.1] - 2022-07-27
### Added
- Add support for `PATCH` methods. [PR#422](https://github.com/paperclip-rs/paperclip/pull/422)
- Add support for header parameters through the newly introduced `Apiv2Header` derive macro. [PR#413](https://github.com/paperclip-rs/paperclip/pull/413)
- Add support for [RapiDoc UI](https://mrin9.github.io/RapiDoc/index.html). [PR#420](https://github.com/paperclip-rs/paperclip/pull/420)
- Add example support for derived `Apiv2Schema`. [PR#421](https://github.com/paperclip-rs/paperclip/pull/421)
- Add ability to not generate documentation for some operations through the skip attribute on api_v2_schema macro. [PR#423](https://github.com/paperclip-rs/paperclip/pull/423)
- Add support for deprecated operations. [PR#424](https://github.com/paperclip-rs/paperclip/pull/424)

### Fixed
- Fix missing slash between url parts [PR#416](https://github.com/paperclip-rs/paperclip/pull/416)
- Properly support non-BoxBody response payload types [PR#414](https://github.com/paperclip-rs/paperclip/pull/414)
- Fix required fields definition when using serde flatten [PR#412](https://github.com/paperclip-rs/paperclip/pull/412)
- Fix reference urls not being RFC3986-compliant [PR#411](https://github.com/paperclip-rs/paperclip/pull/411)

## [0.7.0] - 2022-04-03
### Added
- Add openapi component rename attribute [PR#367](https://github.com/paperclip-rs/paperclip/pull/367)
- Allow automatically adding the module path to the openapi component name, via a feature "path-in-definition" [PR#373](https://github.com/paperclip-rs/paperclip/pull/373)
- Add missing ip, ipv4 and ipv6 string format types
- Add support for actix-web 4
  - Middleware support does not support non-`BoxBody` response payload types.
    As a workaround you can use `actix-web::middlware::Compat`.
- Add support for Schemas wrapping Generic types (e.g. `DataResponse<T>` where `T` also derives
`Apiv2Schema`) [PR#332](https://github.com/paperclip-rs/paperclip/pull/332)
- Add support for actix-web validator [PR#403](https://github.com/paperclip-rs/paperclip/pull/403)

### Fixed
- Add more tuple sizes for web::Path for OperationModifier impl [PR#379](https://github.com/paperclip-rs/paperclip/pull/379)
- Add missing extensions to openapi v2 Info
- Schemas that enclose Generics are no longer conflicting/overwritten

## [0.6.1] - 2021-10-15
### Fixed
- Actix2 plugin: fix compilation error `ReqData` not found

## [0.6.0] - 2021-10-13
### Added
- Add support for actix-web-macros methods routing [PR#289](https://github.com/paperclip-rs/paperclip/pull/289)
- Actix plugin: add an empty impl for actix-web `ReqData<T>`
- Add support for the `#[serde(skip)]` attribute in structs and enums.
- Expose openapi v3 spec through `with_json_spec_v3_at` and `with_raw_json_spec_v3` - this is done through a conversion from
 the v2 types to v3 and so all existing code should be valid. It also means that we're not yet exposing any specific
 v3 features.
- Added new method `trim_base_path` to trim the api base path from all method paths.
- `Apiv2Schema` supports `url` [PR#334](https://github.com/paperclip-rs/paperclip/pull/334)
- Add [swagger-ui](https://swagger.io/tools/swagger-ui/) for visualization/test of API via `with_swagger_ui_at` [PR#331](https://github.com/paperclip-rs/paperclip/pull/331).

### Changed
- Actix plugin: `#[api_v2_errors]` macro now supports adding different error schemes per response code.
- Actix plugin: Add new `#[api_v2_errors_overlay]` macro which can be used to filter out unwanted responses from an existing error type.

### Fixed
- Optional type aliases like `type Email = Option<String>` will not be added to the `required` fields.
- Actix plugin: Path tuples now inherit field names and descriptions from doc comments

## [0.5.0] - 2020-11-28
### Added
- Actix plugin: Support for actix-web 3.0 (is now default).
- Arrays up to length 32 are supported (in codegen and actix-web plugin).
- Actix plugin: `#[api_v2_operation]` macro now supports specifying `consumes`, `produces`, `summary`, `description`, `tags`
and `operation_id` in macro.
- Actix plugin: Support for `actix-session`, `serde_qs` and `chrono` types in handlers.
- Actix plugin: `App::wrap_api_with_spec` allows to provide default specification with `info` and other custom settings
- Actix plugin: Support tags in api_v2_operation macros.
- Actix plugin: `#[serde(flatten)]` support in `Apiv2Schema` derive.
- Actix plugin: Added wrapper types for some 2xx status codes.

### Changed
- Actix plugin: Refactored internals of `#[api_v2_operation]` proc macro (long-outstanding technical debt). This now generates operation metadata (on the fly) for each handler, which enables us to tie custom changes to operations easily.
- Actix plugin: Grouping of parameters across handlers have been disabled as a result of major bugs (it's now under `normalize` feature).
- Actix plugin: actix-web `2.x` is supported through `actix2` and `actix2-nightly` features.

### Fixed
- `Apiv2Schema` supports `HashMap<Uuid, Foo>`.
- Actix plugin: `#[api_v2_operation]` supports referencing inside handlers.
- Actix plugin: Fixed a bug on using `where` clause in handlers marked with `#[api_v2_operation]`.
- Actix plugin: API objects have `object` type specified in their schemas.

## [0.4.1] - 2020-07-01
### Fixed
- Re-exported import from `proc-macro-error` which broke builds.

## [0.4.0] - 2020-06-13
### Added
- Client timeout in CLI.
- Codegen for header parameters in operations.
- Validation for non-body parameters.
- `x-rust-dependencies` field in root for specifying additional crate dependencies.
- Default en/decoders (JSON/YAML) and support for custom en/decoders through `x-rust-coders` field in root.
- Methods to `Schema` trait for aiding resolution and codegen.
- Codegen for nested arrays in operation parameters.
- Codegen for form data parameters in operations.
- Codegen for `Any` type in schema.
- CLI payload encoding/decoding supports custom types and not limited to JSON.
- Codegen for `#[deprecated]` attribute when `deprecated` field is set to `true` in schema.
- Codegen adds `Request` and `Response` traits for HTTP request and response objects.
- CLI uses name and version from `info` field in spec.
- Codegen for file responses with streaming to `AsyncWrite` implementors.
- Codegen for `multipart/form-data` parameters with file streaming.
- Referencing globally defined parameters and responses.
- Codegen for enums in object definitions.
- Response wrapper containing headers and status code for operation.
- Parsing for custom response headers.
- Actix plugin: `#[openapi(empty)]` attribute can be used to any type to implement empty schema and ignore the warning.
- Actix plugin: Empty impls for some actix-web types (like `Payload`, `Data<T>`, etc.).
- Actix plugin: Raw JSON spec generation from handlers.
- Actix plugin: Support for `#[serde(rename = "...")]` and `#[serde(rename_all = "...")]`.
- Actix plugin: Support for error (non-2xx) response codes.
- Actix plugin: Type-level and field-level documentation is now used for `description` fields in schema and properties.
- Actix plugin: Security definitions (globally) and security requirements (for operations).
- Actix plugin: Support for `serde_json::Value` and `serde_yaml::Value` (by default), and for objects from `uuid`, `rust_decimal`, `chrono`, etc. (through features of the same name) in structs.

### Changed
- Switched to templating for (almost) static modules.
- Operation IDs are preferred for method names if they exist.
- `ApiClient` uses the newly added `Request` and `Response` and is now async/await.
- `Sendable` accepts the new `ApiClient` and is now async/await.
- `SchemaRepr` renamed to `Resolvable`.
- `OperationMap` renamed to `PathItem`.
- `Api` struct is now generic over parameters and responses in addition to definitions.
- Actix plugin: Switched to `actix-web = "^2.0"`.
- Actix plugin: **Supports stable compiler**.
- Actix plugin: `#[api_v2_schema]` macro attribute is now `#[derive(Apiv2Schema)]`.

### Fixed
- Switched `enum` field to array of `any` rather than strings.
- Resolution of anonymous schema definitions in objects, operation parameters and responses.
- Unmappable operations (i.e., without body parameters and simple response types) are now namespaced in a separate module.
- Array definitions are now allowed in schemas.
- `additionalProperties` takes boolean or a schema.
- Deadlock when resolving some recursive types.
- Actix plugin: `.route()` method call on `App`, `Scope` and `ServiceConfig` don't override existing route operations.
- Actix plugin: `web::Path<T>` also supports simple types (strings, integers, etc.).
- Actix plugin: `#[api_v2_schema]` derivatives can now use references.
- Actix plugin: Breakage of `#[api_v2_operation]` when returning `impl Handler`.
- Actix plugin: `web::scope` supports having path parameters.
- Actix plugin: Misuse of `actix_web::Scope` in `App::configure` which resulted in missing overwritten routes.

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

[Unreleased]: https://github.com/paperclip-rs/paperclip/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/paperclip-rs/paperclip/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/paperclip-rs/paperclip/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/paperclip-rs/paperclip/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/paperclip-rs/paperclip/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/paperclip-rs/paperclip/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/paperclip-rs/paperclip/releases/tag/v0.1.0
