#![feature(specialization)]
//! Core structs and traits for paperclip.

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod error;
pub mod im;
#[cfg(feature = "v2")]
pub mod v2;

pub use self::error::ValidationError;
