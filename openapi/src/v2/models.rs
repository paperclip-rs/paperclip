//! Models used by OpenAPI v2 spec.

use super::im::ArcRwLock;
use crate as paperclip_openapi; // hack for proc macro

use std::collections::BTreeMap;

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
#[derive(Debug, Deserialize)]
pub struct Api<S> {
    pub swagger: Version,
    pub definitions: BTreeMap<String, ArcRwLock<S>>,
}

/// Default schema if your schema doesn't have any custom fields.
#[api_v2_schema]
#[derive(Clone, Debug, Deserialize)]
pub struct DefaultSchema {}
