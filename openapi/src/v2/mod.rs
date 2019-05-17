pub mod models;

use self::models::{Api, ResolvedSchema, Schema};
use std::io::Read;

pub type ApiSchemaV2 = Api<Schema>;

pub type ResolvedApiSchemaV2 = Api<ResolvedSchema>;

pub fn from_json_reader<R>(reader: R) -> Result<ApiSchemaV2, serde_json::Error>
where
    R: Read,
{
    serde_json::from_reader(reader)
}
