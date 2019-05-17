use paperclip_openapi::v2::{self, models::Version};

use std::fs::File;

#[test]
fn test_heavy_load_and_resolve_definitions() {
    let root = String::from(env!("CARGO_MANIFEST_DIR"));
    let fd = File::open(root + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("file?");
    let raw = v2::from_reader(fd).expect("deserializing spec");
    let resolved = raw.resolve().expect("resolution");
    assert_eq!(resolved.swagger, Version::V2);
    assert_eq!(resolved.definitions.len(), 614);
}
