pub mod models;
mod resolver;

use self::models::{ApiSchema, RawDefinitions};
use crate::error::PaperClipError;

use std::io::{Read, Seek, SeekFrom};

/// Deserialize the schema from the given reader. Currently, this only supports
/// JSON and YAML formats.
pub fn from_reader<R>(mut reader: R) -> Result<ApiSchema<RawDefinitions>, PaperClipError>
where
    R: Read + Seek,
{
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    reader.seek(SeekFrom::Start(0))?;

    if buf[0] == b'{' {
        return Ok(serde_json::from_reader(reader)?);
    }

    Ok(serde_yaml::from_reader(reader)?)
}
