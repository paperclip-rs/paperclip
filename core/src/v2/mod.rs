//! Core types and traits associated with the
//! [OpenAPI v2 specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md).

#[cfg(feature = "actix")]
mod actix;
mod extensions;
pub mod models;
#[cfg(feature = "codegen")]
mod resolver;
pub mod schema;

#[cfg(feature = "actix")]
pub use self::actix::{FutureWrapper, ResponderWrapper};

pub use self::models::{Api, DefaultSchema};
pub use self::schema::Schema;
pub use paperclip_macros::*;

#[cfg(feature = "codegen")]
use self::resolver::Resolver;
#[cfg(feature = "codegen")]
use crate::error::ValidationError;

#[cfg(feature = "codegen")]
impl<S: Schema + Default> Api<S> {
    /// Consumes this API schema, resolves the references and returns
    /// the resolved schema.
    ///
    /// This walks recursively, collects the referenced schema objects,
    /// substitutes the referenced IDs with the pointer to schema objects
    /// and returns the resolved object or an error if it encountered one.
    pub fn resolve(self) -> Result<Api<S>, ValidationError> {
        let mut resolver = Resolver::from((self.definitions, self.paths));
        resolver.resolve()?;
        Ok(Api {
            swagger: self.swagger,
            definitions: resolver.defs,
            paths: resolver.paths,
            base_path: self.base_path,
            host: self.host,
            schemes: self.schemes,
            consumes: self.consumes,
            produces: self.produces,
            coders: self.coders,
            support_crates: self.support_crates,
            spec_format: self.spec_format,
        })
    }
}
