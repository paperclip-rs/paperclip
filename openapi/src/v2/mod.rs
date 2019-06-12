//! Types and traits related to the [OpenAPI v2 spec](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md).
//!
//! # Detailed example
//!
//! To parse your v2 spec, you begin with transforming the schema into a
//! Rust struct. If your schema doesn't have custom properties, then you
//! can use the `DefaultSchema`.
//!
//! ```rust,no_run
//! use paperclip::v2::{self, Api, DefaultSchema, models::Version};
//!
//! use std::fs::File;
//!
//! let mut fd = File::open("my_spec.yaml").unwrap(); // yaml or json
//! let api: Api<DefaultSchema> = v2::from_reader(&mut fd).unwrap();
//! assert_eq!(api.swagger, Version::V2);
//! ```
//!
//! On the other hand, if your schema does have custom properties which you'd
//! like to parse, then use the `#[api_v2_schema]` proc macro.
//!
//! For example, let's take the [Kubernetes API spec][kube-spec]
//! which uses some custom thingmabobs. Let's say we're only interested in the
//! `x-kubernetes-patch-strategy` field for now.
//!
//! [kube-spec]: https://github.com/kubernetes/kubernetes/tree/afd928b8bc81cea385eba4c94558373df7aeae75/api/openapi-spec
//!
//! ```rust,no_run
//! #[macro_use] extern crate paperclip_macros;
//! #[macro_use] extern crate serde_derive; // NOTE: We're using serde for decoding stuff.
//!
//! use paperclip::v2::{self, Api};
//!
//! use std::fs::File;
//!
//! #[derive(Debug, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! enum PatchStrategy {
//!     Merge,
//!     RetainKeys,
//!     #[serde(rename = "merge,retainKeys")]
//!     MergeAndRetain,
//!     #[serde(other)]
//!     Other,
//! }
//!
//! #[api_v2_schema]
//! #[derive(Debug, Deserialize)]
//! struct K8sSchema {
//!     #[serde(rename = "x-kubernetes-patch-strategy")]
//!     patch_strategy: Option<PatchStrategy>,
//! }
//!
//! // K8sSchema now implements `Schema` trait.
//! let mut fd = File::open("k8s_spec.yaml").unwrap();
//! let api: Api<K8sSchema> = v2::from_reader(&mut fd).unwrap();
//! ```
//!
//! Next stop is to resolve this raw schema i.e., walk through the nodes,
//! find `$ref` fields and assign references to the corresponding definitions.
//!
//! ```rust,no_run
//! # use paperclip::v2::{self, Api, DefaultSchema};
//! # let api: Api<DefaultSchema> = v2::from_reader(&mut std::io::Cursor::new(vec![])).unwrap();
//!
//! let resolved = api.resolve().unwrap();
//! ```
//!
//! Now, if `codegen` feature is enabled (it is by default), we can use the
//! emitter to emit the API into some path.
//!
//! ```rust,no_run
//! # use paperclip::v2::{self, Api, DefaultSchema};
//! # let api: Api<DefaultSchema> = v2::from_reader(&mut std::io::Cursor::new(vec![])).unwrap();
//! use paperclip::v2::{DefaultEmitter, EmitterState, Emitter};
//!
//! let mut state = EmitterState::default();
//! state.working_dir = "/path/to/my/crate".into();
//! let emitter = DefaultEmitter::from(state);
//! emitter.generate(&api).unwrap(); // generate code!
//! ```

#[cfg(feature = "codegen")]
pub mod codegen;
pub mod im;
pub mod models;
mod resolver;

use self::models::{DataType, DataTypeFormat, SchemaRepr};
use self::resolver::Resolver;
use crate::error::PaperClipError;
use failure::Error;
use serde::Deserialize;

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Seek, SeekFrom};

#[cfg(feature = "codegen")]
pub use self::codegen::{DefaultEmitter, Emitter, EmitterState};
pub use self::models::{Api, DefaultSchema};

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
/// This is only used for resolving the definitions.
///
/// **NOTE:** Don't implement this by yourself! Please use the `#[api_v2_schema]`
/// proc macro attribute instead.
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
    fn items(&self) -> Option<&SchemaRepr<Self>>;

    /// Mutable access to the `items` field, if it exists.
    fn items_mut(&mut self) -> Option<&mut SchemaRepr<Self>>;

    /// Value schema for maps (`additional_properties` field).
    fn additional_properties(&self) -> Option<&SchemaRepr<Self>>;

    /// Mutable access to `additional_properties` field, if it's a map.
    fn additional_properties_mut(&mut self) -> Option<&mut SchemaRepr<Self>>;

    /// Map of names and schema for properties, if it's an object (`properties` field)
    fn properties(&self) -> Option<&BTreeMap<String, SchemaRepr<Self>>>;

    /// Mutable access to `properties` field.
    fn properties_mut(&mut self) -> Option<&mut BTreeMap<String, SchemaRepr<Self>>>;

    /// Returns the required properties (if any) for this object.
    fn required_properties(&self) -> Option<&BTreeSet<String>>;

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
        let mut resolver = Resolver::from((self.definitions, self.paths));
        resolver.resolve()?;
        Ok(Api {
            swagger: self.swagger,
            definitions: resolver.defs,
            paths: resolver.paths,
        })
    }
}
