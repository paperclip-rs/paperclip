#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait BlockDevices {




    async fn get_node_block_devices(Path(node): Path<String>, Query(all): Query<Option<bool>>) -> Result<Vec<crate::models::BlockDevice>, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;