//! Models used by OpenAPI v2.

pub use super::extensions::{
    Coder, Coders, MediaRange, JSON_CODER, JSON_MIME, YAML_CODER, YAML_MIME,
};

use super::schema::Schema;
use crate::error::ValidationError;
use crate::im::ArcRwLock;
use lazy_static::lazy_static;
use paperclip_macros::api_v2_schema_struct;
use regex::{Captures, Regex};

#[cfg(feature = "actix")]
use actix_http::http::Method;

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

lazy_static! {
    /// Regex that can be used for fetching templated path parameters.
    static ref PATH_TEMPLATE_REGEX: Regex = Regex::new(r"\{(.*?)\}").expect("path template regex");
}

// Headers that have special meaning in OpenAPI. These cannot be used in header parameter.
// Ensure that they're all lowercase for case insensitive check.
const SPECIAL_HEADERS: &[&str] = &["content-type", "accept", "authorization"];

/// OpenAPI version.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
}

/// Supported data types.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Integer,
    Number,
    String,
    Boolean,
    Array,
    Object,
    File,
}

impl DataType {
    /// Checks if this is a primitive type.
    pub fn is_primitive(self) -> bool {
        match self {
            DataType::Integer | DataType::Number | DataType::String | DataType::Boolean => true,
            _ => false,
        }
    }
}

/// Supported data type formats.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataTypeFormat {
    Int32,
    Int64,
    Float,
    Double,
    Byte,
    Binary,
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
    Uuid,
    #[serde(other)]
    Other,
}

/// OpenAPI v2 spec.
pub type Api<S> = GenericApi<SchemaRepr<S>>;

/// OpenAPI v2 spec generic over schema.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenericApi<S> {
    pub swagger: Version,
    #[serde(default = "BTreeMap::new")]
    pub definitions: BTreeMap<String, S>,
    pub paths: BTreeMap<String, OperationMap<S>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "basePath", skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub consumes: BTreeSet<MediaRange>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub produces: BTreeSet<MediaRange>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub schemes: BTreeSet<OperationProtocol>,
    #[serde(
        default,
        rename = "x-rust-coders",
        skip_serializing_if = "<Coders as Deref>::Target::is_empty"
    )]
    pub coders: Coders,
    /// Additional crates that need to be added to the manifest.
    ///
    /// The key is the LHS of a dependency, which is the crate name.
    /// The value is the RHS of a crate's requirements as it would appear
    /// in the manifest. Note that the caller must add proper quoting
    /// whenever required.
    ///
    /// For example, in a JSON spec, the following are all valid:
    /// - `{"my_crate": "0.7"}`
    /// - `{"my_crate": "{ git = \"git://foo.bar/repo\" }"}`
    /// - `{"my_crate": "{ version = \"0.9\", features = [\"booya\"] }"}`
    #[serde(
        default,
        rename = "x-rust-dependencies",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub support_crates: BTreeMap<String, String>,
    /// This field is set manually, because we don't know the format in which
    /// the spec was provided.
    #[serde(skip)]
    pub spec_format: SpecFormat,
    pub info: Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecFormat {
    Json,
    Yaml,
}

impl SpecFormat {
    /// The en/decoder used for this format.
    pub fn coder(self) -> Arc<Coder> {
        match self {
            SpecFormat::Json => JSON_CODER.clone(),
            SpecFormat::Yaml => YAML_CODER.clone(),
        }
    }

    /// The mime for this format.
    pub fn mime(self) -> &'static MediaRange {
        match self {
            SpecFormat::Json => &*JSON_MIME,
            SpecFormat::Yaml => &*YAML_MIME,
        }
    }
}

impl<S> GenericApi<S> {
    /// Gets the parameters from the given path template and calls
    /// the given function with the parameter names.
    pub fn path_parameters_map(
        path: &str,
        mut f: impl FnMut(&str) -> Cow<'static, str>,
    ) -> Cow<'_, str> {
        PATH_TEMPLATE_REGEX.replace_all(path, |c: &Captures| f(&c[1]))
    }
}

/// `Either` from "either" crate. We can't use that crate because
/// we don't want the enum to be tagged during de/serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Get a mutable reference to the right variant (if it exists).
    pub fn right_mut(&mut self) -> Option<&mut R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }

    /// Get a mutable reference to the left variant (if it exists).
    pub fn left_mut(&mut self) -> Option<&mut L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }
}

impl<T> Either<T, Vec<T>> {
    /// Convenience method for getting either the value in the left
    /// or one from right, given that the right variant can contain
    /// more than one values in a vector.
    pub fn left_or_one_in_right(&self) -> Option<&T> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(v) if v.len() == 1 => Some(&v[0]),
            _ => None,
        }
    }
}

use crate as paperclip; // hack for proc macro

/// Default schema if your schema doesn't have any custom fields.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[api_v2_schema_struct]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultSchema;

/// Wrapper for schema. This uses `Arc<RwLock<S>>` for interior
/// mutability and differentiates raw schema from resolved schema
/// (i.e., the one where `$ref` references point to the actual schema).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaRepr<S> {
    Raw(ArcRwLock<S>),
    #[serde(skip)]
    Resolved {
        new: ArcRwLock<S>,
        old: ArcRwLock<S>,
    },
}

/// Info Object
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#infoObject
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Info {
    pub version: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
}

/// Contact Object
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#contactObject
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// License Object
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#licenseObject
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct License {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Path item.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#pathItemObject
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OperationMap<S> {
    #[serde(flatten, default = "BTreeMap::default")]
    pub methods: BTreeMap<HttpMethod, Operation<S>>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter<S>>,
}

impl<S> OperationMap<S> {
    /// Normalizes this operation map.
    /// - Collects and removes parameters shared across operations
    /// and adds them to the list global to this map.
    pub fn normalize(&mut self) {
        // We're using `Option<BTreeSet>` over `BTreeSet` because we need to
        // differentiate between the first operation that we use for initial
        // value of the set and  an operation that doesn't have any parameters.
        let mut shared_params = None;
        for op in self.methods.values() {
            let params = op
                .parameters
                .iter()
                .map(|p| p.name.clone())
                .collect::<BTreeSet<_>>();
            if let Some(p) = shared_params.take() {
                shared_params = Some(&p & &params); // set intersection
            } else {
                shared_params = Some(params);
            }
        }

        let shared_params = match shared_params {
            Some(p) => p,
            None => return,
        };

        // FIXME: A parameter defined at path level could be overridden at
        // the operation level with a different type. We shouldn't remove such
        // path-level parameters.
        for name in &shared_params {
            for op in self.methods.values_mut() {
                let idx = op
                    .parameters
                    .iter()
                    .position(|p| p.name == name.as_str())
                    .expect("collected parameter missing?");
                let p = op.parameters.swap_remove(idx);
                if self
                    .parameters
                    .iter()
                    .find(|p| p.name == name.as_str())
                    .is_none()
                {
                    self.parameters.push(p);
                }
            }
        }
    }
}

/// Request parameter.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameterObject
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter<S> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub in_: ParameterIn,
    pub name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<S>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataTypeFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Items>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<CollectionFormat>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_empty_value: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f32>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f32>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u32>,
    #[serde(default, rename = "uniqueItems", skip_serializing_if = "is_false")]
    pub unique_items: bool,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f32>,
    #[serde(default, rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,
}

/// Items object.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#itemsObject
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataTypeFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<CollectionFormat>,
    #[serde(default, rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f32>,
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f32>,
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<u32>,
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<u32>,
    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f32>,
}

impl<S> Parameter<SchemaRepr<S>>
where
    S: Schema,
{
    /// Checks the validity of this parameter using the relative URL
    /// path it's associated with.
    pub fn check(&self, path: &str) -> Result<(), ValidationError> {
        if self.in_ == ParameterIn::Body {
            // Body parameter must specify a schema.
            if self.schema.is_none() {
                return Err(ValidationError::MissingSchemaForBodyParameter(
                    self.name.clone(),
                    path.into(),
                ));
            }

            return Ok(());
        } else if self.in_ == ParameterIn::Header {
            // Some headers aren't allowed.
            let lower = self.name.to_lowercase();
            if SPECIAL_HEADERS.iter().any(|&h| lower == h) {
                return Err(ValidationError::InvalidHeader(
                    self.name.clone(),
                    path.into(),
                ));
            }
        }

        // Non-body parameters must be primitives or an array - they can't have objects.
        let mut is_invalid = false;
        match self.data_type {
            Some(dt) if dt.is_primitive() => (),
            Some(DataType::Array) => {
                let mut inner = self.items.as_ref();
                loop {
                    let dt = inner.as_ref().and_then(|s| s.data_type);
                    match dt {
                        Some(ty) if ty.is_primitive() => break,
                        Some(DataType::Array) => {
                            inner = inner
                                .as_ref()
                                .and_then(|s| s.items.as_ref().map(Deref::deref));
                        }
                        None => {
                            return Err(ValidationError::InvalidParameterType(
                                self.name.clone(),
                                path.into(),
                                dt,
                                self.in_,
                            ));
                        }
                        _ => {
                            is_invalid = true;
                            break;
                        }
                    }
                }
            }
            // If "file" is specified, then it must be `formData` parameter.
            Some(DataType::File) => {
                // FIXME: Check against `consumes` and `produces` fields.
                if self.in_ != ParameterIn::FormData {
                    is_invalid = true;
                }
            }
            _ => is_invalid = true,
        }

        if is_invalid {
            return Err(ValidationError::InvalidParameterType(
                self.name.clone(),
                path.into(),
                self.data_type,
                self.in_,
            ));
        }

        Ok(())
    }
}

/// The location of the parameter.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    FormData,
    Body,
}

/// Possible formats for array values in parameter.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum CollectionFormat {
    Csv,
    Ssv,
    Tsv,
    Pipes,
    Multi,
}

/// An operation.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#operationObject
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation<S> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    // *NOTE:* `consumes` and `produces` are optional, because
    // local media ranges can be used to override global media ranges
    // (including setting it to empty), so we cannot go for an empty set.
    pub consumes: Option<BTreeSet<MediaRange>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub produces: Option<BTreeSet<MediaRange>>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub schemes: BTreeSet<OperationProtocol>,
    // FIXME: Validate using `http::status::StatusCode::from_u16`
    pub responses: BTreeMap<String, Response<S>>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter<S>>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub deprecated: bool,
}

impl<S> Operation<S> {
    /// Overwrites the names of parameters in this operation using the
    /// given path template.
    ///
    /// # Panics
    ///
    /// This method will panic if there's a mismatch between the parameters
    /// in this operation and those in the given path template.
    pub fn set_parameter_names_from_path_template(&mut self, path: &str) {
        let mut params = self
            .parameters
            .iter_mut()
            .filter(|p| p.in_ == ParameterIn::Path)
            .peekable();
        Api::<()>::path_parameters_map(path, |p| {
            let mut param = params
                .next()
                .unwrap_or_else(|| panic!("missing parameter {:?} in path {:?}", p, path));
            param.name = p.into();
            ":".into()
        });

        if params.peek().is_some() {
            panic!(
                "{} parameter(s) haven't been addressed by path {:?}",
                params.count(),
                path
            );
        }
    }
}

/// The protocol used for an operation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum OperationProtocol {
    Http,
    Https,
    Ws,
    Wss,
}

/// HTTP response.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responseObject
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<S> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<S>,
}

/// The HTTP method used for an operation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Patch,
}

impl HttpMethod {
    /// Whether this method allows body in requests.
    pub fn allows_body(self) -> bool {
        match self {
            HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch => true,
            _ => false,
        }
    }
}

/* Common trait impls */

impl Default for SpecFormat {
    fn default() -> Self {
        SpecFormat::Json
    }
}

#[cfg(feature = "actix")]
impl From<&Method> for HttpMethod {
    fn from(method: &Method) -> HttpMethod {
        match method.as_str() {
            "PUT" => HttpMethod::Put,
            "POST" => HttpMethod::Post,
            "DELETE" => HttpMethod::Delete,
            "OPTIONS" => HttpMethod::Options,
            "HEAD" => HttpMethod::Head,
            "PATCH" => HttpMethod::Patch,
            _ => HttpMethod::Get,
        }
    }
}

impl<S> SchemaRepr<S>
where
    S: Schema,
{
    /// Fetch the description for this schema.
    pub fn get_description(&self) -> Option<String> {
        match *self {
            SchemaRepr::Raw(ref s) => s.read().description().map(String::from),
            // We don't want parameters/fields to describe the actual refrenced object.
            SchemaRepr::Resolved { ref old, .. } => old.read().description().map(String::from),
        }
    }
}

impl<S> Deref for SchemaRepr<S> {
    type Target = ArcRwLock<S>;

    fn deref(&self) -> &Self::Target {
        match *self {
            SchemaRepr::Raw(ref s) => s,
            SchemaRepr::Resolved { ref new, .. } => new,
        }
    }
}

impl<S> DerefMut for SchemaRepr<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match *self {
            SchemaRepr::Raw(ref mut s) => s,
            SchemaRepr::Resolved { ref mut new, .. } => new,
        }
    }
}

impl<S> From<S> for SchemaRepr<S> {
    fn from(t: S) -> Self {
        SchemaRepr::Raw(t.into())
    }
}

impl<S> Clone for SchemaRepr<S> {
    fn clone(&self) -> Self {
        match *self {
            SchemaRepr::Raw(ref s) => SchemaRepr::Raw(s.clone()),
            SchemaRepr::Resolved { ref new, ref old } => SchemaRepr::Resolved {
                new: new.clone(),
                old: old.clone(),
            },
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::V2
    }
}

impl Default for CollectionFormat {
    fn default() -> Self {
        CollectionFormat::Csv
    }
}

/// **NOTE:** This is just a stub. This is usually set explicitly.
impl Default for ParameterIn {
    fn default() -> Self {
        ParameterIn::Body
    }
}

/* Serde helpers */

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(val: &bool) -> bool {
    !*val
}
