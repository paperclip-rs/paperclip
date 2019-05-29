#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate paperclip_macros;
#[macro_use]
extern crate serde_derive;

use paperclip_openapi::v2::{
    self,
    codegen::{DefaultEmitter, EmitterState, SchemaEmitter},
    models::{Api, HttpMethod, Version},
};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref SCHEMA: Api<K8sSchema> = {
        let root = env!("CARGO_MANIFEST_DIR");
        let fd = File::open(String::from(root) + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json")
            .expect("file?");
        let raw: Api<K8sSchema> = v2::from_reader(fd).expect("deserializing spec");
        raw.resolve().expect("resolution")
    };
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum PatchStrategy {
    Merge,
    RetainKeys,
    #[serde(rename = "merge,retainKeys")]
    MergeAndRetain,
}

#[api_v2_schema]
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct K8sSchema {
    #[serde(rename = "x-kubernetes-patch-strategy")]
    patch_strategy: Option<PatchStrategy>,
}

#[test]
fn test_definition_ref_cycles() {
    assert_eq!(SCHEMA.swagger, Version::V2);
    assert_eq!(SCHEMA.definitions.len(), 614);

    let json_props_def = &SCHEMA.definitions
        ["io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps"];
    let desc = json_props_def.read().description.clone();
    let all_of = json_props_def.read().properties.as_ref().unwrap()["allOf"].clone();
    let items = all_of.read().items.as_ref().unwrap().clone();
    assert_eq!(items.read().description, desc); // both point to same `JSONSchemaProps`
}

#[test]
fn test_path_with_schema() {
    let api_versions = &SCHEMA.paths["/api/"].methods[&HttpMethod::Get].responses["200"].schema;
    let schema = api_versions.as_ref().expect("bleh?").read();
    assert!(schema.reference.is_none()); // this was a reference
    assert_eq!(
        &SCHEMA.definitions["io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions"]
            .read()
            .description
            .as_ref()
            .unwrap()
            .as_str(),
        schema.description.as_ref().unwrap()
    );
}

#[test]
fn test_emitter() {
    // env_logger::builder()
    //     .filter(Some("paperclip_openapi"), log::LevelFilter::Trace)
    //     .init();

    let root = env!("CARGO_MANIFEST_DIR");
    let mut state = EmitterState::default();
    state.working_dir = root.into();
    state.working_dir.push("tests");
    state.working_dir.push("test_k8s");

    let some_schema_path = state
        .working_dir
        .join("io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/json_schema_props.rs");

    let emitter = DefaultEmitter::from(state);
    emitter.create_defs(&SCHEMA).expect("creating definitions");

    let mut contents = String::new();
    let mut fd = File::open(&some_schema_path).expect("missing mod");
    fd.read_to_string(&mut contents).expect("reading mod");

    // We're interested in this definition because:
    // - It uses some Rust keywords.
    // - It has a number of camelcase fields.
    // - It has some fields which are maps.
    // - It uses pretty much all types (including custom types).
    // - It references other definitions (directly and through an array).
    // - It's a cyclic type.
    assert_eq!(contents, "
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonSchemaProps {
    #[serde(rename = \"$ref\")]
    pub ref_: Option<String>,
    #[serde(rename = \"$schema\")]
    pub schema: Option<String>,
    #[serde(rename = \"additionalItems\")]
    pub additional_items: Option<String>,
    #[serde(rename = \"additionalProperties\")]
    pub additional_properties: Option<String>,
    #[serde(rename = \"allOf\")]
    pub all_of: Option<Vec<crate::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    #[serde(rename = \"anyOf\")]
    pub any_of: Option<Vec<crate::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub default: Option<String>,
    pub definitions: Option<std::collections::BTreeMap<String, String>>,
    pub dependencies: Option<std::collections::BTreeMap<String, String>>,
    pub description: Option<String>,
    #[serde(rename = \"enum\")]
    pub enum_: Option<Vec<String>>,
    pub example: Option<String>,
    #[serde(rename = \"exclusiveMaximum\")]
    pub exclusive_maximum: Option<bool>,
    #[serde(rename = \"exclusiveMinimum\")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = \"externalDocs\")]
    pub external_docs: Option<crate::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::external_documentation::ExternalDocumentation>,
    pub format: Option<String>,
    pub id: Option<String>,
    pub items: Option<String>,
    #[serde(rename = \"maxItems\")]
    pub max_items: Option<i64>,
    #[serde(rename = \"maxLength\")]
    pub max_length: Option<i64>,
    #[serde(rename = \"maxProperties\")]
    pub max_properties: Option<i64>,
    pub maximum: Option<f64>,
    #[serde(rename = \"minItems\")]
    pub min_items: Option<i64>,
    #[serde(rename = \"minLength\")]
    pub min_length: Option<i64>,
    #[serde(rename = \"minProperties\")]
    pub min_properties: Option<i64>,
    pub minimum: Option<f64>,
    #[serde(rename = \"multipleOf\")]
    pub multiple_of: Option<f64>,
    pub not: Option<Box<crate::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub nullable: Option<bool>,
    #[serde(rename = \"oneOf\")]
    pub one_of: Option<Vec<crate::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub pattern: Option<String>,
    #[serde(rename = \"patternProperties\")]
    pub pattern_properties: Option<std::collections::BTreeMap<String, String>>,
    pub properties: Option<std::collections::BTreeMap<String, String>>,
    pub required: Option<Vec<String>>,
    pub title: Option<String>,
    #[serde(rename = \"type\")]
    pub type_: Option<String>,
    #[serde(rename = \"uniqueItems\")]
    pub unique_items: Option<bool>,
    #[serde(rename = \"x-kubernetes-embedded-resource\")]
    pub x_kubernetes_embedded_resource: Option<bool>,
    #[serde(rename = \"x-kubernetes-int-or-string\")]
    pub x_kubernetes_int_or_string: Option<bool>,
    #[serde(rename = \"x-kubernetes-preserve-unknown-fields\")]
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
}
");
}
