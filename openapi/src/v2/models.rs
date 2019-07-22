//! Models used by OpenAPI v2.

use super::{im::ArcRwLock, Schema};
use crate::error::PaperClipError;
use failure::Error;
use regex::{Captures, Regex};

#[cfg(feature = "actix")]
use actix_http::http::Method;

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

lazy_static! {
    /// Regex that can be used for fetching templated path parameters.
    static ref PATH_TEMPLATE_REGEX: Regex = Regex::new(r"\{(.*?)\}").expect("path template regex");
}

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
    #[serde(other)]
    Other,
}

/// OpenAPI v2 spec.
pub type Api<S> = GenericApi<SchemaRepr<S>>;

/// OpenAPI v2 spec generic over schema.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GenericApi<S> {
    pub swagger: Version,
    pub definitions: BTreeMap<String, S>,
    pub paths: BTreeMap<String, OperationMap<S>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "basePath", skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
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

use crate as paperclip; // hack for proc macro

/// Default schema if your schema doesn't have any custom fields.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[api_v2_schema]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultSchema {}

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
        let mut shared_params = BTreeSet::new();
        for op in self.methods.values() {
            let params = op.parameters.iter().map(|p| p.name.clone()).collect();
            if shared_params.is_empty() {
                shared_params = params;
            } else {
                shared_params = &shared_params & &params;
            }
        }

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
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Parameter<S> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub in_: ParameterIn,
    pub name: String,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<S>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataTypeFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<S>,
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
    // FIXME: Switch to `mime::MediaType` (which adds serde support) once 0.4 is released.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<OperationProtocol>,
    // FIXME: Validate using `http::status::StatusCode::from_u16`
    pub responses: BTreeMap<String, Response<S>>,
    #[serde(default = "Vec::default", skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<Parameter<S>>,
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
                .unwrap_or_else(|| panic!("missing parameter {} in path {}", p, path));
            param.name = p.into();
            ":".into()
        });

        if params.peek().is_some() {
            panic!(
                "{} parameter(s) haven't been addressed by path {}",
                params.count(),
                path
            );
        }
    }
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

/// The protocol used for an operation.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationProtocol {
    Http,
    Https,
    Ws,
    Wss,
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

impl<S> Parameter<S> {
    /// Checks if this parameter is valid.
    pub fn check(&self, path: &str) -> Result<(), Error> {
        if self.in_ == ParameterIn::Body {
            if self.schema.is_none() {
                Err(PaperClipError::MissingSchemaForBodyParameter(
                    self.name.clone(),
                    path.into(),
                ))?
            }
        } else if self.data_type.is_none() {
            Err(PaperClipError::MissingParameterType(
                self.name.clone(),
                path.into(),
            ))?
        }

        Ok(())
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
