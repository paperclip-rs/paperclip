#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;

mod codegen {
    include!("./mod.rs");
}

pub use codegen::io::k8s::*;
pub use codegen::client::{ApiError, Sendable};
