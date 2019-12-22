#[macro_use]
extern crate lazy_static;
use paperclip::api_v2_schema;
#[macro_use]
extern crate serde_derive;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, EmitMode, Emitter, EmitterState},
    models::{HttpMethod, ResolvableApi, Version},
};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref ROOT: String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref SCHEMA: ResolvableApi<K8sSchema> = {
        let fd =
            File::open(ROOT.clone() + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("file?");
        let raw: ResolvableApi<K8sSchema> = v2::from_reader(fd).expect("deserializing spec");
        raw.resolve().expect("resolution")
    };
    static ref CODEGEN: () = {
        env_logger::builder()
            .filter(Some("paperclip"), log::LevelFilter::Info)
            .init();
        let mut state = EmitterState::default();
        state.working_dir = (&*ROOT).into();
        state.working_dir.push("tests");
        state.working_dir.push("test_k8s");
        state.mod_prefix = "crate::codegen::";

        let emitter = DefaultEmitter::from(state);
        emitter.generate(&SCHEMA).expect("codegen");
    };
    static ref CLI_CODEGEN: () = {
        let _ = &*CODEGEN;
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
        emitter.generate(&SCHEMA).expect("codegen");
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
    assert_eq!(SCHEMA.definitions.len(), 663);

    let json_props_def = &SCHEMA.definitions
        ["io.k8s.apiextensions-apiserver.pkg.apis.apiextensions.v1beta1.JSONSchemaProps"];
    let desc = json_props_def.read().description.clone();
    let all_of = json_props_def.read().properties["allOf"].clone();
    let items = all_of
        .read()
        .items
        .as_ref()
        .and_then(|e| e.left_or_one_in_right())
        .unwrap()
        .clone();
    assert_eq!(items.read().description, desc); // both point to same `JSONSchemaProps`
}

#[test]
fn test_resolved_schema() {
    let resp = &SCHEMA.paths["/api/"].methods[&HttpMethod::Get].responses["200"].read();
    let schema = resp.schema.as_ref().expect("bleh?").read();
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

fn assert_file_contains_content_at(path: &str, matching_content: &str, index: Option<usize>) {
    let _ = &*CODEGEN;

    let mut contents = String::new();
    let mut fd = File::open(path).expect("missing file");
    fd.read_to_string(&mut contents).expect("reading file");

    if index.is_some() {
        assert_eq!(contents.find(matching_content), index);
    } else {
        assert!(contents.find(matching_content).is_some());
    }
}

#[test]
fn test_child_module_declarations() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/mod.rs"),
        "
pub mod admissionregistration {
    include!(\"./admissionregistration/mod.rs\");
}

pub mod apps {
    include!(\"./apps/mod.rs\");
}

pub mod auditregistration {
    include!(\"./auditregistration/mod.rs\");
}

pub mod authentication {
    include!(\"./authentication/mod.rs\");
}

pub mod authorization {
    include!(\"./authorization/mod.rs\");
}

pub mod autoscaling {
    include!(\"./autoscaling/mod.rs\");
}

pub mod batch {
    include!(\"./batch/mod.rs\");
}

pub mod certificates {
    include!(\"./certificates/mod.rs\");
}

pub mod coordination {
    include!(\"./coordination/mod.rs\");
}

pub mod core {
    include!(\"./core/mod.rs\");
}

pub mod events {
    include!(\"./events/mod.rs\");
}

pub mod extensions {
    include!(\"./extensions/mod.rs\");
}

pub mod networking {
    include!(\"./networking/mod.rs\");
}

pub mod node {
    include!(\"./node/mod.rs\");
}

pub mod policy {
    include!(\"./policy/mod.rs\");
}

pub mod rbac {
    include!(\"./rbac/mod.rs\");
}

pub mod scheduling {
    include!(\"./scheduling/mod.rs\");
}

pub mod settings {
    include!(\"./settings/mod.rs\");
}

pub mod storage {
    include!(\"./storage/mod.rs\");
}
",
        Some(0),
    );

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/batch/v1/mod.rs"),
        "
pub mod job {
    include!(\"./job.rs\");
}

pub mod job_condition {
    include!(\"./job_condition.rs\");
}

pub mod job_list {
    include!(\"./job_list.rs\");
}

pub mod job_spec {
    include!(\"./job_spec.rs\");
}

pub mod job_status {
    include!(\"./job_status.rs\");
}
",
        Some(0),
    );
}

#[test]
fn test_transparency_with_parameters() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/custom_resource_definition.rs"),
        "
/// Builder created by [`CustomResourceDefinition::create_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.create_apiextensions_v1beta1_custom_resource_definition) method for a `POST` operation associated with `CustomResourceDefinition`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CustomResourceDefinitionPostBuilder<Spec, Any> {
    inner: CustomResourceDefinitionPostBuilderContainer<Any>,
    _spec: core::marker::PhantomData<Spec>,
}

#[derive(Debug, Default, Clone)]
struct CustomResourceDefinitionPostBuilderContainer<Any> {
    body: self::CustomResourceDefinition<Any>,
    param_dry_run: Option<String>,
    param_field_manager: Option<String>,
    param_pretty: Option<String>,
}

impl<Spec, Any> CustomResourceDefinitionPostBuilder<Spec, Any> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    #[inline]
    pub fn dry_run(mut self, value: impl Into<String>) -> Self {
        self.inner.param_dry_run = Some(value.into());
        self
    }

    /// fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
    #[inline]
    pub fn field_manager(mut self, value: impl Into<String>) -> Self {
        self.inner.param_field_manager = Some(value.into());
        self
    }

    /// If 'true', then the output is pretty printed.
    #[inline]
    pub fn pretty(mut self, value: impl Into<String>) -> Self {
        self.inner.param_pretty = Some(value.into());
        self
    }

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[inline]
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.inner.body.api_version = Some(value.into());
        self
    }

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[inline]
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.inner.body.kind = Some(value.into());
        self
    }

    #[inline]
    pub fn metadata(mut self, value: crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::object_meta::ObjectMeta) -> Self {
        self.inner.body.metadata = Some(value.into());
        self
    }

    /// Spec describes how the user wants the resources to appear
    #[inline]
    pub fn spec(mut self, value: crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::custom_resource_definition_spec::CustomResourceDefinitionSpecBuilder<crate::codegen::generics::GroupExists, crate::codegen::generics::NamesExists, crate::codegen::generics::ScopeExists, Any>) -> CustomResourceDefinitionPostBuilder<crate::codegen::generics::SpecExists, Any> {
        self.inner.body.spec = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// Status indicates the actual state of the CustomResourceDefinition
    #[inline]
    pub fn status(mut self, value: crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::custom_resource_definition_status::CustomResourceDefinitionStatusBuilder<crate::codegen::generics::AcceptedNamesExists, crate::codegen::generics::ConditionsExists, crate::codegen::generics::StoredVersionsExists>) -> Self {
        self.inner.body.status = Some(value.into());
        self
    }
}

impl<Client: crate::codegen::client::ApiClient + Sync + 'static, Any: serde::Serialize> crate::codegen::client::Sendable<Client> for CustomResourceDefinitionPostBuilder<crate::codegen::generics::SpecExists, Any> {
    type Output = crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::custom_resource_definition::CustomResourceDefinition<serde_json::Value>;

    const METHOD: http::Method = http::Method::POST;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions\".into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::codegen::client::ApiError<Client::Response>> {
        use crate::codegen::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), \"application/json\")
        .query(&[
            (\"dryRun\", self.inner.param_dry_run.as_ref().map(std::string::ToString::to_string)),
            (\"fieldManager\", self.inner.param_field_manager.as_ref().map(std::string::ToString::to_string)),
            (\"pretty\", self.inner.param_pretty.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
",
        Some(7319),
    );

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/custom_resource_definition.rs"),
        "
/// Builder created by [`CustomResourceDefinition::read_apiextensions_v1beta1_custom_resource_definition`](./struct.CustomResourceDefinition.html#method.read_apiextensions_v1beta1_custom_resource_definition) method for a `GET` operation associated with `CustomResourceDefinition`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CustomResourceDefinitionGetBuilder1<Name> {
    inner: CustomResourceDefinitionGetBuilder1Container,
    _param_name: core::marker::PhantomData<Name>,
}

#[derive(Debug, Default, Clone)]
struct CustomResourceDefinitionGetBuilder1Container {
    param_exact: Option<bool>,
    param_export: Option<bool>,
    param_name: Option<String>,
    param_pretty: Option<String>,
}

impl<Name> CustomResourceDefinitionGetBuilder1<Name> {
    /// Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'. Deprecated. Planned for removal in 1.18.
    #[inline]
    pub fn exact(mut self, value: impl Into<bool>) -> Self {
        self.inner.param_exact = Some(value.into());
        self
    }

    /// Should this value be exported.  Export strips fields that a user can not specify. Deprecated. Planned for removal in 1.18.
    #[inline]
    pub fn export(mut self, value: impl Into<bool>) -> Self {
        self.inner.param_export = Some(value.into());
        self
    }

    /// name of the CustomResourceDefinition
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> CustomResourceDefinitionGetBuilder1<crate::codegen::generics::NameExists> {
        self.inner.param_name = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// If 'true', then the output is pretty printed.
    #[inline]
    pub fn pretty(mut self, value: impl Into<String>) -> Self {
        self.inner.param_pretty = Some(value.into());
        self
    }
}

impl<Client: crate::codegen::client::ApiClient + Sync + 'static> crate::codegen::client::Sendable<Client> for CustomResourceDefinitionGetBuilder1<crate::codegen::generics::NameExists> {
    type Output = CustomResourceDefinition<serde_json::Value>;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}\", name=self.inner.param_name.as_ref().expect(\"missing parameter name?\")).into()
    }
",
        Some(12543),
    );
}

#[test]
fn test_struct_for_complex_object() {
    // We're interested in this definition because:
    // - It uses some Rust keywords.
    // - It has a number of camelcase fields.
    // - It has some fields which are maps.
    // - It uses pretty much all types (including custom types).
    // - It references other definitions (directly and through an array).
    // - It's a cyclic type.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apiextensions_apiserver/pkg/apis/apiextensions/v1beta1/json_schema_props.rs"),
        "
/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct JsonSchemaProps<Any> {
    #[serde(rename = \"$ref\")]
    pub ref_: Option<String>,
    #[serde(rename = \"$schema\")]
    pub schema: Option<String>,
    #[serde(rename = \"additionalItems\")]
    pub additional_items: Option<Any>,
    #[serde(rename = \"additionalProperties\")]
    pub additional_properties: Option<Any>,
    #[serde(rename = \"allOf\")]
    pub all_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    #[serde(rename = \"anyOf\")]
    pub any_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub default: Option<Any>,
    pub definitions: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub dependencies: Option<std::collections::BTreeMap<String, Any>>,
    pub description: Option<String>,
    #[serde(rename = \"enum\")]
    pub enum_: Option<Vec<Any>>,
    pub example: Option<Any>,
    #[serde(rename = \"exclusiveMaximum\")]
    pub exclusive_maximum: Option<bool>,
    #[serde(rename = \"exclusiveMinimum\")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = \"externalDocs\")]
    pub external_docs: Option<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::external_documentation::ExternalDocumentation>,
    pub format: Option<String>,
    pub id: Option<String>,
    pub items: Option<Any>,
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
    pub not: Option<Box<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub nullable: Option<bool>,
    #[serde(rename = \"oneOf\")]
    pub one_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub pattern: Option<String>,
    #[serde(rename = \"patternProperties\")]
    pub pattern_properties: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub properties: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>>,
    pub required: Option<Vec<String>>,
    pub title: Option<String>,
    #[serde(rename = \"type\")]
    pub type_: Option<String>,
    #[serde(rename = \"uniqueItems\")]
    pub unique_items: Option<bool>,
    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).
    #[serde(rename = \"x-kubernetes-embedded-resource\")]
    pub x_kubernetes_embedded_resource: Option<bool>,
    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///    - type: integer
    ///    - type: string
    /// 2) allOf:
    ///    - anyOf:
    ///      - type: integer
    ///      - type: string
    ///    - ... zero or more
    #[serde(rename = \"x-kubernetes-int-or-string\")]
    pub x_kubernetes_int_or_string: Option<bool>,
    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.
    #[serde(rename = \"x-kubernetes-preserve-unknown-fields\")]
    pub x_kubernetes_preserve_unknown_fields: Option<bool>,
}

impl<Any: Default> JsonSchemaProps<Any> {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> JsonSchemaPropsBuilder<Any> {
        JsonSchemaPropsBuilder {
            body: Default::default(),
        }
    }
}

impl<Any> Into<JsonSchemaProps<Any>> for JsonSchemaPropsBuilder<Any> {
    fn into(self) -> JsonSchemaProps<Any> {
        self.body
    }
}

/// Builder for [`JsonSchemaProps`](./struct.JsonSchemaProps.html) object.
#[derive(Debug, Clone)]
pub struct JsonSchemaPropsBuilder<Any> {
    body: self::JsonSchemaProps<Any>,
}

impl<Any> JsonSchemaPropsBuilder<Any> {
    #[inline]
    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.body.ref_ = Some(value.into());
        self
    }

    #[inline]
    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.body.schema = Some(value.into());
        self
    }

    #[inline]
    pub fn additional_items(mut self, value: impl Into<Any>) -> Self {
        self.body.additional_items = Some(value.into());
        self
    }

    #[inline]
    pub fn additional_properties(mut self, value: impl Into<Any>) -> Self {
        self.body.additional_properties = Some(value.into());
        self
    }

    #[inline]
    pub fn all_of(mut self, value: impl Iterator<Item = crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>) -> Self {
        self.body.all_of = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn any_of(mut self, value: impl Iterator<Item = crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>) -> Self {
        self.body.any_of = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn default(mut self, value: impl Into<Any>) -> Self {
        self.body.default = Some(value.into());
        self
    }

    #[inline]
    pub fn definitions(mut self, value: impl Iterator<Item = (String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>)>) -> Self {
        self.body.definitions = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn dependencies(mut self, value: impl Iterator<Item = (String, impl Into<Any>)>) -> Self {
        self.body.dependencies = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.body.description = Some(value.into());
        self
    }

    #[inline]
    pub fn r#enum(mut self, value: impl Iterator<Item = impl Into<Any>>) -> Self {
        self.body.enum_ = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn example(mut self, value: impl Into<Any>) -> Self {
        self.body.example = Some(value.into());
        self
    }

    #[inline]
    pub fn exclusive_maximum(mut self, value: impl Into<bool>) -> Self {
        self.body.exclusive_maximum = Some(value.into());
        self
    }

    #[inline]
    pub fn exclusive_minimum(mut self, value: impl Into<bool>) -> Self {
        self.body.exclusive_minimum = Some(value.into());
        self
    }

    #[inline]
    pub fn external_docs(mut self, value: crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::external_documentation::ExternalDocumentation) -> Self {
        self.body.external_docs = Some(value.into());
        self
    }

    #[inline]
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.body.format = Some(value.into());
        self
    }

    #[inline]
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.body.id = Some(value.into());
        self
    }

    #[inline]
    pub fn items(mut self, value: impl Into<Any>) -> Self {
        self.body.items = Some(value.into());
        self
    }

    #[inline]
    pub fn max_items(mut self, value: impl Into<i64>) -> Self {
        self.body.max_items = Some(value.into());
        self
    }

    #[inline]
    pub fn max_length(mut self, value: impl Into<i64>) -> Self {
        self.body.max_length = Some(value.into());
        self
    }

    #[inline]
    pub fn max_properties(mut self, value: impl Into<i64>) -> Self {
        self.body.max_properties = Some(value.into());
        self
    }

    #[inline]
    pub fn maximum(mut self, value: impl Into<f64>) -> Self {
        self.body.maximum = Some(value.into());
        self
    }

    #[inline]
    pub fn min_items(mut self, value: impl Into<i64>) -> Self {
        self.body.min_items = Some(value.into());
        self
    }

    #[inline]
    pub fn min_length(mut self, value: impl Into<i64>) -> Self {
        self.body.min_length = Some(value.into());
        self
    }

    #[inline]
    pub fn min_properties(mut self, value: impl Into<i64>) -> Self {
        self.body.min_properties = Some(value.into());
        self
    }

    #[inline]
    pub fn minimum(mut self, value: impl Into<f64>) -> Self {
        self.body.minimum = Some(value.into());
        self
    }

    #[inline]
    pub fn multiple_of(mut self, value: impl Into<f64>) -> Self {
        self.body.multiple_of = Some(value.into());
        self
    }

    #[inline]
    pub fn not(mut self, value: crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>) -> Self {
        self.body.not = Some(value.into());
        self
    }

    #[inline]
    pub fn nullable(mut self, value: impl Into<bool>) -> Self {
        self.body.nullable = Some(value.into());
        self
    }

    #[inline]
    pub fn one_of(mut self, value: impl Iterator<Item = crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>>) -> Self {
        self.body.one_of = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.body.pattern = Some(value.into());
        self
    }

    #[inline]
    pub fn pattern_properties(mut self, value: impl Iterator<Item = (String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>)>) -> Self {
        self.body.pattern_properties = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn properties(mut self, value: impl Iterator<Item = (String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps<Any>)>) -> Self {
        self.body.properties = Some(value.map(|(key, value)| (key, value.into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    #[inline]
    pub fn required(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.required = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    #[inline]
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.body.title = Some(value.into());
        self
    }

    #[inline]
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.body.type_ = Some(value.into());
        self
    }

    #[inline]
    pub fn unique_items(mut self, value: impl Into<bool>) -> Self {
        self.body.unique_items = Some(value.into());
        self
    }

    /// x-kubernetes-embedded-resource defines that the value is an embedded Kubernetes runtime.Object, with TypeMeta and ObjectMeta. The type must be object. It is allowed to further restrict the embedded object. kind, apiVersion and metadata are validated automatically. x-kubernetes-preserve-unknown-fields is allowed to be true, but does not have to be if the object is fully specified (up to kind, apiVersion, metadata).
    #[inline]
    pub fn x_kubernetes_embedded_resource(mut self, value: impl Into<bool>) -> Self {
        self.body.x_kubernetes_embedded_resource = Some(value.into());
        self
    }

    /// x-kubernetes-int-or-string specifies that this value is either an integer or a string. If this is true, an empty type is allowed and type as child of anyOf is permitted if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///    - type: integer
    ///    - type: string
    /// 2) allOf:
    ///    - anyOf:
    ///      - type: integer
    ///      - type: string
    ///    - ... zero or more
    #[inline]
    pub fn x_kubernetes_int_or_string(mut self, value: impl Into<bool>) -> Self {
        self.body.x_kubernetes_int_or_string = Some(value.into());
        self
    }

    /// x-kubernetes-preserve-unknown-fields stops the API server decoding step from pruning fields which are not specified in the validation schema. This affects fields recursively, but switches back to normal pruning behaviour if nested properties or additionalProperties are specified in the schema. This can either be true or undefined. False is forbidden.
    #[inline]
    pub fn x_kubernetes_preserve_unknown_fields(mut self, value: impl Into<bool>) -> Self {
        self.body.x_kubernetes_preserve_unknown_fields = Some(value.into());
        self
    }
}
",
        Some(0),
    );
}

#[test]
fn test_root_mod() {
    // Root mod contains the builder markers and helper code for client.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/mod.rs"),
        "
pub mod io {
    include!(\"./io/mod.rs\");
}

pub mod miscellaneous {
    include!(\"./miscellaneous.rs\");
}

pub mod client {
    use crate::codegen::util::{AsyncReadStream, ResponseStream};
    use failure::Fail;
    use futures::{Stream, Future};
    use futures_preview::compat::Future01CompatExt;
    use parking_lot::Mutex;

    use std::borrow::Cow;
    use std::fmt::Debug;
    use std::path::Path;

    /// Common API errors.
    #[derive(Debug, Fail)]
    pub enum ApiError<R: Debug + Send + 'static> {
        #[fail(display = \"API request failed for path: {} (code: {})\", _0, _1)]
        Failure(String, http::status::StatusCode, Mutex<R>),
        #[fail(display = \"Unsupported media type in response: {}\", _0)]
        UnsupportedMediaType(String, Mutex<R>),
        #[fail(display = \"An error has occurred while performing the API request: {}\", _0)]
        Reqwest(reqwest::Error),
        #[fail(display = \"I/O error: {}\", _0)]
        Io(std::io::Error),
        #[fail(display = \"Error en/decoding \\\"application/json\\\" data: {}\", _0)]
        ApplicationJson(serde_json::Error),
        #[fail(display = \"Error en/decoding \\\"application/yaml\\\" data: {}\", _0)]
        ApplicationYaml(serde_yaml::Error),
    }

    /// Form object for building multipart request body.
    pub trait Form: Sized {
        /// Creates a new builder.
        fn new() -> Self;

        /// Adds the given key and value as text.
        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>;

        /// Adds the file from the given path for streaming.
        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>;
    }

    /// HTTP Request.
    pub trait Request {
        type Form: Form;

        /// Sets the header with the given key and value.
        fn header(self, name: &'static str, value: &str) -> Self;

        /// Sets body using the given vector of bytes.
        ///
        /// **NOTE:** Appropriate `Content-Type` header must be set
        /// after calling this method.
        fn body_bytes(self, body: Vec<u8>) -> Self;

        /// Sets JSON body based on the given value.
        fn json<T: serde::Serialize>(self, value: &T) -> Self;

        /// Sets `multipart/form-data` body using the given form.
        fn multipart_form_data(self, form: Self::Form) -> Self;

        /// Sets/adds query parameters based on the given value.
        ///
        /// **NOTE:** This method must be called only once. It's unspecified
        /// as to whether this appends/replaces query parameters.
        fn query<T: serde::Serialize>(self, params: &T) -> Self;
    }

    impl Form for reqwest::r#async::multipart::Form {
        fn new() -> Self {
            reqwest::r#async::multipart::Form::new()
        }

        fn text<T, U>(self, key: T, value: U) -> Self
            where T: Into<Cow<'static, str>>,
                  U: Into<Cow<'static, str>>
        {
            reqwest::r#async::multipart::Form::text(self, key, value)
        }

        fn file<K>(self, key: K, path: &Path) -> std::io::Result<Self>
            where K: Into<Cow<'static, str>>
        {
            let fd = std::fs::File::open(path)?;
            let reader = std::io::BufReader::new(tokio_fs_old::File::from_std(fd));
            Ok(reqwest::r#async::multipart::Form::part(self, key, AsyncReadStream::from(reader).into()))
        }
    }

    impl Request for reqwest::r#async::RequestBuilder {
        type Form = reqwest::r#async::multipart::Form;

        fn header(self, name: &'static str, value: &str) -> Self {
            reqwest::r#async::RequestBuilder::header(self, name, value)
        }

        fn multipart_form_data(self, form: Self::Form) -> Self {
            self.multipart(form)
        }

        fn body_bytes(self, body: Vec<u8>) -> Self {
            self.body(body)
        }

        fn json<T: serde::Serialize>(self, value: &T) -> Self {
            reqwest::r#async::RequestBuilder::json(self, value)
        }

        fn query<T: serde::Serialize>(self, params: &T) -> Self {
            reqwest::r#async::RequestBuilder::query(self, params)
        }
    }

    /// HTTP Response.
    #[async_trait::async_trait]
    pub trait Response: Debug + Send + Sized {
        type Bytes: AsRef<[u8]>;
        type Stream;

        /// Gets the value for the given header name, if any.
        fn header(&self, name: &'static str) -> Option<&str>;

        /// Status code for this response.
        fn status(&self) -> http::status::StatusCode;

        /// Media type for this response body (if any).
        fn media_type(&self) -> Option<mime::MediaType>;

        /// Response body as a stream.
        fn stream(&mut self) -> ResponseStream<Self::Stream>;

        /// Vector of bytes from the response body.
        async fn body_bytes(self) -> Result<(Self, Self::Bytes), ApiError<Self>>;
    }

    #[async_trait::async_trait]
    impl Response for reqwest::r#async::Response {
        type Bytes = reqwest::r#async::Chunk;
        type Stream = reqwest::r#async::Decoder;

        fn header(&self, name: &'static str) -> Option<&str> {
            self.headers().get(name).and_then(|v| v.to_str().ok())
        }

        fn status(&self) -> http::status::StatusCode {
            reqwest::r#async::Response::status(self)
        }

        fn media_type(&self) -> Option<mime::MediaType> {
            self.header(http::header::CONTENT_TYPE.as_str())
                .and_then(|v| v.parse().ok())
        }

        fn stream(&mut self) -> ResponseStream<Self::Stream> {
            let body = std::mem::replace(self.body_mut(), reqwest::r#async::Decoder::empty());
            ResponseStream::from(body)
        }

        async fn body_bytes(mut self) -> Result<(Self, Self::Bytes), ApiError<Self>> {
            let body = std::mem::replace(self.body_mut(), reqwest::r#async::Decoder::empty());
            let bytes = body.concat2().map_err(ApiError::Reqwest).compat().await?;
            Ok((self, bytes))
        }
    }

    /// Represents an API client.
    #[async_trait::async_trait]
    pub trait ApiClient {
        type Request: Request + Send;
        type Response: Response;

        /// Consumes a method and a relative path and produces a request builder for a single API call.
        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request;

        /// Performs the HTTP request using the given `Request` object
        /// and returns a `Response` future.
        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>>;
    }

    #[async_trait::async_trait]
    impl ApiClient for reqwest::r#async::Client {
        type Request = reqwest::r#async::RequestBuilder;
        type Response = reqwest::r#async::Response;

        fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
            let mut u = String::from(\"https://example.com/\");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }

        async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
            let req = req.build().map_err(ApiError::Reqwest)?;
            let resp = self.execute(req).map_err(ApiError::Reqwest).compat().await?;
            Ok(resp)
        }
    }

    /// A trait for indicating that the implementor can send an API call.
    #[async_trait::async_trait]
    pub trait Sendable<Client>
    where
        Client: ApiClient + Sync + 'static,
    {
        /// The output object from this API request.
        type Output: serde::de::DeserializeOwned;

        /// HTTP method used by this call.
        const METHOD: http::Method;

        /// Relative URL for this API call formatted appropriately with parameter values.
        ///
        /// **NOTE:** This URL **must** begin with `/`.
        fn rel_path(&self) -> std::borrow::Cow<'static, str>;

        /// Modifier for this object. Builders override this method if they
        /// wish to add query parameters, set body, etc.
        fn modify(&self, req: Client::Request) -> Result<Client::Request, ApiError<Client::Response>> {
            Ok(req)
        }

        /// Sends the request and returns a future for the response object.
        async fn send(&self, client: &Client) -> Result<Self::Output, ApiError<Client::Response>> {
            let resp = self.send_raw(client).await?;
            let media = resp.media_type();
            if let Some(ty) = media {
                if media_types::M_0.matches(&ty) {
                    let (_, bytes) = resp.body_bytes().await?;
                    return serde_json::from_reader(bytes.as_ref()).map_err(ApiError::from)
                }
                else if media_types::M_1.matches(&ty) {
                    let (_, bytes) = resp.body_bytes().await?;
                    return serde_yaml::from_reader(bytes.as_ref()).map_err(ApiError::from)
                }
            }

            let ty = resp.header(http::header::CONTENT_TYPE.as_str())
                .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
                .unwrap_or_default();
            Err(ApiError::UnsupportedMediaType(ty, Mutex::new(resp)))
        }

        /// Convenience method for returning a raw response after sending a request.
        async fn send_raw(&self, client: &Client) -> Result<Client::Response, ApiError<Client::Response>> {
            let rel_path = self.rel_path();
            let req = self.modify(client.request_builder(Self::METHOD, &rel_path))?;
            let resp = client.make_request(req).await?;
            if resp.status().is_success() {
                Ok(resp)
            } else {
                Err(ApiError::Failure(rel_path.into_owned(), resp.status(), Mutex::new(resp)))
            }
        }
    }

    pub mod media_types {
        use lazy_static::lazy_static;

        lazy_static! {
            pub static ref M_0: mime::MediaRange =
                mime::MediaRange::parse(\"application/json\").expect(\"cannot parse \\\"application/json\\\" as media range\");
            pub static ref M_1: mime::MediaRange =
                mime::MediaRange::parse(\"application/yaml\").expect(\"cannot parse \\\"application/yaml\\\" as media range\");
        }
    }

    impl<R: Response + 'static> From<std::io::Error> for ApiError<R> {
        fn from(e: std::io::Error) -> Self {
            ApiError::Io(e)
        }
    }

    impl<R: Response + 'static> From<serde_json::Error> for ApiError<R> {
        fn from(e: serde_json::Error) -> Self {
            ApiError::ApplicationJson(e)
        }
    }

    impl<R: Response + 'static> From<serde_yaml::Error> for ApiError<R> {
        fn from(e: serde_yaml::Error) -> Self {
            ApiError::ApplicationYaml(e)
        }
    }
}

pub mod generics {
    include!(\"./generics.rs\");
}

pub mod util {
    include!(\"./util.rs\");
}
",
        Some(0),
    );
}

#[test]
fn test_generics_mod() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/generics.rs"),
        "
pub struct",
        Some(0),
    );
}

#[test]
fn test_same_object_creates_multiple_builders() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/core/v1/config_map.rs"),
        "
impl ConfigMap {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ConfigMapBuilder {
        ConfigMapBuilder {
            body: Default::default(),
        }
    }

    /// create a ConfigMap
    #[inline]
    pub fn create_core_v1_namespaced_config_map() -> ConfigMapPostBuilder<crate::codegen::generics::MissingNamespace> {
        ConfigMapPostBuilder {
            inner: Default::default(),
            _param_namespace: core::marker::PhantomData,
        }
    }

    /// read the specified ConfigMap
    #[inline]
    pub fn read_core_v1_namespaced_config_map() -> ConfigMapGetBuilder1<crate::codegen::generics::MissingName, crate::codegen::generics::MissingNamespace> {
        ConfigMapGetBuilder1 {
            inner: Default::default(),
            _param_name: core::marker::PhantomData,
            _param_namespace: core::marker::PhantomData,
        }
    }

    /// replace the specified ConfigMap
    #[inline]
    pub fn replace_core_v1_namespaced_config_map() -> ConfigMapPutBuilder1<crate::codegen::generics::MissingName, crate::codegen::generics::MissingNamespace> {
        ConfigMapPutBuilder1 {
            inner: Default::default(),
            _param_name: core::marker::PhantomData,
            _param_namespace: core::marker::PhantomData,
        }
    }
}

impl Into<ConfigMap> for ConfigMapBuilder {
    fn into(self) -> ConfigMap {
        self.body
    }
}

impl Into<ConfigMap> for ConfigMapPostBuilder<crate::codegen::generics::NamespaceExists> {
    fn into(self) -> ConfigMap {
        self.inner.body
    }
}

impl Into<ConfigMap> for ConfigMapPutBuilder1<crate::codegen::generics::NameExists, crate::codegen::generics::NamespaceExists> {
    fn into(self) -> ConfigMap {
        self.inner.body
    }
}
",
        Some(1889),
    );
}

#[test]
fn test_same_object_with_multiple_builders_has_basic_builder() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/core/v1/pod.rs"),
        "
/// Builder for [`Pod`](./struct.Pod.html) object.
#[derive(Debug, Clone)]
pub struct PodBuilder {
    body: self::Pod,
}

impl PodBuilder {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[inline]
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.body.api_version = Some(value.into());
        self
    }

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[inline]
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.body.kind = Some(value.into());
        self
    }

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[inline]
    pub fn metadata(mut self, value: crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::object_meta::ObjectMeta) -> Self {
        self.body.metadata = Some(value.into());
        self
    }

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[inline]
    pub fn spec(mut self, value: crate::codegen::io::k8s::api::core::v1::pod_spec::PodSpecBuilder<crate::codegen::generics::ContainersExists>) -> Self {
        self.body.spec = Some(value.into());
        self
    }

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[inline]
    pub fn status(mut self, value: crate::codegen::io::k8s::api::core::v1::pod_status::PodStatus) -> Self {
        self.body.status = Some(value.into());
        self
    }
}
",
        Some(4237),
    )
}

#[test]
fn test_simple_object_builder_with_required_fields() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/rbac/v1/policy_rule.rs"),
        "
impl PolicyRule {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> PolicyRuleBuilder<crate::codegen::generics::MissingVerbs> {
        PolicyRuleBuilder {
            body: Default::default(),
            _verbs: core::marker::PhantomData,
        }
    }
}

impl Into<PolicyRule> for PolicyRuleBuilder<crate::codegen::generics::VerbsExists> {
    fn into(self) -> PolicyRule {
        self.body
    }
}

/// Builder for [`PolicyRule`](./struct.PolicyRule.html) object.
#[derive(Debug, Clone)]
pub struct PolicyRuleBuilder<Verbs> {
    body: self::PolicyRule,
    _verbs: core::marker::PhantomData<Verbs>,
}

impl<Verbs> PolicyRuleBuilder<Verbs> {
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.
    #[inline]
    pub fn api_groups(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.api_groups = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both.
    #[inline]
    pub fn non_resource_ur_ls(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.non_resource_ur_ls = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[inline]
    pub fn resource_names(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.resource_names = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
    #[inline]
    pub fn resources(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.resources = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
    #[inline]
    pub fn verbs(mut self, value: impl Iterator<Item = impl Into<String>>) -> PolicyRuleBuilder<crate::codegen::generics::VerbsExists> {
        self.body.verbs = value.map(|value| value.into()).collect::<Vec<_>>().into();
        unsafe { std::mem::transmute(self) }
    }
}
",
        Some(1564),
    );
}

#[test]
fn test_builder_with_field_parameter_collision_and_method_collision() {
    // grace_period_seconds, orphan_dependents and propagation_policy
    // exist in both the object and as a query parameter. If one is set,
    // we should also set the other.
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/delete_options.rs"),
        "
/// Builder created by [`DeleteOptions::delete_rbac_authorization_v1_namespaced_role`](./struct.DeleteOptions.html#method.delete_rbac_authorization_v1_namespaced_role) method for a `DELETE` operation associated with `DeleteOptions`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DeleteOptionsDeleteBuilder59<Name, Namespace> {
    inner: DeleteOptionsDeleteBuilder59Container,
    _param_name: core::marker::PhantomData<Name>,
    _param_namespace: core::marker::PhantomData<Namespace>,
}

#[derive(Debug, Default, Clone)]
struct DeleteOptionsDeleteBuilder59Container {
    body: self::DeleteOptions,
    param_dry_run: Option<String>,
    param_grace_period_seconds: Option<i64>,
    param_orphan_dependents: Option<bool>,
    param_propagation_policy: Option<String>,
    param_name: Option<String>,
    param_namespace: Option<String>,
    param_pretty: Option<String>,
}

impl<Name, Namespace> DeleteOptionsDeleteBuilder59<Name, Namespace> {
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    #[inline]
    pub fn dry_run(mut self, value: impl Into<String>) -> Self {
        self.inner.param_dry_run = Some(value.into());
        self
    }

    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    #[inline]
    pub fn grace_period_seconds(mut self, value: impl Into<i64>) -> Self {
        self.inner.param_grace_period_seconds = Some({
            let val = value.into();
            self.inner.body.grace_period_seconds = val.clone().into();
            val
        });
        self
    }

    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \"orphan\" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    #[inline]
    pub fn orphan_dependents(mut self, value: impl Into<bool>) -> Self {
        self.inner.param_orphan_dependents = Some({
            let val = value.into();
            self.inner.body.orphan_dependents = val.clone().into();
            val
        });
        self
    }

    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    #[inline]
    pub fn propagation_policy(mut self, value: impl Into<String>) -> Self {
        self.inner.param_propagation_policy = Some({
            let val = value.into();
            self.inner.body.propagation_policy = val.clone().into();
            val
        });
        self
    }

    /// name of the Role
    #[inline]
    pub fn name(mut self, value: impl Into<String>) -> DeleteOptionsDeleteBuilder59<crate::codegen::generics::NameExists, Namespace> {
        self.inner.param_name = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// object name and auth scope, such as for teams and projects
    #[inline]
    pub fn namespace(mut self, value: impl Into<String>) -> DeleteOptionsDeleteBuilder59<Name, crate::codegen::generics::NamespaceExists> {
        self.inner.param_namespace = Some(value.into());
        unsafe { std::mem::transmute(self) }
    }

    /// If 'true', then the output is pretty printed.
    #[inline]
    pub fn pretty(mut self, value: impl Into<String>) -> Self {
        self.inner.param_pretty = Some(value.into());
        self
    }

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[inline]
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.inner.body.api_version = Some(value.into());
        self
    }

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[inline]
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.inner.body.kind = Some(value.into());
        self
    }

    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    #[inline]
    pub fn preconditions(mut self, value: crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::preconditions::Preconditions) -> Self {
        self.inner.body.preconditions = Some(value.into());
        self
    }
}

impl<Client: crate::codegen::client::ApiClient + Sync + 'static> crate::codegen::client::Sendable<Client> for DeleteOptionsDeleteBuilder59<crate::codegen::generics::NameExists, crate::codegen::generics::NamespaceExists> {
    type Output = crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::status::Status;

    const METHOD: http::Method = http::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/roles/{name}\", name=self.inner.param_name.as_ref().expect(\"missing parameter name?\"), namespace=self.inner.param_namespace.as_ref().expect(\"missing parameter namespace?\")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::codegen::client::ApiError<Client::Response>> {
        use crate::codegen::client::Request;
        Ok(req
        .json(&self.inner.body)
        .query(&[
            (\"dryRun\", self.inner.param_dry_run.as_ref().map(std::string::ToString::to_string)),
            (\"gracePeriodSeconds\", self.inner.param_grace_period_seconds.as_ref().map(std::string::ToString::to_string)),
            (\"orphanDependents\", self.inner.param_orphan_dependents.as_ref().map(std::string::ToString::to_string)),
            (\"propagationPolicy\", self.inner.param_propagation_policy.as_ref().map(std::string::ToString::to_string)),
            (\"pretty\", self.inner.param_pretty.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
",
        Some(448229),
    );
}

#[test]
fn test_unit_builder_with_no_modifier() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/api_group_list.rs"),
        "
impl ApiGroupList {
    /// Create a builder for this object.
    #[inline]
    pub fn builder() -> ApiGroupListBuilder<crate::codegen::generics::MissingGroups> {
        ApiGroupListBuilder {
            body: Default::default(),
            _groups: core::marker::PhantomData,
        }
    }

    /// get available API versions
    #[inline]
    pub fn get_api_versions() -> ApiGroupListGetBuilder {
        ApiGroupListGetBuilder
    }
}

impl Into<ApiGroupList> for ApiGroupListBuilder<crate::codegen::generics::GroupsExists> {
    fn into(self) -> ApiGroupList {
        self.body
    }
}

/// Builder for [`ApiGroupList`](./struct.ApiGroupList.html) object.
#[derive(Debug, Clone)]
pub struct ApiGroupListBuilder<Groups> {
    body: self::ApiGroupList,
    _groups: core::marker::PhantomData<Groups>,
}

impl<Groups> ApiGroupListBuilder<Groups> {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[inline]
    pub fn api_version(mut self, value: impl Into<String>) -> Self {
        self.body.api_version = Some(value.into());
        self
    }

    /// groups is a list of APIGroup.
    #[inline]
    pub fn groups(mut self, value: impl Iterator<Item = crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::api_group::ApiGroupBuilder<crate::codegen::generics::NameExists, crate::codegen::generics::VersionsExists>>) -> ApiGroupListBuilder<crate::codegen::generics::GroupsExists> {
        self.body.groups = value.map(|value| value.into()).collect::<Vec<_>>().into();
        unsafe { std::mem::transmute(self) }
    }

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[inline]
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.body.kind = Some(value.into());
        self
    }
}

/// Builder created by [`ApiGroupList::get_api_versions`](./struct.ApiGroupList.html#method.get_api_versions) method for a `GET` operation associated with `ApiGroupList`.
#[derive(Debug, Clone)]
pub struct ApiGroupListGetBuilder;


impl<Client: crate::codegen::client::ApiClient + Sync + 'static> crate::codegen::client::Sendable<Client> for ApiGroupListGetBuilder {
    type Output = ApiGroupList;

    const METHOD: http::Method = http::Method::GET;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        \"/apis/\".into()
    }
}
",
        Some(979),
    );
}

#[test]
fn test_builder_field_with_iterators() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/api/certificates/v1beta1/certificate_signing_request_spec.rs"),
        "
/// Builder for [`CertificateSigningRequestSpec`](./struct.CertificateSigningRequestSpec.html) object.
#[derive(Debug, Clone)]
pub struct CertificateSigningRequestSpecBuilder<Request> {
    body: self::CertificateSigningRequestSpec,
    _request: core::marker::PhantomData<Request>,
}

impl<Request> CertificateSigningRequestSpecBuilder<Request> {
    /// Extra information about the requesting user. See user.Info interface for details.
    #[inline]
    pub fn extra(mut self, value: impl Iterator<Item = (String, impl Iterator<Item = impl Into<String>>)>) -> Self {
        self.body.extra = Some(value.map(|(key, value)| (key, value.map(|value| value.into()).collect::<Vec<_>>().into())).collect::<std::collections::BTreeMap<_, _>>().into());
        self
    }

    /// Group information about the requesting user. See user.Info interface for details.
    #[inline]
    pub fn groups(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.groups = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Base64-encoded PKCS#10 CSR data
    #[inline]
    pub fn request(mut self, value: impl Into<String>) -> CertificateSigningRequestSpecBuilder<crate::codegen::generics::RequestExists> {
        self.body.request = value.into();
        unsafe { std::mem::transmute(self) }
    }

    /// UID information about the requesting user. See user.Info interface for details.
    #[inline]
    pub fn uid(mut self, value: impl Into<String>) -> Self {
        self.body.uid = Some(value.into());
        self
    }

    /// allowedUsages specifies a set of usage contexts the key will be valid for. See: https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    ///      https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    #[inline]
    pub fn usages(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.usages = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
        self
    }

    /// Information about the requesting user. See user.Info interface for details.
    #[inline]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.body.username = Some(value.into());
        self
    }
}
",
        Some(1686),
    );
}

#[test]
fn test_any_in_operation_bound_to_unrelated_struct() {
    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/io/k8s/apimachinery/pkg/apis/meta/v1/patch.rs"),
        "
impl<Client: crate::codegen::client::ApiClient + Sync + 'static> crate::codegen::client::Sendable<Client> for PatchPatchBuilder26<crate::codegen::generics::NameExists> {
    type Output = crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::custom_resource_definition::CustomResourceDefinition<serde_json::Value>;

    const METHOD: http::Method = http::Method::PATCH;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/apis/apiextensions.k8s.io/v1beta1/customresourcedefinitions/{name}\", name=self.inner.param_name.as_ref().expect(\"missing parameter name?\")).into()
    }

    fn modify(&self, req: Client::Request) -> Result<Client::Request, crate::codegen::client::ApiError<Client::Response>> {
        use crate::codegen::client::Request;
        Ok(req
        .json(&self.inner.body)
        .header(http::header::ACCEPT.as_str(), \"application/json\")
        .query(&[
            (\"dryRun\", self.inner.param_dry_run.as_ref().map(std::string::ToString::to_string)),
            (\"fieldManager\", self.inner.param_field_manager.as_ref().map(std::string::ToString::to_string)),
            (\"force\", self.inner.param_force.as_ref().map(std::string::ToString::to_string)),
            (\"pretty\", self.inner.param_pretty.as_ref().map(std::string::ToString::to_string))
        ]))
    }
}
",
        Some(180127),
    );
}

#[test]
fn test_cli_manifest() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/cli/Cargo.toml"),
        "
[[bin]]
name = \"test-k8s-cli\"
path = \"main.rs\"

[dependencies]
async-trait = \"0.1\"
failure = \"0.1\"
futures = \"0.1\"
futures-preview = { version = \"0.3.0-alpha.19\", features = [\"compat\"], package = \"futures-preview\" }
http = \"0.1\"
lazy_static = \"1.4\"
log = \"0.4\"
mime = { git = \"https://github.com/hyperium/mime\" }
mime_guess = \"2.0\"
parking_lot = \"0.8\"
reqwest = \"0.9\"
serde = \"1.0\"
serde_json = \"1.0\"
serde_yaml = \"0.8\"
tokio-io-old = { version = \"0.1\", package = \"tokio-io\" }
tokio-fs-old = { version = \"0.1\", package = \"tokio-fs\" }
url = \"2.1\"

clap = { version = \"2.33\", features = [\"yaml\"] }
env_logger = \"0.6\"
humantime = \"1.2\"
openssl = { version = \"0.10\", features = [\"vendored\"] }
tokio = { version = \"0.2.0-alpha.6\", features = [\"rt-current-thread\"] }

[workspace]
",
        Some(101),
    );
}

#[test]
fn test_cli_main() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/cli/main.rs"),
        "
use self::client::{ApiClient, ApiError};
use clap::{App, ArgMatches};
use failure::Error;
use futures::{Future, Stream};
use futures_preview::compat::Future01CompatExt;
use openssl::pkcs12::Pkcs12;
use openssl::pkey::PKey;
use openssl::x509::X509;

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Fail)]
#[allow(dead_code)]
enum ClientError {
    #[fail(display = \"Duration parse error: {}\", _0)]
    Duration(humantime::DurationError),
    #[fail(display = \"I/O error: {}\", _0)]
    Io(std::io::Error),
    #[fail(display = \"OpenSSL error: {}\", _0)]
    OpenSsl(openssl::error::ErrorStack),
    #[fail(display = \"Client error: {}\", _0)]
    Reqwest(reqwest::Error),
    #[fail(display = \"URL error: {}\", _0)]
    Url(reqwest::UrlError),
    #[fail(display = \"{}\", _0)]
    Api(self::client::ApiError<reqwest::r#async::Response>),
    #[fail(display = \"\")]
    Empty,
}

impl From<ApiError<reqwest::r#async::Response>> for ClientError {
    fn from(e: ApiError<reqwest::r#async::Response>) -> Self {
        ClientError::Api(e)
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut data = vec![];
    let mut fd = File::open(path.as_ref()).map_err(ClientError::Io)?;
    fd.read_to_end(&mut data).map_err(ClientError::Io)?;
    Ok(data)
}

#[derive(Clone)]
struct WrappedClient {
    verbose: bool,
    inner: reqwest::r#async::Client,
    url: reqwest::Url,
}

#[async_trait::async_trait]
impl ApiClient for WrappedClient {
    type Request = reqwest::r#async::RequestBuilder;
    type Response = reqwest::r#async::Response;

    async fn make_request(&self, req: Self::Request) -> Result<Self::Response, ApiError<Self::Response>> {
        let req = req.build().map_err(ApiError::Reqwest)?;
        if self.verbose {
            println!(\"{} {}\", req.method(), req.url());
        }

        Ok(self.inner.execute(req).map_err(ApiError::Reqwest).compat().await?)
    }

    fn request_builder(&self, method: http::Method, rel_path: &str) -> Self::Request {
        let mut u = self.url.clone();
        let mut path = u.path().trim_matches('/').to_owned();
        if !path.is_empty() {
            path = String::from(\"/\") + &path;
        }

        path.push_str(rel_path);
        u.set_path(&path);
        self.inner.request(method, u)
    }
}

fn make_client<'a>(matches: &'a ArgMatches<'a>) -> Result<WrappedClient, Error> {
    let mut client = reqwest::r#async::Client::builder();

    if let Some(p) = matches.value_of(\"ca-cert\") {
        let ca_cert = X509::from_pem(&read_file(p)?)
            .map_err(ClientError::OpenSsl)?;
        let ca_der = ca_cert.to_der().map_err(ClientError::OpenSsl)?;
        client = client.add_root_certificate(
            reqwest::Certificate::from_der(&ca_der)
                .map_err(ClientError::Reqwest)?
        );
    }

    // FIXME: Is this the only way?
    if let (Some(p1), Some(p2)) = (matches.value_of(\"client-key\"), matches.value_of(\"client-cert\")) {
        let cert = X509::from_pem(&read_file(p2)?).map_err(ClientError::OpenSsl)?;
        let key = PKey::private_key_from_pem(&read_file(p1)?)
            .map_err(ClientError::OpenSsl)?;
        let builder = Pkcs12::builder();
        let pkcs12 = builder.build(\"foobar\", \"my-client\", &key, &cert)
            .map_err(ClientError::OpenSsl)?;
        let identity = reqwest::Identity::from_pkcs12_der(
            &pkcs12.to_der().map_err(ClientError::OpenSsl)?,
            \"foobar\"
        ).map_err(ClientError::Reqwest)?;
        client = client.identity(identity);
    }

    if let Some(timeout) = matches.value_of(\"timeout\") {
        let d = timeout.parse::<humantime::Duration>()?;
        client = client.timeout(d.into());
    }

    let is_verbose = matches.is_present(\"verbose\");
    let url = matches.value_of(\"url\").expect(\"required arg URL?\");
    Ok(WrappedClient {
        inner: client.build().map_err(ClientError::Reqwest)?,
        url: reqwest::Url::parse(url).map_err(ClientError::Url)?,
        verbose: is_verbose,
    })
}

async fn run_app() -> Result<(), Error> {
    let yml = load_yaml!(\"app.yaml\");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    let (sub_cmd, sub_matches) = matches.subcommand();

    let client = make_client(&matches)?;
    let response = self::cli::fetch_response(&client, &matches, sub_cmd, sub_matches).await?;

    let status = response.status();
    if client.verbose {
        println!(\"{}\", status);
    }

    let bytes = response
        .into_body()
        .concat2()
        .map_err(ClientError::Reqwest)
        .compat()
        .await?;

    let _ = std::io::copy(&mut &*bytes, &mut std::io::stdout());
    if !status.is_success() {
        Err(ClientError::Empty)?
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();
    if let Err(e) = run_app().await {
        println!(\"{}\", e);
    }
}
",
        Some(11157),
    );
}

#[test]
fn test_clap_yaml() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/cli/app.yaml"),
        "
name: test-k8s-cli
version: \"0.0.0\"

settings:
- SubcommandRequiredElseHelp

args:
    - ca-cert:
        long: ca-cert
        help: Path to CA certificate to be added to trust store.
        takes_value: true
    - client-cert:
        long: client-cert
        help: Path to certificate for TLS client verification.
        takes_value: true
        requires:
            - client-key
    - client-key:
        long: client-key
        help: Path to private key for TLS client verification.
        takes_value: true
        requires:
            - client-cert
    - url:
        long: url
        help: Base URL for your API.
        takes_value: true
        required: true
    - verbose:
        short: v
        long: verbose
        help: Enable verbose mode.
    - timeout:
        short: t
        long: timeout
        help: Set the request timeout.
        takes_value: true

subcommands:
",
        Some(0),
    );
}

#[test]
fn test_clap_yaml_cmd_description() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/cli/app.yaml"),
        "
  - delete-apps-v1-namespaced-deployment:
      about: \"delete a Deployment\"
      args:
        - payload:
            long: payload
            help: \"Path to payload (schema: DeleteOptions) or pass '-' for stdin\"
            takes_value: true
            required: true
        - dry-run:
            long: dry-run
            help: \"When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed\"
            takes_value: true
",
        None
    );
}
