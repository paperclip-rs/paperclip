## Defining request headers for your API

You can use `Apiv2Headers` derive macro for structs which can then be used as handler parameters to have those headers added in the operation parameters.

```rust
use paperclip::actix::Apiv2Headers;

/// Headers for Foo
#[derive(Apiv2Header, Deserialize)]
struct FooHeaders {
    /// Id of foo
    foo_id: uuid::Uuid,
    /// Count of foo
    foo_count: i32,
    #[openapi(skip)]
    other_cruft: XXXX
}

/// Headers for Doo
#[derive(Apiv2Header, Deserialize)]
struct DooHeaders {
    /// id of doo
    doo_id: uuid::Uuid
}

/// Headers for Doo
#[derive(Apiv2Header, Deserialize)]
struct BooHeaders(#[openapi(name="x-boo", description="id of boo")] uuid::Uuid);

#[api_v2_operation]
async fn handler(_: FooHeaders, _: DooHeaders, _: BooHeaders, body: web::Json<X>) -> web::Json<X> {
    unimplemented!()
}
```

Supported `#[openapi]` parameters are 
- skip: Allow to ignore a parameter from the struct which derive `Apiv2Headers`
- name: Allow to override the header name. By default the header will have the name of the struct field, This parameter is required for new type or braced struct fields.
- description: Allow to set a description for the header.
- format: Allow to specify the format. Must be a supported openapi v2 data type format.
