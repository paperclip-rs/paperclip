#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait AppNodes {




    async fn get_app_nodes(Query(max_entries, starting_token): Query<isize, Option<isize>>) -> Result<crate::models::AppNodes, RestError<crate::models::RestJsonError>>;



    async fn get_app_node(Path(app_node_id): Path<String>) -> Result<crate::models::AppNode, RestError<crate::models::RestJsonError>>;



    async fn register_app_node(Path(app_node_id): Path<String>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn deregister_app_node(Path(app_node_id): Path<String>) -> Result<(), RestError<crate::models::RestJsonError>>;


}

pub mod handlers;