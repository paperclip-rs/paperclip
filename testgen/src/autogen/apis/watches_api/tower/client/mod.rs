#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct WatchesClient {
    configuration: Arc<configuration::Configuration>,
}

impl WatchesClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for WatchesClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Watches: Clone + Send + Sync {
    
    
    
    
    async fn get_watch_volume(&self, volume_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::RestWatch>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Watches` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Watches: Clone + Send + Sync {
        
        
        
        
        async fn get_watch_volume(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::RestWatch>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Watches for WatchesClient {
    
    
    
    
    async fn get_watch_volume(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::RestWatch>, Error<crate::models::RestJsonError>> {
    
        Watches::get_watch_volume(self, volume_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Watches::put_watch_volume(self, callback, volume_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Watches::del_watch_volume(self, callback, volume_id).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Watches for WatchesClient {
    
    
    
    
    async fn get_watch_volume(&self, volume_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::RestWatch>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/watches/volumes/{volume_id}", configuration.base_path, volume_id=volume_id.to_string());
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
            let local_var_content: Vec<crate::models::RestWatch> =
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
    
    
    
    async fn put_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/watches/volumes/{volume_id}", configuration.base_path, volume_id=volume_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::PUT);

        
        let query_params: Option<String> = None;
        
        
        let query_params = match query_params {
            None => Some(format!("callback={}", callback.to_string())),
            Some(previous) => Some(format!("{previous}&callback={}", callback.to_string()))
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
    
    
    
    async fn del_watch_volume(&self, callback: &str, volume_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/watches/volumes/{volume_id}", configuration.base_path, volume_id=volume_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::DELETE);

        
        let query_params: Option<String> = None;
        
        
        let query_params = match query_params {
            None => Some(format!("callback={}", callback.to_string())),
            Some(previous) => Some(format!("{previous}&callback={}", callback.to_string()))
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
    
    
}