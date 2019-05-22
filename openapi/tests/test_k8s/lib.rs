#[macro_use] extern crate serde_derive;

mod io;

pub use io::k8s::api as k8s_api;
