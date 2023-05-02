#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Nexuses {




    async fn get_nexuses() -> Result<Vec<crate::models::Nexus>, RestError<crate::models::RestJsonError>>;



    async fn get_nexus(Path(nexus_id): Path<uuid::Uuid>) -> Result<crate::models::Nexus, RestError<crate::models::RestJsonError>>;



    async fn del_nexus(Path(nexus_id): Path<uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn get_node_nexuses(Path(id): Path<String>) -> Result<Vec<crate::models::Nexus>, RestError<crate::models::RestJsonError>>;



    async fn get_node_nexus(Path(node_id, nexus_id): Path<String, uuid::Uuid>) -> Result<crate::models::Nexus, RestError<crate::models::RestJsonError>>;



    async fn put_node_nexus(Path(node_id, nexus_id): Path<String, uuid::Uuid>) -> Result<crate::models::Nexus, RestError<crate::models::RestJsonError>>;



    async fn del_node_nexus(Path(node_id, nexus_id): Path<String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn del_node_nexus_share(Path(node_id, nexus_id): Path<String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn put_node_nexus_share(Path(node_id, nexus_id, protocol): Path<String, uuid::Uuid, String>) -> Result<String, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;