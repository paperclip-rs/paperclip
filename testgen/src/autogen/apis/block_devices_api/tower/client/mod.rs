#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct BlockDevicesClient {
    configuration: Arc<configuration::Configuration>,
}

impl BlockDevicesClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for BlockDevicesClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait BlockDevices: Clone + Send + Sync {
    
    
    
    
    async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<ResponseContent<Vec<crate::models::BlockDevice>>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `BlockDevices` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait BlockDevices: Clone + Send + Sync {
        
        
        
        
        async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<Vec<crate::models::BlockDevice>, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::BlockDevices for BlockDevicesClient {
    
    
    
    
    async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<Vec<crate::models::BlockDevice>, Error<crate::models::RestJsonError>> {
    
        BlockDevices::get_node_block_devices(self, all, node).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl BlockDevices for BlockDevicesClient {
    
    
    
    
    async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<ResponseContent<Vec<crate::models::BlockDevice>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node}/block_devices", configuration.base_path, node=crate::apis::urlencode(node));
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        let query_params: Option<String> = None;
        
        
        
        let query_params = if let Some(local_var_str) = all {
            match query_params {
                None => Some(format!("all={}", local_var_str.to_string())),
                Some(previous) => Some(format!("{previous}&all={}", local_var_str.to_string()))
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
            let local_var_content: Vec<crate::models::BlockDevice> =
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