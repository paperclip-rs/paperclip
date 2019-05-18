use super::im::RcRefCell;
use crate as paperclip_openapi; // hack for proc macro

use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum Version {
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Integer,
    Number,
    String,
    Boolean,
    Array,
    Object,
}

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

#[derive(Debug, Deserialize)]
pub struct Api<S> {
    pub swagger: Version,
    pub definitions: BTreeMap<String, RcRefCell<S>>,
}

#[api_schema]
#[derive(Clone, Debug, Deserialize)]
pub struct DefaultSchema {}
