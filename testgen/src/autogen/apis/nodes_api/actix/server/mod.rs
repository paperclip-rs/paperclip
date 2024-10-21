#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Nodes {




    async fn get_nodes(Query(node_id): Query<Option<String>>) -> Result<Vec<crate::models::Node>, RestError<crate::models::RestJsonError>>;



    async fn get_node(Path(id): Path<String>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;



    async fn put_node_cordon(Path(id, label): Path<String, String>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;



    async fn delete_node_cordon(Path(id, label): Path<String, String>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;



    async fn put_node_drain(Path(id, label): Path<String, String>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;



    async fn put_node_label(Path(id, key, value): Path<String, String, String>, Query(overwrite): Query<Option<bool>>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;



    async fn delete_node_label(Path(id, key): Path<String, String>) -> Result<crate::models::Node, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;