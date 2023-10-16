#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct JsonGrpcClient {
    configuration: Arc<configuration::Configuration>,
}

impl JsonGrpcClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for JsonGrpcClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait JsonGrpc: Clone + Send + Sync {
    
    
    
    
    async fn put_node_jsongrpc(&self, node: &str, method: &str) -> Result<ResponseContent<serde_json::Value>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `JsonGrpc` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait JsonGrpc: Clone + Send + Sync {
        
        
        
        
        async fn put_node_jsongrpc(&self, node: &str, method: &str) -> Result<serde_json::Value, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::JsonGrpc for JsonGrpcClient {
    
    
    
    
    async fn put_node_jsongrpc(&self, node: &str, method: &str) -> Result<serde_json::Value, Error<crate::models::RestJsonError>> {
    
        JsonGrpc::put_node_jsongrpc(self, node, method).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl JsonGrpc for JsonGrpcClient {
    
    
    
    
    async fn put_node_jsongrpc(&self, node: &str, method: &str) -> Result<ResponseContent<serde_json::Value>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node}/jsongrpc/{method}", configuration.base_path, node=crate::apis::urlencode(node), method=crate::apis::urlencode(method));
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
            let local_var_content: serde_json::Value =
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