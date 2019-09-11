#[macro_use]
extern crate lazy_static;
use paperclip::api_v2_schema;
#[macro_use]
extern crate serde_derive;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, Emitter, EmitterState},
    models::{Api, HttpMethod, Version},
};

use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref ROOT: String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref SCHEMA: Api<K8sSchema> = {
        let fd =
            File::open(ROOT.clone() + "/tests/k8s-v1.16.0-alpha.0-openapi-v2.json").expect("file?");
        let raw: Api<K8sSchema> = v2::from_reader(fd).expect("deserializing spec");
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
        meta.name = Some("test-k8s-cli".into());
        meta.version = Some("0.0.0".into());
        meta.authors = Some(vec!["Me <me@example.com>".into()]);
        meta.is_cli = true;
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
    let items = all_of.read().items.as_ref().unwrap().clone();
    assert_eq!(items.read().description, desc); // both point to same `JSONSchemaProps`
}

#[test]
fn test_resolved_schema() {
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
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
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
    pub all_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    #[serde(rename = \"anyOf\")]
    pub any_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub default: Option<String>,
    pub definitions: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
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
    pub external_docs: Option<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::external_documentation::ExternalDocumentation>,
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
    pub not: Option<Box<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub nullable: Option<bool>,
    #[serde(rename = \"oneOf\")]
    pub one_of: Option<Vec<crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub pattern: Option<String>,
    #[serde(rename = \"patternProperties\")]
    pub pattern_properties: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
    pub properties: Option<std::collections::BTreeMap<String, crate::codegen::io::k8s::apiextensions_apiserver::pkg::apis::apiextensions::v1beta1::json_schema_props::JsonSchemaProps>>,
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

pub mod client {
    use futures::{Future, future};
    use futures::stream::Stream;
    use parking_lot::Mutex;
    use reqwest::r#async::{Decoder, Response};
    use serde::de::DeserializeOwned;

    /// Common API errors.
    #[derive(Debug, Fail)]
    pub enum ApiError {
        #[fail(display = \"API request failed for path: {} (code: {})\", _0, _1)]
        Failure(String, reqwest::StatusCode, Mutex<Response>),
        #[fail(display = \"Unsupported media type in response: {}\", _0)]
        UnsupportedMediaType(String, Mutex<Response>),
        #[fail(display = \"An error has occurred while performing the API request: {}\", _0)]
        Reqwest(reqwest::Error),
        #[fail(display = \"Error en/decoding \\\"application/json\\\" data: {}\", _0)]
        ApplicationJson(serde_json::Error),
        #[fail(display = \"Error en/decoding \\\"application/yaml\\\" data: {}\", _0)]
        ApplicationYaml(serde_yaml::Error),
    }

    /// Represents an API client.
    pub trait ApiClient {
        /// Consumes a method and a relative path and produces a request builder for a single API call.
        fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder;

        /// Performs the HTTP request using the given `Request` object
        /// and returns a `Response` future.
        fn make_request(&self, req: reqwest::r#async::Request)
                       -> Box<dyn Future<Item=Response, Error=reqwest::Error> + Send>;
    }

    impl ApiClient for reqwest::r#async::Client {
        #[inline]
        fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder {
            let mut u = String::from(\"https://example.com/\");
            u.push_str(rel_path.trim_start_matches('/'));
            self.request(method, &u)
        }

        #[inline]
        fn make_request(&self, req: reqwest::r#async::Request)
                       -> Box<dyn Future<Item=Response, Error=reqwest::Error> + Send> {
            Box::new(self.execute(req)) as Box<_>
        }
    }

    /// A trait for indicating that the implementor can send an API call.
    pub trait Sendable {
        /// The output object from this API request.
        type Output: DeserializeOwned + Send + 'static;

        /// HTTP method used by this call.
        const METHOD: reqwest::Method;

        /// Relative URL for this API call formatted appropriately with parameter values.
        ///
        /// **NOTE:** This URL **must** begin with `/`.
        fn rel_path(&self) -> std::borrow::Cow<'static, str>;

        /// Modifier for this object. Builders override this method if they
        /// wish to add query parameters, set body, etc.
        fn modify(&self, req: reqwest::r#async::RequestBuilder) -> reqwest::r#async::RequestBuilder {
            req
        }

        /// Sends the request and returns a future for the response object.
        fn send(&self, client: &dyn ApiClient) -> Box<dyn Future<Item=Self::Output, Error=ApiError> + Send> {
            Box::new(self.send_raw(client).and_then(|mut resp| -> Box<dyn Future<Item=_, Error=ApiError> + Send> {
                let value = resp.headers().get(reqwest::header::CONTENT_TYPE);
                let body_concat = |resp: &mut Response| {
                    let body = std::mem::replace(resp.body_mut(), Decoder::empty());
                    body.concat2().map_err(ApiError::from)
                };

                if let Some(ty) = value.as_ref()
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<mime::MediaType>().ok())
                {
                    if media_types::M_0.matches(&ty) {
                        return Box::new(body_concat(&mut resp).and_then(|v| {
                            serde_json::from_slice(&v).map_err(ApiError::from)
                        })) as Box<_>
                    }
                    else if media_types::M_1.matches(&ty) {
                        return Box::new(body_concat(&mut resp).and_then(|v| {
                            serde_yaml::from_slice(&v).map_err(ApiError::from)
                        })) as Box<_>
                    }
                }

                let ty = value
                    .map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned())
                    .unwrap_or_default();
                Box::new(futures::future::err(ApiError::UnsupportedMediaType(ty, Mutex::new(resp)))) as Box<_>
            })) as Box<_>
        }

        /// Convenience method for returning a raw response after sending a request.
        fn send_raw(&self, client: &dyn ApiClient) -> Box<dyn Future<Item=Response, Error=ApiError> + Send> {
            let rel_path = self.rel_path();
            let builder = self.modify(client.request_builder(Self::METHOD, &rel_path));
            let req = match builder.build() {
                Ok(r) => r,
                Err(e) => return Box::new(future::err(ApiError::Reqwest(e))),
            };

            Box::new(client.make_request(req).map_err(ApiError::Reqwest).and_then(move |resp| {
                if resp.status().is_success() {
                    futures::future::ok(resp)
                } else {
                    futures::future::err(ApiError::Failure(rel_path.into_owned(), resp.status(), Mutex::new(resp)).into())
                }
            })) as Box<_>
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

    impl From<reqwest::Error> for ApiError {
        fn from(e: reqwest::Error) -> Self {
            ApiError::Reqwest(e)
        }
    }

    impl From<serde_json::Error> for ApiError {
        fn from(e: serde_json::Error) -> Self {
            ApiError::ApplicationJson(e)
        }
    }

    impl From<serde_yaml::Error> for ApiError {
        fn from(e: serde_yaml::Error) -> Self {
            ApiError::ApplicationYaml(e)
        }
    }
}

pub mod generics {
    include!(\"./generics.rs\");
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
        self.body.api_groups = Some(value.map(|value| value.into()).collect::<Vec<_>>());
        self
    }

    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding. Rules can either apply to API resources (such as \"pods\" or \"secrets\") or non-resource URL paths (such as \"/api\"),  but not both.
    #[inline]
    pub fn non_resource_ur_ls(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.non_resource_ur_ls = Some(value.map(|value| value.into()).collect::<Vec<_>>());
        self
    }

    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    #[inline]
    pub fn resource_names(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.resource_names = Some(value.map(|value| value.into()).collect::<Vec<_>>());
        self
    }

    /// Resources is a list of resources this rule applies to.  ResourceAll represents all resources.
    #[inline]
    pub fn resources(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.resources = Some(value.map(|value| value.into()).collect::<Vec<_>>());
        self
    }

    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds and AttributeRestrictions contained in this rule.  VerbAll represents all kinds.
    #[inline]
    pub fn verbs(mut self, value: impl Iterator<Item = impl Into<String>>) -> PolicyRuleBuilder<crate::codegen::generics::VerbsExists> {
        self.body.verbs = value.map(|value| value.into()).collect::<Vec<_>>();
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

impl crate::codegen::client::Sendable for DeleteOptionsDeleteBuilder59<crate::codegen::generics::NameExists, crate::codegen::generics::NamespaceExists> {
    type Output = crate::codegen::io::k8s::apimachinery::pkg::apis::meta::v1::status::Status;

    const METHOD: reqwest::Method = reqwest::Method::DELETE;

    fn rel_path(&self) -> std::borrow::Cow<'static, str> {
        format!(\"/apis/rbac.authorization.k8s.io/v1/namespaces/{namespace}/roles/{name}\", name=self.inner.param_name.as_ref().expect(\"missing parameter name?\"), namespace=self.inner.param_namespace.as_ref().expect(\"missing parameter namespace?\")).into()
    }

    fn modify(&self, req: reqwest::r#async::RequestBuilder) -> reqwest::r#async::RequestBuilder {
        req
        .json(&self.inner.body)
        .query(&[
            (\"dryRun\", self.inner.param_dry_run.as_ref().map(std::string::ToString::to_string)),
            (\"gracePeriodSeconds\", self.inner.param_grace_period_seconds.as_ref().map(std::string::ToString::to_string)),
            (\"orphanDependents\", self.inner.param_orphan_dependents.as_ref().map(std::string::ToString::to_string)),
            (\"propagationPolicy\", self.inner.param_propagation_policy.as_ref().map(std::string::ToString::to_string)),
            (\"pretty\", self.inner.param_pretty.as_ref().map(std::string::ToString::to_string))
        ])
    }
}
",
        Some(440139),
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
    pub fn get() -> ApiGroupListGetBuilder {
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
        self.body.groups = value.map(|value| value.into()).collect::<Vec<_>>();
        unsafe { std::mem::transmute(self) }
    }

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[inline]
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.body.kind = Some(value.into());
        self
    }
}

/// Builder created by [`ApiGroupList::get`](./struct.ApiGroupList.html#method.get) method for a `GET` operation associated with `ApiGroupList`.
#[derive(Debug, Clone)]
pub struct ApiGroupListGetBuilder;


impl crate::codegen::client::Sendable for ApiGroupListGetBuilder {
    type Output = ApiGroupList;

    const METHOD: reqwest::Method = reqwest::Method::GET;

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
        self.body.extra = Some(value.map(|(key, value)| (key, value.map(|value| value.into()).collect::<Vec<_>>())).collect::<std::collections::BTreeMap<_, _>>());
        self
    }

    /// Group information about the requesting user. See user.Info interface for details.
    #[inline]
    pub fn groups(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
        self.body.groups = Some(value.map(|value| value.into()).collect::<Vec<_>>());
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
        self.body.usages = Some(value.map(|value| value.into()).collect::<Vec<_>>());
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
fn test_cli_manifest() {
    let _ = &*CLI_CODEGEN;

    assert_file_contains_content_at(
        &(ROOT.clone() + "/tests/test_k8s/cli/Cargo.toml"),
        "
[[bin]]
name = \"test-k8s-cli\"
path = \"main.rs\"

[dependencies]
failure = \"0.1\"
futures = \"0.1\"
lazy_static = \"1.4\"
log = \"0.4\"
mime = { git = \"https://github.com/hyperium/mime\" }
parking_lot = \"0.8\"
reqwest = \"0.9\"
serde = \"1.0\"
serde_json = \"1.0\"
serde_yaml = \"0.8\"

clap = { version = \"2.33\", features = [\"yaml\"] }
env_logger = \"0.6\"
futures-preview = { version = \"0.3.0-alpha.16\", features = [\"compat\"], package = \"futures-preview\" }
humantime = \"1.2\"
openssl = { version = \"0.10\", features = [\"vendored\"] }
runtime = \"0.3.0-alpha.7\"
runtime-tokio = \"0.3.0-alpha.6\"
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
use clap::App;
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
    Api(self::client::ApiError),
    #[fail(display = \"Payload error: {}\", _0)]
    Json(serde_json::Error),
    #[fail(display = \"\")]
    Empty,
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut data = vec![];
    let mut fd = File::open(path.as_ref()).map_err(ClientError::Io)?;
    fd.read_to_end(&mut data).map_err(ClientError::Io)?;
    Ok(data)
}

struct WrappedClient {
    verbose: bool,
    inner: reqwest::r#async::Client,
    url: reqwest::Url,
}

impl ApiClient for WrappedClient {
    fn make_request(&self, req: reqwest::r#async::Request)
                   -> Box<dyn futures::Future<Item=reqwest::r#async::Response, Error=reqwest::Error> + Send>
    {
        if self.verbose {
            println!(\"{} {}\", req.method(), req.url());
        }

        self.inner.make_request(req)
    }

    fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder {
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

fn parse_args_and_fetch()
    -> Result<(WrappedClient, Box<dyn futures::Future<Item=reqwest::r#async::Response, Error=ApiError> + Send + 'static>), Error>
{
    let yml = load_yaml!(\"app.yaml\");
    let app = App::from_yaml(yml);
    let matches = app.get_matches();
    let (sub_cmd, sub_matches) = matches.subcommand();

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
    let client = WrappedClient {
        inner: client.build().map_err(ClientError::Reqwest)?,
        url: reqwest::Url::parse(url).map_err(ClientError::Url)?,
        verbose: is_verbose,
    };

    let f = self::cli::response_future(&client, &matches, sub_cmd, sub_matches)?;
    Ok((client, f))
}

async fn run_app() -> Result<(), Error> {
    let (client, f) = parse_args_and_fetch()?;
    let response = match f.map_err(ClientError::Api).compat().await {
        Ok(r) => r,
        Err(ClientError::Api(ApiError::Failure(_, _, r))) => r.into_inner(),
        Err(e) => return Err(e.into()),
    };

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

#[runtime::main(runtime_tokio::Tokio)]
async fn main() {
    env_logger::init();
    if let Err(e) = run_app().await {
        println!(\"{}\", e);
    }
}
",
        Some(6531),
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
