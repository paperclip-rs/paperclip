#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate paperclip_macros;
#[macro_use]
extern crate serde_derive;

use paperclip_openapi::v2::{
    self,
    codegen::{Config, DefaultEmitter, SchemaEmitter},
    models::{Api, Version},
};

use std::fs::{self, File};

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

#[api_schema]
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct K8sSchema {
    #[serde(rename = "x-kubernetes-patch-strategy")]
    patch_strategy: Option<PatchStrategy>,
}

#[test]
fn test_ref_cycles() {
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
fn test_emitter() {
    // env_logger::builder()
    //     .filter(Some("paperclip_openapi"), log::LevelFilter::Trace)
    //     .init();

    let root = env!("CARGO_MANIFEST_DIR");
    let mut config = Config::default();
    config.working_dir = root.into();
    config.working_dir.push("tests");
    config.working_dir.push("test_k8s");

    let gen_dir = config.working_dir.join("io");
    if gen_dir.exists() {
        fs::remove_dir_all(&gen_dir).expect("cleaning up dir");
    }

    let emitter = DefaultEmitter::from(config);
    emitter.create_defs(&SCHEMA).expect("creating definitions");
}
