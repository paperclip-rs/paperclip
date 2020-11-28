
Paperclip deduces your API schema using macros (which read the types of handlers and parameter structs at compile time), so in order for paperclip to know what response code sent by the API, it needs type information about it at compile time. It cannot be inferred from response at runtime.

The plugin exports some newtypes which encode this information for the some of the common 2xx codes (`200 OK`, `201 Created` and `202 Accepted`) for JSON responses and `204 No Content`.

```rust
use paperclip::actix::web::Json;
use paperclip::actix::{CreatedJson, AcceptedJson, NoContent};

// 201 Created
#[api_v2_operation]
async fn adopt_pet(body: Json<Pet>) -> Result<CreatedJson<Pet>, ()> {
    let pet: Pet = body.into_inner();
    // bring the pet home
    Ok(CreatedJson(pet))
}
// 204 No Content
#[api_v2_operation]
async fn acknowledge_pet(body: Json<Pet>) -> NoContent {
    NoContent
}
```

### Manually defining error response codes

Another macro `api_v2_errors` helps to manually add error response codes.

```rust
use paperclip::actix::api_v2_errors;

#[api_v2_errors(
    code=400,
    code=401, description="Unauthorized: Can't read session from header",
    code=500,
)]
pub enum MyError {
    /* ... */
}
```

You can now use this error in handlers and they'll emit operations containing those response codes.

```rust
#[api_v2_operation]
async fn my_handler() -> Result<(), MyError> {
    /* ... */
}
```
