#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::apis::actix_server::{Body, Path, Query, RestError};
use actix_web::web::Json;

#[async_trait::async_trait]
pub trait Watches {




    async fn get_watch_volume(Path(volume_id): Path<uuid::Uuid>) -> Result<Vec<crate::models::RestWatch>, RestError<crate::models::RestJsonError>>;



    async fn put_watch_volume(Path(volume_id): Path<uuid::Uuid>, Query(callback): Query<String>) -> Result<(), RestError<crate::models::RestJsonError>>;



    async fn del_watch_volume(Path(volume_id): Path<uuid::Uuid>, Query(callback): Query<String>) -> Result<(), RestError<crate::models::RestJsonError>>;


}

pub mod handlers;