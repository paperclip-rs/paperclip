//! Models used by OpenAPI v2.

use super::im::ArcRwLock;
use crate as paperclip_openapi;
use crate::error::PaperClipError; // hack for proc macro
use failure::Error;

use std::collections::BTreeMap;
use std::fmt::{self, Display};

/// OpenAPI version.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
}

/// Supported data types.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged, rename_all = "lowercase")]
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
    Other(String),
}

/// OpenAPI v2 spec.
#[derive(Clone, Debug, Deserialize)]
pub struct Api<S> {
    pub swagger: Version,
    pub definitions: BTreeMap<String, ArcRwLock<S>>,
    pub paths: BTreeMap<String, OperationMap<S>>,
}

/// Default schema if your schema doesn't have any custom fields.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[api_v2_schema]
#[derive(Clone, Debug, Deserialize)]
pub struct DefaultSchema {}

/// Path item.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#pathItemObject
#[derive(Clone, Debug, Deserialize)]
pub struct OperationMap<S> {
    #[serde(flatten)]
    pub methods: BTreeMap<HttpMethod, Operation<S>>,
    pub parameters: Option<Vec<Parameter<S>>>,
}

/// Request parameter.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameterObject
#[derive(Clone, Debug, Deserialize)]
pub struct Parameter<S> {
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub in_: ParameterIn,
    pub name: String,
    #[serde(default)]
    pub required: bool,
    pub schema: Option<ArcRwLock<S>>,
    #[serde(rename = "type")]
    pub data_type: Option<DataType>,
    pub format: Option<DataTypeFormat>,
    pub items: Option<ArcRwLock<S>>,
}

/// The location of the parameter.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
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
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation<S> {
    pub operation_id: Option<String>,
    pub description: Option<String>,
    // FIXME: Switch to `mime::MediaType` (which adds serde support) once 0.4 is released.
    #[serde(default)]
    pub consumes: Vec<String>,
    #[serde(default)]
    pub produces: Vec<String>,
    pub schemes: Vec<OperationProtocol>,
    // FIXME: Validate using `http::status::StatusCode::from_u16`
    pub responses: BTreeMap<String, Response<S>>,
    pub parameters: Option<Vec<Parameter<S>>>,
}

/// HTTP response.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responseObject
#[derive(Clone, Debug, Deserialize)]
pub struct Response<S> {
    pub description: Option<String>,
    pub schema: Option<ArcRwLock<S>>,
}

/// The HTTP method used for an operation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Ord, PartialOrd)]
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

impl Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The protocol used for an operation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationProtocol {
    Http,
    Https,
    Ws,
    Wss,
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
