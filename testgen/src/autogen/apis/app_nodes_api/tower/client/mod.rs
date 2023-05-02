#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct AppNodesClient {
    configuration: Arc<configuration::Configuration>,
}

impl AppNodesClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for AppNodesClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait AppNodes: Clone + Send + Sync {
    
    
    
    
    async fn get_app_nodes(&self, max_entries: isize, starting_token: Option<isize>) -> Result<ResponseContent<crate::models::AppNodes>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_app_node(&self, app_node_id: &str) -> Result<ResponseContent<crate::models::AppNode>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn register_app_node(&self, app_node_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn deregister_app_node(&self, app_node_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `AppNodes` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait AppNodes: Clone + Send + Sync {
        
        
        
        
        async fn get_app_nodes(&self, max_entries: isize, starting_token: Option<isize>) -> Result<crate::models::AppNodes, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_app_node(&self, app_node_id: &str) -> Result<crate::models::AppNode, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn register_app_node(&self, app_node_id: &str) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn deregister_app_node(&self, app_node_id: &str) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::AppNodes for AppNodesClient {
    
    
    
    
    async fn get_app_nodes(&self, max_entries: isize, starting_token: Option<isize>) -> Result<crate::models::AppNodes, Error<crate::models::RestJsonError>> {
    
        AppNodes::get_app_nodes(self, max_entries, starting_token).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_app_node(&self, app_node_id: &str) -> Result<crate::models::AppNode, Error<crate::models::RestJsonError>> {
    
        AppNodes::get_app_node(self, app_node_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn register_app_node(&self, app_node_id: &str) -> Result<(), Error<crate::models::RestJsonError>> {
    
        AppNodes::register_app_node(self, app_node_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn deregister_app_node(&self, app_node_id: &str) -> Result<(), Error<crate::models::RestJsonError>> {
    
        AppNodes::deregister_app_node(self, app_node_id).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl AppNodes for AppNodesClient {
    
    
    
    
    async fn get_app_nodes(&self, max_entries: isize, starting_token: Option<isize>) -> Result<ResponseContent<crate::models::AppNodes>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/app-nodes", configuration.base_path);
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
            let local_var_content: crate::models::AppNodes =
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
    
    
    
    async fn get_app_node(&self, app_node_id: &str) -> Result<ResponseContent<crate::models::AppNode>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/app-nodes/{app_node_id}", configuration.base_path, app_node_id=crate::apis::urlencode(app_node_id));
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
            let local_var_content: crate::models::AppNode =
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
    
    
    
    async fn register_app_node(&self, app_node_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/app-nodes/{app_node_id}", configuration.base_path, app_node_id=crate::apis::urlencode(app_node_id));
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
    
    
    
    async fn deregister_app_node(&self, app_node_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/app-nodes/{app_node_id}", configuration.base_path, app_node_id=crate::apis::urlencode(app_node_id));
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
    
    
}