#[macro_use]
extern crate paperclip_derive;
#[macro_use]
extern crate serde_derive;

use paperclip_openapi::v2::{
    self,
    models::{Api, Version},
};

use std::fs::File;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum PatchStrategy {
    Merge,
    RetainKeys,
    #[serde(rename = "merge,retainKeys")]
    MergeAndRetain,
}

#[api_schema]
#[allow(dead_code)]
#[derive(Deserialize)]
struct K8sSchema {
    #[serde(rename = "x-kubernetes-patch-strategy")]
    patch_strategy: Option<PatchStrategy>,
}

#[test]
fn test_heavy_load_and_resolve_definitions() {
    let root = String::from(env!("CARGO_MANIFEST_DIR"));
    let fd = File::open(root + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("file?");
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
}
