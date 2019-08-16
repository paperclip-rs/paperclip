//! Utilities related to the [OpenAPI v2 specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md).
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
//! #[macro_use] extern crate paperclip;
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

use crate::error::PaperClipError;
use serde::Deserialize;

use std::io::{Read, Seek, SeekFrom};

#[cfg(feature = "codegen")]
pub use self::codegen::{DefaultEmitter, Emitter, EmitterState};
pub use paperclip_core::im;
pub use paperclip_core::v2::models::{self, Api, DefaultSchema};
pub use paperclip_core::v2::schema::{self, Schema};

/// Deserialize the schema from the given reader. Currently, this only supports
/// JSON and YAML formats.
pub fn from_reader<R, S>(mut reader: R) -> Result<Api<S>, PaperClipError>
where
    R: Read + Seek,
    for<'de> S: Deserialize<'de> + Schema,
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
