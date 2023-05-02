#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct ChildrenClient {
    configuration: Arc<configuration::Configuration>,
}

impl ChildrenClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for ChildrenClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Children: Clone + Send + Sync {
    
    
    
    
    async fn get_nexus_children(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::Child>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexus_children(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::Child>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Children` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Children: Clone + Send + Sync {
        
        
        
        
        async fn get_nexus_children(&self, nexus_id: &uuid::Uuid) -> Result<Vec<crate::models::Child>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_nexus_children(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<Vec<crate::models::Child>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Children for ChildrenClient {
    
    
    
    
    async fn get_nexus_children(&self, nexus_id: &uuid::Uuid) -> Result<Vec<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        Children::get_nexus_children(self, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, Error<crate::models::RestJsonError>> {
    
        Children::get_nexus_child(self, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, Error<crate::models::RestJsonError>> {
    
        Children::put_nexus_child(self, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Children::del_nexus_child(self, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_nexus_children(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<Vec<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        Children::get_node_nexus_children(self, node_id, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, Error<crate::models::RestJsonError>> {
    
        Children::get_node_nexus_child(self, node_id, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<crate::models::Child, Error<crate::models::RestJsonError>> {
    
        Children::put_node_nexus_child(self, node_id, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Children::del_node_nexus_child(self, node_id, nexus_id, child_id).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Children for ChildrenClient {
    
    
    
    
    async fn get_nexus_children(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::Child>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}/children", configuration.base_path, nexus_id=nexus_id.to_string());
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
            let local_var_content: Vec<crate::models::Child> =
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
    
    
    
    async fn get_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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
            let local_var_content: crate::models::Child =
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
    
    
    
    async fn put_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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
            let local_var_content: crate::models::Child =
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
    
    
    
    async fn del_nexus_child(&self, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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
    
    
    
    async fn get_node_nexus_children(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::Child>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/children", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            let local_var_content: Vec<crate::models::Child> =
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
    
    
    
    async fn get_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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
            let local_var_content: crate::models::Child =
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
    
    
    
    async fn put_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<crate::models::Child>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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
            let local_var_content: crate::models::Child =
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
    
    
    
    async fn del_node_nexus_child(&self, node_id: &str, nexus_id: &uuid::Uuid, child_id: &str) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/children/{child_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string(), child_id=crate::apis::urlencode(child_id));
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