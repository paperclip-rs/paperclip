use failure::Error;

use std::collections::BTreeMap;
use std::rc::Rc;

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
pub struct Api<Schema> {
    pub swagger: Version,
    pub definitions: BTreeMap<String, Schema>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Schema {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<DataTypeFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
}

impl Api<Schema> {
    /// Consumes this API schema, resolves the references and returns
    /// the resolved schema.
    ///
    /// This walks recursively, collects the referenced schema objects,
    /// substitutes the referenced IDs with the pointer to schema objects
    /// and returns the resolved object or an error if it encountered one.
    pub fn resolve(self) -> Result<Api<ResolvedSchema>, Error> {
        panic!("resolver");
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedSchema {
    pub reference: Option<Rc<ResolvedSchema>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub data_type: Option<DataType>,
    pub format: Option<DataTypeFormat>,
    pub properties: Option<BTreeMap<String, Rc<ResolvedSchema>>>,
    pub items: Option<Rc<ResolvedSchema>>,
}
