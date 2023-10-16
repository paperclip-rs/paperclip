#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Volumes {




    async fn get_volumes(Query(volume_id, max_entries, starting_token): Query<Option<uuid::Uuid>, isize, Option<isize>>) -> Result<crate::models::Volumes, RestError<crate::models::RestJsonError>>;



    async fn get_volume(Path(volume_id): Path<uuid::Uuid>) -> Result<crate::models::Volume, RestError<crate::models::RestJsonError>>;



    async fn put_volume(Path(volume_id): Path<uuid::Uuid>) -> Result<crate::models::Volume, RestError<crate::models::RestJsonError>>;



    async fn del_volume(Path(volume_id): Path<uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn put_volume_replica_count(Path(volume_id, replica_count): Path<uuid::Uuid, u8>) -> Result<crate::models::Volume, RestError<crate::models::RestJsonError>>;



    async fn put_volume_target(Path(volume_id): Path<uuid::Uuid>) -> Result<crate::models::Volume, RestError<crate::models::RestJsonError>>;



    async fn del_volume_target(Path(volume_id): Path<uuid::Uuid>, Query(force): Query<Option<bool>>) -> Result<crate::models::Volume, RestError<crate::models::RestJsonError>>;



    async fn del_volume_shutdown_targets(Path(volume_id): Path<uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn put_volume_share(Path(volume_id, protocol): Path<uuid::Uuid, String>, Query(frontend_host): Query<Option<String>>) -> Result<String, RestError<crate::models::RestJsonError>>;



    async fn del_share(Path(volume_id): Path<uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;


}

pub mod handlers;