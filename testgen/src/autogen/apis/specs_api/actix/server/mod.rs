#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Specs {




    async fn get_specs() -> Result<crate::models::Specs, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;