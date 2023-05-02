#![allow(clippy::vec_init_then_push)]

use crate::clients::actix::{
    configuration, Error, ResponseContent, ResponseContentUnexpected,
};
use actix_web_opentelemetry::ClientExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct NexusesClient {
    configuration: Rc<configuration::Configuration>,
}

impl NexusesClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}

#[async_trait::async_trait(?Send)]
#[dyn_clonable::clonable]
pub trait Nexuses: Clone {
    
    
    
    
    async fn get_nexuses(&self, ) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexuses(&self, id: &str) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<String, Error<crate::models::RestJsonError>>;
    
    
}

#[async_trait::async_trait(?Send)]
impl Nexuses for NexusesClient {
    
    
    
    
    async fn get_nexuses(&self, ) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nexuses", configuration.base_path);
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
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::Nexus>>().await?;
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
    
    
    
    async fn get_nexus(&self, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}", configuration.base_path, nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Nexus>().await?;
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
    
    
    
    async fn del_nexus(&self, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nexuses/{nexus_id}", configuration.base_path, nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<()>().await?;
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
    
    
    
    async fn get_node_nexuses(&self, id: &str) -> Result<Vec<crate::models::Nexus>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{id}/nexuses", configuration.base_path, id=crate::apis::urlencode(id));
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
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::Nexus>>().await?;
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
    
    
    
    async fn get_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Nexus>().await?;
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
    
    
    
    async fn put_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<crate::models::Nexus, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::Nexus>().await?;
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
    
    
    
    async fn del_node_nexus(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<()>().await?;
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
    
    
    
    async fn del_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/share", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string());
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
            
            
            
            let local_var_content = local_var_resp.json::<()>().await?;
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
    
    
    
    async fn put_node_nexus_share(&self, node_id: &str, nexus_id: &uuid::Uuid, protocol: &str) -> Result<String, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node_id}/nexuses/{nexus_id}/share/{protocol}", configuration.base_path, node_id=crate::apis::urlencode(node_id), nexus_id=nexus_id.to_string(), protocol=crate::apis::urlencode(protocol));
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
            
            
            
            let local_var_content = local_var_resp.json::<String>().await?;
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