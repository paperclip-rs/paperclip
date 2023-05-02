#![allow(clippy::vec_init_then_push)]

use crate::clients::actix::{
    configuration, Error, ResponseContent, ResponseContentUnexpected,
};
use actix_web_opentelemetry::ClientExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct NodesClient {
    configuration: Rc<configuration::Configuration>,
}

impl NodesClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}

#[async_trait::async_trait(?Send)]
#[dyn_clonable::clonable]
pub trait Nodes: Clone {
    
    
    
    
    async fn get_nodes(&self, node_id: Option<&str>) -> Result<Vec<crate::models::Node>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node(&self, id: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_cordon(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
    
    async fn delete_node_cordon(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_drain(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_label(&self, overwrite: Option<bool>, id: &str, key: &str, value: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
    
    async fn delete_node_label(&self, id: &str, key: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>>;
    
    
}

#[async_trait::async_trait(?Send)]
impl Nodes for NodesClient {
    
    
    
    
    async fn get_nodes(&self, node_id: Option<&str>) -> Result<Vec<crate::models::Node>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes", configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        let mut query_params = vec![];
        
        
        
        if let Some(ref local_var_str) = node_id {
            query_params.push(("node_id", local_var_str.to_string()));
        }
        
        
        local_var_req_builder = local_var_req_builder.query(&query_params)?;
        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::Node>>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn get_node(&self, id: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}", configuration.base_path, id=crate::apis::urlencode(id));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn put_node_cordon(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/cordon/{label}", configuration.base_path, id=crate::apis::urlencode(id), label=crate::apis::urlencode(label));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn delete_node_cordon(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/cordon/{label}", configuration.base_path, id=crate::apis::urlencode(id), label=crate::apis::urlencode(label));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn put_node_drain(&self, id: &str, label: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/drain/{label}", configuration.base_path, id=crate::apis::urlencode(id), label=crate::apis::urlencode(label));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn put_node_label(&self, overwrite: Option<bool>, id: &str, key: &str, value: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/label/{key}={value}", configuration.base_path, id=crate::apis::urlencode(id), key=crate::apis::urlencode(key), value=crate::apis::urlencode(value));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        
        let mut query_params = vec![];
        
        
        
        if let Some(ref local_var_str) = overwrite {
            query_params.push(("overwrite", local_var_str.to_string()));
        }
        
        
        local_var_req_builder = local_var_req_builder.query(&query_params)?;
        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn delete_node_label(&self, id: &str, key: &str) -> Result<crate::models::Node, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/label/{key}", configuration.base_path, id=crate::apis::urlencode(id), key=crate::apis::urlencode(key));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Node>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
}