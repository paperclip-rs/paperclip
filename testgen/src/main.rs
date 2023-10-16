fn main() {
    println!("Hello, world!");
}

mod autogen;
use autogen::*;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
