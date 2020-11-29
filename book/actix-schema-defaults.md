## Setting defaults in API schema

It is possible to define default initial values for the schema, which might be useful to define "info",
"schemes", "tags" and other top level schema fields which are not inherited from handlers.

```rust
use paperclip::v2::models::{DefaultApiRaw, Tag};

let mut spec = DefaultApiRaw::default();
spec.tags = vec![
    Tag {
        name: "Dogs".to_string(),
        description: Some("Images of dogs".to_string()),
        external_docs: None,
    },
    Tag {
        name: "Cats".to_string(),
        description: Some("Images of cats".to_string()),
        external_docs: None,
    },
    Tag {
        name: "Api reference".to_string(),
        description: Some("List of all api endpoints".to_string()),
        external_docs: None,
    },
];
spec.info = Info {
    version: "0.1".into(),
    title: "Image server".into(),
    ..Default::default()
};
App::new()
    .wrap_api_with_spec(spec)
    .with_json_spec_at("/api/spec")
    .service(web::resource("/images/pets").route(web::get().to(some_pets_images)))
    .build()
```
