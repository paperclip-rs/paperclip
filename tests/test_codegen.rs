use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, EmitMode, Emitter, EmitterState},
    models::{DefaultSchema, ResolvableApi},
};

use once_cell::sync::Lazy;
use std::{fs::File, path::PathBuf};

static ROOT: Lazy<PathBuf> = Lazy::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
static PET_SCHEMA: Lazy<ResolvableApi<DefaultSchema>> = Lazy::new(|| {
    let fd = File::open(ROOT.join("tests/pet-v2.yaml")).expect("file?");
    let raw: ResolvableApi<DefaultSchema> = v2::from_reader(fd).expect("deserializing spec");
    raw.resolve().expect("resolution")
});
static K8S_SCHEMA: Lazy<ResolvableApi<K8sSchema>> = Lazy::new(|| {
    let fd = File::open(ROOT.join("tests/k8s-v1.16.0-alpha.0-openapi-v2.json")).expect("file?");
    let raw: ResolvableApi<K8sSchema> = v2::from_reader(fd).expect("deserializing spec");
    raw.resolve().expect("resolution")
});
static CODEGEN_PET_LIB: Lazy<()> = Lazy::new(|| {
    let mut state = EmitterState::default();
    state.working_dir = ROOT.clone();
    state.working_dir.push("tests/test_pet");
    let mut meta = CrateMeta::default();
    meta.authors = Some(vec!["Me <me@example.com>".into()]);
    meta.mode = EmitMode::Crate;
    state.set_meta(meta);

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&PET_SCHEMA).expect("codegen");
});
static CODEGEN_PET_LIB_NO_ROOT: Lazy<()> = Lazy::new(|| {
    let mut state = EmitterState::default();
    state.working_dir = ROOT.clone();
    state.working_dir.push("tests/test_pet/no_root");
    let mut meta = CrateMeta::default();
    meta.authors = Some(vec!["Me <me@example.com>".into()]);
    meta.mode = EmitMode::Crate;
    meta.no_root = true;
    state.set_meta(meta);

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&PET_SCHEMA).expect("codegen");
});
static CODEGEN_PET_CLI: Lazy<()> = Lazy::new(|| {
    let mut state = EmitterState::default();
    state.working_dir = (&*ROOT).into();
    state.working_dir.push("tests/test_pet/cli");
    let mut meta = CrateMeta::default();
    meta.authors = Some(vec!["Me <me@example.com>".into()]);
    meta.mode = EmitMode::App;
    state.set_meta(meta);

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&PET_SCHEMA).expect("codegen");
});

static CODEGEN_K8S_LIB: Lazy<()> = Lazy::new(|| {
    let mut state = EmitterState::default();
    state.working_dir = (&*ROOT).into();
    state.working_dir.push("tests");
    state.working_dir.push("test_k8s");
    state.mod_prefix = "crate::codegen::";

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&K8S_SCHEMA).expect("codegen");
});
static CODEGEN_K8S_CLI: Lazy<()> = Lazy::new(|| {
    let mut state = EmitterState::default();
    state.working_dir = (&*ROOT).into();
    state.working_dir.push("tests/test_k8s/cli");
    let mut meta = CrateMeta::default();
    assert_eq!(meta.mode, EmitMode::Module);
    meta.name = Some("test-k8s-cli".into());
    meta.version = Some("0.0.0".into());
    meta.authors = Some(vec!["Me <me@example.com>".into()]);
    meta.mode = EmitMode::App;
    state.set_meta(meta);

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&K8S_SCHEMA).expect("codegen");
});

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
enum PatchStrategy {
    Merge,
    RetainKeys,
    #[serde(rename = "merge,retainKeys")]
    MergeAndRetain,
}

#[paperclip::api_v2_schema]
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct K8sSchema {
    #[serde(rename = "x-kubernetes-patch-strategy")]
    patch_strategy: Option<PatchStrategy>,
}

static CODEGEN: Lazy<()> = Lazy::new(|| {
    env_logger::builder()
        .filter(Some("paperclip"), log::LevelFilter::Info)
        .init();
    Lazy::force(&CODEGEN_PET_LIB);
    Lazy::force(&CODEGEN_PET_LIB_NO_ROOT);
    Lazy::force(&CODEGEN_PET_CLI);
    Lazy::force(&CODEGEN_K8S_LIB);
    Lazy::force(&CODEGEN_K8S_CLI);
});

fn assert_file(path: &str) {
    Lazy::force(&CODEGEN);

    let data = std::fs::read_to_string(ROOT.join(path))
        .unwrap_or_else(|err| panic!("missing file {}: {}", path, err));
    insta::assert_snapshot!(path, data);
}

#[cfg(test)]
mod tests_pet {
    use super::*;
    #[test]
    fn test_lib_root() {
        assert_file("tests/test_pet/lib.rs");
    }

    #[test]
    fn test_no_root_manifest() {
        assert_file("tests/test_pet/no_root/Cargo.toml");
    }

    #[test]
    fn test_manifest() {
        assert_file("tests/test_pet/Cargo.toml");
    }

    #[test]
    fn test_header_parameters() {
        assert_file("tests/test_pet/pet.rs");
    }

    #[test]
    fn test_cli_params() {
        assert_file("tests/test_pet/cli/app.yaml");
    }

    #[test]
    fn test_recusrive_object() {
        assert_file("tests/test_pet/recursive_container.rs");
        assert_file("tests/test_pet/recursive_object.rs");
    }

    #[test]
    fn test_anonymous_object_definition_in_schema() {
        // An object "can" define objects in its schema without referencing
        // them from known definitions. In that case, we autogenerate stuff.
        assert_file("tests/test_pet/order.rs");
    }
    #[test]
    fn test_anonymous_object_definition_in_body() {
        assert_file("tests/test_pet/post_shipments_body.rs");
    }

    #[test]
    fn test_anonumous_object_definition_in_response() {
        assert_file("tests/test_pet/get_shipments_id_response.rs");
    }

    #[test]
    fn test_simple_array_parameter_in_path() {
        assert_file("tests/test_pet/status.rs");
    }

    #[test]
    fn test_misc() {
        assert_file("tests/test_pet/miscellaneous.rs");
    }

    #[test]
    fn test_multipart_with_file() {
        assert_file("tests/test_pet/cli/status.rs");
    }

    #[test]
    fn test_simple_any_enum() {
        assert_file("tests/test_pet/test_enum.rs");
    }
}

#[cfg(test)]
mod tests_k8s {
    use super::*;
    use paperclip::v2::models::{HttpMethod, Version};

    #[test]
    fn test_definition_ref_cycles() {
        assert_eq!(K8S_SCHEMA.swagger, Version::V2);
        assert_eq!(K8S_SCHEMA.definitions.len(), 663);

        let json_props_def = &K8S_SCHEMA.definitions
            ["io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps"];
        let desc = json_props_def.read().unwrap().description.clone();
        let all_of = json_props_def.read().unwrap().properties["allOf"].clone();
        let items = all_of.read().unwrap().items.as_ref().unwrap().clone();
        assert_eq!(items.read().unwrap().description, desc); // both point to same `JSONSchemaProps`
    }

    #[test]
    fn test_resolved_schema() {
        let resp = &K8S_SCHEMA.paths["/api/"].methods[&HttpMethod::Get].responses["200"]
            .read()
            .unwrap();
        let schema = resp.schema.as_ref().expect("bleh?").read().unwrap();
        assert!(schema.reference.is_none()); // this was a reference
        assert_eq!(
            &K8S_SCHEMA.definitions["io.k8s.apimachinery.pkg.apis.meta.v1.APIVersions"]
                .read()
                .unwrap()
                .description
                .as_ref()
                .unwrap()
                .as_str(),
            schema.description.as_ref().unwrap()
        );
    }

    #[test]
    fn test_child_module_declarations() {
        assert_file("tests/test_k8s/io/k8s/api/mod.rs");
    }

    #[test]
    fn test_transparency_with_params() {
        assert_file("tests/test_k8s/io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/custom_resource_definition.rs");
    }
    #[test]
    fn test_struct_for_complex_object() {
        assert_file("tests/test_k8s/io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/json_schema_props.rs");
    }

    #[test]
    fn test_root_mod() {
        assert_file("tests/test_k8s/mod.rs");
    }

    #[test]
    fn test_generics_mod() {
        assert_file("tests/test_k8s/generics.rs");
    }

    #[test]
    fn test_same_object_creates_multiple_builders() {
        assert_file("tests/test_k8s/io/k8s/api/core/v1/config_map.rs")
    }

    #[test]
    fn test_same_object_with_multiple_builders_has_basic_builder() {
        assert_file("tests/test_k8s/io/k8s/api/core/v1/pod.rs")
    }

    #[test]
    fn test_simple_object_builder_with_required_fields() {
        assert_file("tests/test_k8s/io/k8s/api/rbac/v1/policy_rule.rs")
    }

    #[test]
    fn test_builder_with_field_parameter_collision_and_method_collision() {
        // grace_period_seconds, orphan_dependents and propagation_policy
        // exist in both the object and as a query parameter. If one is set,
        // we should also set the other.
        assert_file("tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/delete_options.rs");
    }

    #[test]
    fn test_unit_builder_with_no_modifier() {
        assert_file("tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/api_group_list.rs");
    }

    #[test]
    fn test_builder_field_with_iterators() {
        assert_file(
            "tests/test_k8s/io/k8s/api/certificates/v1beta1/certificate_signing_request_spec.rs",
        );
    }

    #[test]
    fn test_any_in_operation_bound_to_unrelated_struct() {
        assert_file("tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/patch.rs");
    }

    #[test]
    fn test_cli_manifest() {
        assert_file("tests/test_k8s/cli/Cargo.toml");
    }

    #[test]
    fn test_cli_main() {
        assert_file("tests/test_k8s/cli/main.rs");
    }

    #[test]
    fn test_clap_yaml() {
        assert_file("tests/test_k8s/cli/app.yaml");
    }
}
