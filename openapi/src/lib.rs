//! Paperclip is a OpenAPI code generator for efficient type-safe
//! compile-time checked HTTP APIs in Rust.
//!
//! See the [website](https://paperclip.waffles.space) for detailed
//! documentation and examples.

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate paperclip_macros;
#[macro_use]
extern crate serde_derive;

mod error;
#[cfg(feature = "v2")]
pub mod v2;

pub use error::{PaperClipError, PaperClipResult};
pub use paperclip_macros::api_v2_schema;
