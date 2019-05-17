#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

pub mod error;
#[cfg(feature = "v2")]
pub mod v2;
