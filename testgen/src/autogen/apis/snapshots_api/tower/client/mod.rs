#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct SnapshotsClient {
    configuration: Arc<configuration::Configuration>,
}

impl SnapshotsClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for SnapshotsClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Snapshots: Clone + Send + Sync {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_volume_snapshots(&self, max_entries: isize, starting_token: Option<isize>, volume_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshots>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_volumes_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_volumes_snapshots(&self, snapshot_id: Option<&uuid::Uuid>, volume_id: Option<&uuid::Uuid>, max_entries: isize, starting_token: Option<isize>) -> Result<ResponseContent<crate::models::VolumeSnapshots>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Snapshots` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Snapshots: Clone + Send + Sync {
        
        
        
        
        async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_volume_snapshots(&self, max_entries: isize, starting_token: Option<isize>, volume_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshots, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_volumes_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_volumes_snapshots(&self, snapshot_id: Option<&uuid::Uuid>, volume_id: Option<&uuid::Uuid>, max_entries: isize, starting_token: Option<isize>) -> Result<crate::models::VolumeSnapshots, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Snapshots for SnapshotsClient {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    
        Snapshots::put_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Snapshots::del_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_volume_snapshots(&self, max_entries: isize, starting_token: Option<isize>, volume_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshots, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volume_snapshots(self, max_entries, starting_token, volume_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_volumes_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volumes_snapshot(self, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Snapshots::del_snapshot(self, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_volumes_snapshots(&self, snapshot_id: Option<&uuid::Uuid>, volume_id: Option<&uuid::Uuid>, max_entries: isize, starting_token: Option<isize>) -> Result<crate::models::VolumeSnapshots, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volumes_snapshots(self, snapshot_id, volume_id, max_entries, starting_token).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Snapshots for SnapshotsClient {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshot =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::PUT);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshot =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::DELETE);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: () =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn get_volume_snapshots(&self, max_entries: isize, starting_token: Option<isize>, volume_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshots>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots", configuration.base_path, volume_id=volume_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        let query_params: Option<String> = None;
        
        
        let query_params = match query_params {
            None => Some(format!("max_entries={}", max_entries.to_string())),
            Some(previous) => Some(format!("{previous}&max_entries={}", max_entries.to_string()))
        };
        
        
        
        
        
        let query_params = if let Some(local_var_str) = starting_token {
            match query_params {
                None => Some(format!("starting_token={}", local_var_str.to_string())),
                Some(previous) => Some(format!("{previous}&starting_token={}", local_var_str.to_string()))
            }
        } else {
            query_params
        };
        
        
        let local_var_uri_str = match query_params {
            None => local_var_uri_str,
            Some(params) => format!("{local_var_uri_str}?{params}")
        };
        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshots =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn get_volumes_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/snapshots/{snapshot_id}", configuration.base_path, snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshot =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/snapshots/{snapshot_id}", configuration.base_path, snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::DELETE);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: () =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn get_volumes_snapshots(&self, snapshot_id: Option<&uuid::Uuid>, volume_id: Option<&uuid::Uuid>, max_entries: isize, starting_token: Option<isize>) -> Result<ResponseContent<crate::models::VolumeSnapshots>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/snapshots", configuration.base_path);
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        let query_params: Option<String> = None;
        
        
        
        let query_params = if let Some(local_var_str) = snapshot_id {
            match query_params {
                None => Some(format!("snapshot_id={}", local_var_str.to_string())),
                Some(previous) => Some(format!("{previous}&snapshot_id={}", local_var_str.to_string()))
            }
        } else {
            query_params
        };
        
        
        
        
        let query_params = if let Some(local_var_str) = volume_id {
            match query_params {
                None => Some(format!("volume_id={}", local_var_str.to_string())),
                Some(previous) => Some(format!("{previous}&volume_id={}", local_var_str.to_string()))
            }
        } else {
            query_params
        };
        
        
        
        let query_params = match query_params {
            None => Some(format!("max_entries={}", max_entries.to_string())),
            Some(previous) => Some(format!("{previous}&max_entries={}", max_entries.to_string()))
        };
        
        
        
        
        
        let query_params = if let Some(local_var_str) = starting_token {
            match query_params {
                None => Some(format!("starting_token={}", local_var_str.to_string())),
                Some(previous) => Some(format!("{previous}&starting_token={}", local_var_str.to_string()))
            }
        } else {
            query_params
        };
        
        
        let local_var_uri_str = match query_params {
            None => local_var_uri_str,
            Some(params) => format!("{local_var_uri_str}?{params}")
        };
        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshots =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
}