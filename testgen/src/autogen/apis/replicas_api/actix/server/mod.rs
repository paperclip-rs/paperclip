#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Replicas {




    async fn get_node_replicas(Path(id): Path<String>) -> Result<Vec<crate::models::Replica>, RestError<crate::models::RestJsonError>>;



    async fn get_node_pool_replicas(Path(node_id, pool_id): Path<String, String>) -> Result<Vec<crate::models::Replica>, RestError<crate::models::RestJsonError>>;



    async fn get_node_pool_replica(Path(node_id, pool_id, replica_id): Path<String, String, uuid::Uuid>) -> Result<crate::models::Replica, RestError<crate::models::RestJsonError>>;



    async fn put_node_pool_replica(Path(node_id, pool_id, replica_id): Path<String, String, uuid::Uuid>) -> Result<crate::models::Replica, RestError<crate::models::RestJsonError>>;



    async fn del_node_pool_replica(Path(node_id, pool_id, replica_id): Path<String, String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn del_node_pool_replica_share(Path(node_id, pool_id, replica_id): Path<String, String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn put_node_pool_replica_share(Path(node_id, pool_id, replica_id): Path<String, String, uuid::Uuid>, Query(allowed_hosts): Query<Option<Vec<String>>>) -> Result<String, RestError<crate::models::RestJsonError>>;



    async fn put_pool_replica(Path(pool_id, replica_id): Path<String, uuid::Uuid>) -> Result<crate::models::Replica, RestError<crate::models::RestJsonError>>;



    async fn del_pool_replica(Path(pool_id, replica_id): Path<String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn del_pool_replica_share(Path(pool_id, replica_id): Path<String, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn put_pool_replica_share(Path(pool_id, replica_id): Path<String, uuid::Uuid>, Query(allowed_hosts): Query<Option<Vec<String>>>) -> Result<String, RestError<crate::models::RestJsonError>>;



    async fn get_replicas() -> Result<Vec<crate::models::Replica>, RestError<crate::models::RestJsonError>>;



    async fn get_replica(Path(id): Path<uuid::Uuid>) -> Result<crate::models::Replica, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;