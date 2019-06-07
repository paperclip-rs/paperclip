//! Models used by OpenAPI v2.

use super::{im::ArcRwLock, Schema};
use crate as paperclip_openapi;
use crate::error::PaperClipError; // hack for proc macro
use failure::Error;

use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

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
    pub definitions: BTreeMap<String, SchemaRepr<S>>,
    pub paths: BTreeMap<String, OperationMap<S>>,
}

/// Default schema if your schema doesn't have any custom fields.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[api_v2_schema]
#[derive(Clone, Debug, Deserialize)]
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
    pub schema: Option<SchemaRepr<S>>,
    #[serde(rename = "type")]
    pub data_type: Option<DataType>,
    pub format: Option<DataTypeFormat>,
    pub items: Option<SchemaRepr<S>>,
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
    pub schema: Option<SchemaRepr<S>>,
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

/// The protocol used for an operation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
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
