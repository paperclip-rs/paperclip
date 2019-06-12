#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

mod io;

pub use io::k8s::*;
pub use io::client::{ApiError, Sendable};
