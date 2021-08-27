## Adding additional metadata to operations

By default, the first doc comment (if any) is taken for the `summary` field and the rest of the following doc comments
(if any) will be taken as `description` for that operation.

```rust
/// Default
/// multiline
/// summary
///
/// Default
/// multiline
/// description
async fn my_handler() -> Json<Foo> { /* */ }
```

This can be overridden by explicitly specifying `summary` and `description` in the proc-macro attribute like so:

```rust
#[api_v2_operation(
  summary = "My awesome handler",
  description = "It creates a pretty JSON object",
  /// A few other parameters are also supported
  operation_id = "my_handler",
  consumes = "application/yaml, application/json",
  produces = "application/yaml, application/json",
  tags(Cats, Dogs, "Api reference"),
)]
async fn my_handler() -> Json<Foo> { /* */ }
```
