#[cfg(feature = "codegen")]
pub mod codegen;
pub mod im;
pub mod models;
mod resolver;

use self::im::RcRefCell;
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
        return Ok(serde_json::from_reader(reader)?);
    }

    Ok(serde_yaml::from_reader(reader)?)
}

pub trait Schema: Sized {
    fn description(&self) -> Option<&str>;

    fn reference(&self) -> Option<&str>;

    fn data_type(&self) -> Option<DataType>;

    fn format(&self) -> Option<&DataTypeFormat>;

    fn items(&self) -> Option<&RcRefCell<Self>>;

    fn items_mut(&mut self) -> Option<&mut RcRefCell<Self>>;

    fn properties(&self) -> Option<&BTreeMap<String, RcRefCell<Self>>>;

    fn properties_mut(&mut self) -> Option<&mut BTreeMap<String, RcRefCell<Self>>>;
}

impl<S: Schema> Api<S> {
    /// Consumes this API schema, resolves the references and returns
    /// the resolved schema.
    ///
    /// This walks recursively, collects the referenced schema objects,
    /// substitutes the referenced IDs with the pointer to schema objects
    /// and returns the resolved object or an error if it encountered one.
    pub fn resolve(self) -> Result<Api<S>, Error> {
        let mut resolver = Resolver {
            defs: self.definitions,
        };
        resolver.resolve()?;
        Ok(Api {
            swagger: self.swagger,
            definitions: resolver.defs,
        })
    }
}
