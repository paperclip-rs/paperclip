#![cfg(feature = "v3")]
//! Conversion traits and helps functions that help converting openapi v2 types to openapi v3.
//! For the OpenAPI v3 types the crate `openapiv3` is used.

use super::v2::models as v2;
mod models;

/// Convert this crates openapi v2 (`DefaultApiRaw`) to `openapiv3::OpenAPI`
pub fn openapiv2_to_v3(v2: v2::DefaultApiRaw) -> openapiv3::OpenAPI {
    openapiv3::OpenAPI::from(v2)
}
