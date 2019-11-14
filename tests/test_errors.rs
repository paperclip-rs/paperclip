use paperclip::v2::{
    self,
    codegen::{DefaultEmitter, Emitter, EmitterState},
    models::{DefaultSchema, ResolvableApi},
};

use std::io::Cursor;

#[test]
fn test_templated_path_missing_parameters() {
    let spec = Cursor::new(
        b"
swagger: \"2.0\"
definitions: {}
info:
  title:  \"Petstore\"
  version: \"1.0.0\"
paths:
  /pets/{petId}:
    get:
      responses:
        \"200\":
          schema:
            type: string
" as &[_],
    );

    let raw: ResolvableApi<DefaultSchema> = v2::from_reader(spec).expect("deserializing spec");
    let resolved = raw.resolve().expect("resolution");

    let state = EmitterState::default();
    let emitter = DefaultEmitter::from(state);
    let err = emitter.generate(&resolved).unwrap_err().to_string();
    assert_eq!(
        err,
        "Parameter(s) {\"petId\"} aren't defined for templated path \"/pets/{petId}\"",
    );
}

#[test]
fn test_templated_path_uniqueness() {
    let spec = Cursor::new(
        b"
swagger: \"2.0\"
definitions: {}
info:
  title:  \"Petstore\"
  version: \"1.0.0\"
paths:
  /store/{id1}/pets/{id2}:
    parameters:
    - name: id1
      in: path
      type: integer
      required: true
    - name: id2
      in: path
      type: integer
      required: true
    get:
      responses:
        \"200\":
          schema:
            type: string
  /store/{storeId}/pets/{petId}:
    get:
      responses:
        \"200\":
          schema:
            type: string
" as &[_],
    );

    let raw: ResolvableApi<DefaultSchema> = v2::from_reader(spec).expect("deserializing spec");
    let resolved = raw.resolve().expect("resolution");

    let state = EmitterState::default();
    let emitter = DefaultEmitter::from(state);
    let err = emitter.generate(&resolved).unwrap_err().to_string();
    assert_eq!(
        err,
        "Path similar to \"/store/{storeId}/pets/{petId}\" already exists.",
    );
}
