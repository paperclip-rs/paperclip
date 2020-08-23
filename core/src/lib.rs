#![cfg_attr(feature = "nightly", feature(specialization))]
//! Core structs and traits for paperclip.

#[cfg_attr(feature = "v2", macro_use)]
extern crate serde;

mod error;
pub mod im;
pub mod util;
#[cfg(feature = "v2")]
pub mod v2;

pub use self::error::ValidationError;
