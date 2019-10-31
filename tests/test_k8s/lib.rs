#[macro_use] extern crate failure;
#[macro_use] extern crate serde;

mod codegen {
    include!("./mod.rs");
}

pub use codegen::{io::k8s::*, miscellaneous};
pub use codegen::client::{ApiError, Sendable};
pub use codegen::util::{AsyncReadStream, ResponseStream};
