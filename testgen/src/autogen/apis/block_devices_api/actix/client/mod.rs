#![allow(clippy::vec_init_then_push)]

use crate::clients::actix::{
    configuration, Error, ResponseContent, ResponseContentUnexpected,
};
use actix_web_opentelemetry::ClientExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct BlockDevicesClient {
    configuration: Rc<configuration::Configuration>,
}

impl BlockDevicesClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}

#[async_trait::async_trait(?Send)]
#[dyn_clonable::clonable]
pub trait BlockDevices: Clone {
    
    
    
    
    async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<Vec<crate::models::BlockDevice>, Error<crate::models::RestJsonError>>;
    
    
}

#[async_trait::async_trait(?Send)]
impl BlockDevices for BlockDevicesClient {
    
    
    
    
    async fn get_node_block_devices(&self, all: Option<bool>, node: &str) -> Result<Vec<crate::models::BlockDevice>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/nodes/{node}/block_devices", configuration.base_path, node=crate::apis::urlencode(node));
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        let mut query_params = vec![];
        
        
        
        if let Some(ref local_var_str) = all {
            query_params.push(("all", local_var_str.to_string()));
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
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::BlockDevice>>().await?;
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