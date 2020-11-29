## Defining security schemes for your API

You can use `Apiv2Security` derive macro for structs which can then be used as handler parameters to have those handlers marked as requiring authorization.

```rust
use paperclip::actix::Apiv2Security;

#[derive(Apiv2Security)]
#[openapi(
  apiKey,
  in = "header",
  name = "Authorization",
  description = "Use format 'Bearer TOKEN'"
)]
pub struct AccessToken;

impl FromRequest for Accesstoken { /*...*/ }

#[api_v2_operation]
async fn my_handler(access_token: AccessToken) -> Result<String, MyError> {
    /*...*/
}
```

First parameter is the type of security, currently supported types are "apiKey" and "oauth2". Possible parameters are `alias`, `description`, `name`, `in`, `flow`, `auth_url`, `token_url` or `parent`.

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
