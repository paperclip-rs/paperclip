## Defining security schemes for your API

You can use `Apiv2Security` derive macro for structs which can then be used as handler parameters to have those handlers marked as requiring authorization.

If you need to define a Bearer Authentication :
```rust
use paperclip::actix::Apiv2Security;

#[derive(Apiv2Security)]
#[openapi(
  http,
  scheme = "bearer",
  bearer_format = "JWT",
  description = "Use JWT Bearer token for authentification"
)]
pub struct AccessToken;

impl FromRequest for Accesstoken { /*...*/ }

#[api_v2_operation]
async fn my_handler(access_token: AccessToken) -> Result<String, MyError> {
    /*...*/
}
```

If you need to define an API key Authentication :
```rust
use paperclip::actix::Apiv2Security;

#[derive(Apiv2Security)]
#[openapi(
  apiKey,
  in = "header",
  name = "x-api-key",
  description = "Use API Key"
)]
pub struct AccessToken;

impl FromRequest for Accesstoken { /*...*/ }

#[api_v2_operation]
async fn my_handler(access_token: AccessToken) -> Result<String, MyError> {
    /*...*/
}
```

First parameter is the type of security, currently supported types are "http", "apiKey" and "oauth2". Possible parameters are `alias`, `description`, `name`, `in`, `bearer_format`, `scheme` `flow`, `auth_url`, `token_url` or `parent`.

Use `alias` parameter if you need to have two different security definitions of the same type.

If you need to define scopes for `oauth2`, use `parent` attribute:

```rust
#[derive(Apiv2Security, Deserialize)]
#[openapi(
  oauth2,
  auth_url = "http://example.com/",
  token_url = "http://example.com/token",
  flow = "password"
)]
struct OAuth2Access;

#[derive(Apiv2Security, Deserialize)]
#[openapi(parent = "OAuth2Access", scopes("pets.read", "pets.write"))]
struct PetScopeAccess;
```
