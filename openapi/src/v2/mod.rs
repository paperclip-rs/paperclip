//! Types and traits related to the [OpenAPI v2 spec](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md).

#[cfg(feature = "codegen")]
pub mod codegen;
pub mod im;
pub mod models;
mod resolver;

use self::im::ArcRwLock;
use self::models::{Api, DataType, DataTypeFormat};
use self::resolver::Resolver;
use crate::error::PaperClipError;
use failure::Error;
use serde::Deserialize;

use std::collections::BTreeMap;
use std::io::{Read, Seek, SeekFrom};

/// Deserialize the schema from the given reader. Currently, this only supports
/// JSON and YAML formats.
pub fn from_reader<R, S>(mut reader: R) -> Result<Api<S>, PaperClipError>
where
    R: Read + Seek,
    for<'de> S: Deserialize<'de>,
    S: Schema,
{
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    reader.seek(SeekFrom::Start(0))?;

    if buf[0] == b'{' {
        // FIXME: Support whitespaces
        return Ok(serde_json::from_reader(reader)?);
    }

    Ok(serde_yaml::from_reader(reader)?)
}

/// Interface for the [`Schema`](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject) object.
///
/// This is only used for resolving the definitions. Please use the `#[api_v2_schema]`
/// proc macro attribute instead of implementing this trait by yourself.
pub trait Schema: Sized {
    /// Description for this schema, if any (`description` field).
    fn description(&self) -> Option<&str>;

    /// Reference to some other schema, if any (`$ref` field).
    fn reference(&self) -> Option<&str>;

    /// Data type of this schema, if any (`type` field).
    fn data_type(&self) -> Option<DataType>;

    /// Data type format used by this schema, if any (`format` field).
    fn format(&self) -> Option<&DataTypeFormat>;

    /// Schema for array definitions, if any (`items` field).
    fn items(&self) -> Option<&ArcRwLock<Self>>;

    /// Mutable access to the `items` field, if it exists.
    fn items_mut(&mut self) -> Option<&mut ArcRwLock<Self>>;

    /// Value schema for maps (`additional_properties` field).
    fn additional_properties(&self) -> Option<&ArcRwLock<Self>>;

    /// Mutable access to `additional_properties` field, if it's a map.
    fn additional_properties_mut(&mut self) -> Option<&mut ArcRwLock<Self>>;

    /// Map of names and schema for properties, if it's an object (`properties` field)
    fn properties(&self) -> Option<&BTreeMap<String, ArcRwLock<Self>>>;

    /// Mutable access to `properties` field.
    fn properties_mut(&mut self) -> Option<&mut BTreeMap<String, ArcRwLock<Self>>>;

    /// Set whether this definition is cyclic. This is done by the resolver.
    fn set_cyclic(&mut self, cyclic: bool);

    /// Returns whether this definition is cyclic.
    ///
    /// **NOTE:** This is not part of the schema object, but it's
    /// set by the resolver using `set_cyclic` for codegen.
    fn is_cyclic(&self) -> bool;

    /// Name of this schema, if any.
    ///
    /// **NOTE:** This is not part of the schema object, but it's
    /// set by the resolver using `set_name` for codegen.
    fn name(&self) -> Option<&str>;

    /// Sets the name for this schema. This is done by the resolver.
    fn set_name(&mut self, name: &str);
}

impl<S: Schema> Api<S> {
    /// Consumes this API schema, resolves the references and returns
    /// the resolved schema.
    ///
    /// This walks recursively, collects the referenced schema objects,
    /// substitutes the referenced IDs with the pointer to schema objects
    /// and returns the resolved object or an error if it encountered one.
    pub fn resolve(self) -> Result<Api<S>, Error> {
        let mut resolver = Resolver::from(self.definitions);
        resolver.resolve()?;
        Ok(Api {
            swagger: self.swagger,
            definitions: resolver.defs,
        })
    }
}
