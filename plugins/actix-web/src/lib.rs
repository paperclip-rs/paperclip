#![cfg(any(feature = "actix2", feature = "actix3", feature = "actix4"))]
#![allow(clippy::return_self_not_must_use)]

#[cfg(feature = "actix2")]
extern crate actix_web2 as actix_web;
#[cfg(feature = "actix3")]
extern crate actix_web3 as actix_web;
#[cfg(feature = "actix4")]
extern crate actix_web4 as actix_web;

#[cfg(any(feature = "swagger-ui", feature = "rapidoc"))]
use include_dir::{include_dir, Dir};

#[cfg(feature = "actix4")]
pub mod web;

#[cfg(not(feature = "actix4"))]
pub mod web3;
#[cfg(not(feature = "actix4"))]
pub use web3 as web;

#[cfg(feature = "actix4")]
pub mod app;

#[cfg(not(feature = "actix4"))]
pub mod app3;
#[cfg(not(feature = "actix4"))]
pub use app3 as app;

pub use self::{
    app::{App, OpenApiExt},
    web::{Resource, Route, Scope},
};
pub use paperclip_macros::{
    api_v2_errors, api_v2_errors_overlay, api_v2_operation, delete, get, head, patch, post, put,
    Apiv2Header, Apiv2Schema, Apiv2Security,
};

use paperclip_core::v2::models::{
    DefaultOperationRaw, DefaultPathItemRaw, DefaultSchemaRaw, HttpMethod, SecurityScheme,
};

use std::collections::BTreeMap;

#[cfg(feature = "swagger-ui")]
static SWAGGER_DIST: Dir = include_dir!("$CARGO_MANIFEST_DIR/swagger-ui/dist");
#[cfg(feature = "rapidoc")]
static RAPIDOC: Dir = include_dir!("$CARGO_MANIFEST_DIR/rapidoc");

/// Indicates that this thingmabob has a path and a bunch of definitions and operations.
pub trait Mountable {
    /// Where this thing gets mounted.
    fn path(&self) -> &str;

    /// Map of HTTP methods and the associated API operations.
    fn operations(&mut self) -> BTreeMap<HttpMethod, DefaultOperationRaw>;

    /// The definitions recorded by this object.
    fn definitions(&mut self) -> BTreeMap<String, DefaultSchemaRaw>;

    /// The security definitions recorded by this object.
    fn security_definitions(&mut self) -> BTreeMap<String, SecurityScheme>;

    /// Updates the given map of operations with operations tracked by this object.
    ///
    /// **NOTE:** Overriding implementations must ensure that the `PathItem`
    /// is normalized before updating the input map.
    fn update_operations(&mut self, map: &mut BTreeMap<String, DefaultPathItemRaw>) {
        let operations = self.operations();
        if !operations.is_empty() {
            let op_map = map
                .entry(self.path().into())
                .or_insert_with(Default::default);
            op_map.methods.extend(operations.into_iter());
        }
    }
}
