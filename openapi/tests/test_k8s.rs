#[macro_use]
extern crate paperclip_macros;
#[macro_use]
extern crate serde_derive;

use log::LevelFilter;
use paperclip_openapi::v2::{
    self,
    codegen::{Config, DefaultEmitter, SchemaEmitter},
    models::{Api, Version},
};

use std::fs::{self, File};

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
fn test_heavy_load_and_resolve_definitions() {
    // env_logger::builder()
    //     .filter(Some("paperclip_openapi"), LevelFilter::Trace)
    //     .init();

    let root = env!("CARGO_MANIFEST_DIR");
    let fd = File::open(String::from(root) + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json")
        .expect("file?");
    let raw: Api<K8sSchema> = v2::from_reader(fd).expect("deserializing spec");
    let resolved = raw.resolve().expect("resolution");
    assert_eq!(resolved.swagger, Version::V2);
    assert_eq!(resolved.definitions.len(), 614);

    let json_props_def = &resolved.definitions
        ["io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps"];
    let desc = json_props_def.borrow().description.clone();
    let all_of = json_props_def.borrow().properties.as_ref().unwrap()["allOf"].clone();
    let items = all_of.borrow().items.as_ref().unwrap().clone();
    assert_eq!(items.borrow().description, desc); // both point to same `JSONSchemaProps`

    let mut config = Config::default();
    config.working_dir = root.into();
    config.working_dir.push("target");
    if config.working_dir.exists() {
        fs::remove_dir_all(&config.working_dir).expect("cleaning up dir");
    }

    let emitter = DefaultEmitter::from(config);
    emitter
        .create_defs(&resolved)
        .expect("creating definitions");
}
