#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct ReplicasClient {
    configuration: Arc<configuration::Configuration>,
}

impl ReplicasClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for ReplicasClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Replicas: Clone + Send + Sync {
    
    
    
    
    async fn get_node_replicas(&self, id: &str) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_pool_replicas(&self, node_id: &str, pool_id: &str) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_pool_replica_share(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_pool_replica_share(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_replicas(&self, ) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_replica(&self, id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Replicas` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Replicas: Clone + Send + Sync {
        
        
        
        
        async fn get_node_replicas(&self, id: &str) -> Result<Vec<crate::models::Replica>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_pool_replicas(&self, node_id: &str, pool_id: &str) -> Result<Vec<crate::models::Replica>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_node_pool_replica_share(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_node_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<String, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_pool_replica_share(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, pool_id: &str, replica_id: &uuid::Uuid) -> Result<String, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_replicas(&self, ) -> Result<Vec<crate::models::Replica>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_replica(&self, id: &uuid::Uuid) -> Result<crate::models::Replica, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Replicas for ReplicasClient {
    
    
    
    
    async fn get_node_replicas(&self, id: &str) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        Replicas::get_node_replicas(self, id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_pool_replicas(&self, node_id: &str, pool_id: &str) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        Replicas::get_node_pool_replicas(self, node_id, pool_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
    
        Replicas::get_node_pool_replica(self, node_id, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
    
        Replicas::put_node_pool_replica(self, node_id, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Replicas::del_node_pool_replica(self, node_id, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_node_pool_replica_share(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Replicas::del_node_pool_replica_share(self, node_id, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_node_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<String, Error<crate::models::RestJsonError>> {
    
        Replicas::put_node_pool_replica_share(self, allowed_hosts, node_id, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
    
        Replicas::put_pool_replica(self, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Replicas::del_pool_replica(self, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_pool_replica_share(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Replicas::del_pool_replica_share(self, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, pool_id: &str, replica_id: &uuid::Uuid) -> Result<String, Error<crate::models::RestJsonError>> {
    
        Replicas::put_pool_replica_share(self, allowed_hosts, pool_id, replica_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_replicas(&self, ) -> Result<Vec<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        Replicas::get_replicas(self, ).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_replica(&self, id: &uuid::Uuid) -> Result<crate::models::Replica, Error<crate::models::RestJsonError>> {
    
        Replicas::get_replica(self, id).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Replicas for ReplicasClient {
    
    
    
    
    async fn get_node_replicas(&self, id: &str) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{id}/replicas", configuration.base_path, id=crate::apis::urlencode(id));
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
            let local_var_content: Vec<crate::models::Replica> =
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
    
    
    
    async fn get_node_pool_replicas(&self, node_id: &str, pool_id: &str) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id));
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
            let local_var_content: Vec<crate::models::Replica> =
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
    
    
    
    async fn get_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
            let local_var_content: crate::models::Replica =
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
    
    
    
    async fn put_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
            let local_var_content: crate::models::Replica =
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
    
    
    
    async fn del_node_pool_replica(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
    
    
    
    async fn del_node_pool_replica_share(&self, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
    
    
    
    async fn put_node_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, node_id: &str, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/nodes/{node_id}/pools/{pool_id}/replicas/{replica_id}/share/nvmf", configuration.base_path, node_id=crate::apis::urlencode(node_id), pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::PUT);

        
        let query_params: Option<String> = None;
        
        
        
        let query_params = if let Some(local_var_str) = allowed_hosts {
            match query_params {
                None => Some(format!("allowed-hosts={}", local_var_str.join(","))),
                Some(previous) => Some(format!("{previous}&allowed-hosts={}", local_var_str.join(",")))
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
    
    
    
    async fn put_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/pools/{pool_id}/replicas/{replica_id}", configuration.base_path, pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
            let local_var_content: crate::models::Replica =
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
    
    
    
    async fn del_pool_replica(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/pools/{pool_id}/replicas/{replica_id}", configuration.base_path, pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
    
    
    
    async fn del_pool_replica_share(&self, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/pools/{pool_id}/replicas/{replica_id}/share", configuration.base_path, pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
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
    
    
    
    async fn put_pool_replica_share(&self, allowed_hosts: Option<Vec<String>>, pool_id: &str, replica_id: &uuid::Uuid) -> Result<ResponseContent<String>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/pools/{pool_id}/replicas/{replica_id}/share/nvmf", configuration.base_path, pool_id=crate::apis::urlencode(pool_id), replica_id=replica_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::PUT);

        
        let query_params: Option<String> = None;
        
        
        
        let query_params = if let Some(local_var_str) = allowed_hosts {
            match query_params {
                None => Some(format!("allowed-hosts={}", local_var_str.join(","))),
                Some(previous) => Some(format!("{previous}&allowed-hosts={}", local_var_str.join(",")))
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
    
    
    
    async fn get_replicas(&self, ) -> Result<ResponseContent<Vec<crate::models::Replica>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/replicas", configuration.base_path);
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
            let local_var_content: Vec<crate::models::Replica> =
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
    
    
    
    async fn get_replica(&self, id: &uuid::Uuid) -> Result<ResponseContent<crate::models::Replica>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/replicas/{id}", configuration.base_path, id=id.to_string());
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
            let local_var_content: crate::models::Replica =
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