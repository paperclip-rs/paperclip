#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Pools {




    async fn get_node_pools(Path(id): Path<String>) -> Result<Vec<crate::models::Pool>, RestError<crate::models::RestJsonError>>;



    async fn get_node_pool(Path(node_id, pool_id): Path<String, String>) -> Result<crate::models::Pool, RestError<crate::models::RestJsonError>>;



    async fn put_node_pool(Path(node_id, pool_id): Path<String, String>) -> Result<crate::models::Pool, RestError<crate::models::RestJsonError>>;



    async fn del_node_pool(Path(node_id, pool_id): Path<String, String>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn get_pools() -> Result<Vec<crate::models::Pool>, RestError<crate::models::RestJsonError>>;



    async fn get_pool(Path(pool_id): Path<String>) -> Result<crate::models::Pool, RestError<crate::models::RestJsonError>>;



    async fn del_pool(Path(pool_id): Path<String>) -> Result<(), RestError<crate::models::RestJsonError>>;


}

pub mod handlers;