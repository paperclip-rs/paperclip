#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Snapshots {




    async fn get_volume_snapshot(Path(volume_id, snapshot_id): Path<uuid::Uuid, uuid::Uuid>) -> Result<crate::models::VolumeSnapshot, RestError<crate::models::RestJsonError>>;



    async fn put_volume_snapshot(Path(volume_id, snapshot_id): Path<uuid::Uuid, uuid::Uuid>) -> Result<crate::models::VolumeSnapshot, RestError<crate::models::RestJsonError>>;



    async fn del_volume_snapshot(Path(volume_id, snapshot_id): Path<uuid::Uuid, uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn get_volume_snapshots(Path(volume_id): Path<uuid::Uuid>, Query(max_entries, starting_token): Query<isize, Option<isize>>) -> Result<crate::models::VolumeSnapshots, RestError<crate::models::RestJsonError>>;



    async fn get_volumes_snapshot(Path(snapshot_id): Path<uuid::Uuid>) -> Result<crate::models::VolumeSnapshot, RestError<crate::models::RestJsonError>>;



    async fn del_snapshot(Path(snapshot_id): Path<uuid::Uuid>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn get_volumes_snapshots(Query(snapshot_id, volume_id, max_entries, starting_token): Query<Option<uuid::Uuid>, Option<uuid::Uuid>, isize, Option<isize>>) -> Result<crate::models::VolumeSnapshots, RestError<crate::models::RestJsonError>>;


}

pub mod handlers;