use paperclip_openapi::v2::{self, models::Version};

use std::fs::File;

#[test]
fn test_load() {
    let root = String::from(env!("CARGO_MANIFEST_DIR"));
    let fd = File::open(root + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("file?");
    let spec = v2::from_reader(fd).expect("deserializing spec");
    assert_eq!(spec.swagger, Version::V2);
    assert_eq!(spec.definitions.len(), 614);
}
