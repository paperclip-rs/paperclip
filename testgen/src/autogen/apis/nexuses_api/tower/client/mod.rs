#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct NexusesClient {
    configuration: Arc<configuration::Configuration>,
}

impl NexusesClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for NexusesClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Nexuses: Clone + Send + Sync {
    
    
    
    
    async fn get_nexuses(&self, ) -> Result<ResponseContent<Vec<crate::models::Nexus>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexuses(&self, id: &str) -> Result<ResponseContent<Vec<crate::models::Nexus>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Nexuses` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Nexuses: Clone + Send + Sync {
        
        
        
        
        async fn get_nexuses(&self, ) -> Result<Vec<crate::models::Nexus>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_nexuses(&self, id: &str) -> Result<Vec<crate::models::Nexus>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<String, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Nexuses for NexusesClient {
    
    
    
    
    async fn get_nexuses(&self, ) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    
        Nexuses::get_nexuses(self, ).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    
        Nexuses::get_nexus(self, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Nexuses::del_nexus(self, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_nexuses(&self, id: &str) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    
        Nexuses::get_node_nexuses(self, id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    
        Nexuses::get_node_nexus(self, node_id, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    
        Nexuses::put_node_nexus(self, node_id, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Nexuses::del_node_nexus(self, node_id, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Nexuses::del_node_nexus_share(self, node_id, nexus_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<String, Error<crate::models::RestJsonError>> {
    
        Nexuses::put_node_nexus_share(self, node_id, nexus_id, protocol).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Nexuses for NexusesClient {
    
    
    
    
    async fn get_nexuses(&self, ) -> Result<ResponseContent<Vec<crate::models::Nexus>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses", configuration.base_path);
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
            let local_var_content: Vec<crate::models::Nexus> =
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
    
    
    
    async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}", configuration.base_path, nexus_id=nexus_id.to_string());
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
            let local_var_content: crate::models::Nexus =
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
    
    
    
    async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}", configuration.base_path, nexus_id=nexus_id.to_string());
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
    
    
    
    async fn get_node_nexuses(&self, id: &str) -> Result<ResponseContent<Vec<crate::models::Nexus>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{id}/nexuses", configuration.base_path, id=crate::apis::urlencode(id));
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
            let local_var_content: Vec<crate::models::Nexus> =
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
    
    
    
    async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            let local_var_content: crate::models::Nexus =
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
    
    
    
    async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            let local_var_content: crate::models::Nexus =
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
    
    
    
    async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
    
    
    
    async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/share", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
    
    
    
    async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/share/{protocol}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string(), protocol=crate::apis::urlencode(protocol));
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
            let local_var_content: String =
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