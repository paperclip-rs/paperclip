#[macro_use] extern crate serde_derive;

#[allow(dead_code)]
mod io;

pub use io::k8s::api as k8s_api;
