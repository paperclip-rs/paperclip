mod body;
pub mod configuration;

use configuration::BoxedError;
pub use configuration::Configuration;
pub use hyper::{self, StatusCode, Uri};
pub use url::Url;

use std::{error, fmt, ops::Deref, sync::Arc};

#[derive(Clone)]
pub struct ApiClient {



    block_devices_api: Box<dyn crate::apis::block_devices_api::tower::client::BlockDevices>,



    children_api: Box<dyn crate::apis::children_api::tower::client::Children>,



    json_grpc_api: Box<dyn crate::apis::json_grpc_api::tower::client::JsonGrpc>,



    nexuses_api: Box<dyn crate::apis::nexuses_api::tower::client::Nexuses>,



    nodes_api: Box<dyn crate::apis::nodes_api::tower::client::Nodes>,



    pools_api: Box<dyn crate::apis::pools_api::tower::client::Pools>,



    replicas_api: Box<dyn crate::apis::replicas_api::tower::client::Replicas>,



    snapshots_api: Box<dyn crate::apis::snapshots_api::tower::client::Snapshots>,



    specs_api: Box<dyn crate::apis::specs_api::tower::client::Specs>,



    volumes_api: Box<dyn crate::apis::volumes_api::tower::client::Volumes>,



    watches_api: Box<dyn crate::apis::watches_api::tower::client::Watches>,



}

/// Same as `ApiClient` but returns the body directly
pub mod direct {
    #[derive(Clone)]
    pub struct ApiClient {
    
    
    
        block_devices_api: Box<dyn crate::apis::block_devices_api::tower::client::direct::BlockDevices>,
    
    
    
        children_api: Box<dyn crate::apis::children_api::tower::client::direct::Children>,
    
    
    
        json_grpc_api: Box<dyn crate::apis::json_grpc_api::tower::client::direct::JsonGrpc>,
    
    
    
        nexuses_api: Box<dyn crate::apis::nexuses_api::tower::client::direct::Nexuses>,
    
    
    
        nodes_api: Box<dyn crate::apis::nodes_api::tower::client::direct::Nodes>,
    
    
    
        pools_api: Box<dyn crate::apis::pools_api::tower::client::direct::Pools>,
    
    
    
        replicas_api: Box<dyn crate::apis::replicas_api::tower::client::direct::Replicas>,
    
    
    
        snapshots_api: Box<dyn crate::apis::snapshots_api::tower::client::direct::Snapshots>,
    
    
    
        specs_api: Box<dyn crate::apis::specs_api::tower::client::direct::Specs>,
    
    
    
        volumes_api: Box<dyn crate::apis::volumes_api::tower::client::direct::Volumes>,
    
    
    
        watches_api: Box<dyn crate::apis::watches_api::tower::client::direct::Watches>,
    
    
    
    }

    impl ApiClient {
        pub fn new(configuration: super::Configuration) -> ApiClient {
            let rc = super::Arc::new(configuration);

            ApiClient {
    
    
    
                
                block_devices_api: Box::new(crate::apis::block_devices_api::tower::client::BlockDevicesClient::new(rc.clone())),
                
                
    
    
    
                
                children_api: Box::new(crate::apis::children_api::tower::client::ChildrenClient::new(rc.clone())),
                
                
    
    
    
                
                json_grpc_api: Box::new(crate::apis::json_grpc_api::tower::client::JsonGrpcClient::new(rc.clone())),
                
                
    
    
    
                
                nexuses_api: Box::new(crate::apis::nexuses_api::tower::client::NexusesClient::new(rc.clone())),
                
                
    
    
    
                
                nodes_api: Box::new(crate::apis::nodes_api::tower::client::NodesClient::new(rc.clone())),
                
                
    
    
    
                
                pools_api: Box::new(crate::apis::pools_api::tower::client::PoolsClient::new(rc.clone())),
                
                
    
    
    
                
                replicas_api: Box::new(crate::apis::replicas_api::tower::client::ReplicasClient::new(rc.clone())),
                
                
    
    
    
                
                snapshots_api: Box::new(crate::apis::snapshots_api::tower::client::SnapshotsClient::new(rc.clone())),
                
                
    
    
    
                
                specs_api: Box::new(crate::apis::specs_api::tower::client::SpecsClient::new(rc.clone())),
                
                
    
    
    
                
                volumes_api: Box::new(crate::apis::volumes_api::tower::client::VolumesClient::new(rc.clone())),
                
                
    
    
    
                
                
                watches_api: Box::new(crate::apis::watches_api::tower::client::WatchesClient::new(rc)),
                
    
    
    
            }
        }

    
    
    
        pub fn block_devices_api(&self) -> &dyn crate::apis::block_devices_api::tower::client::direct::BlockDevices {
            self.block_devices_api.as_ref()
        }
    
    
    
        pub fn children_api(&self) -> &dyn crate::apis::children_api::tower::client::direct::Children {
            self.children_api.as_ref()
        }
    
    
    
        pub fn json_grpc_api(&self) -> &dyn crate::apis::json_grpc_api::tower::client::direct::JsonGrpc {
            self.json_grpc_api.as_ref()
        }
    
    
    
        pub fn nexuses_api(&self) -> &dyn crate::apis::nexuses_api::tower::client::direct::Nexuses {
            self.nexuses_api.as_ref()
        }
    
    
    
        pub fn nodes_api(&self) -> &dyn crate::apis::nodes_api::tower::client::direct::Nodes {
            self.nodes_api.as_ref()
        }
    
    
    
        pub fn pools_api(&self) -> &dyn crate::apis::pools_api::tower::client::direct::Pools {
            self.pools_api.as_ref()
        }
    
    
    
        pub fn replicas_api(&self) -> &dyn crate::apis::replicas_api::tower::client::direct::Replicas {
            self.replicas_api.as_ref()
        }
    
    
    
        pub fn snapshots_api(&self) -> &dyn crate::apis::snapshots_api::tower::client::direct::Snapshots {
            self.snapshots_api.as_ref()
        }
    
    
    
        pub fn specs_api(&self) -> &dyn crate::apis::specs_api::tower::client::direct::Specs {
            self.specs_api.as_ref()
        }
    
    
    
        pub fn volumes_api(&self) -> &dyn crate::apis::volumes_api::tower::client::direct::Volumes {
            self.volumes_api.as_ref()
        }
    
    
    
        pub fn watches_api(&self) -> &dyn crate::apis::watches_api::tower::client::direct::Watches {
            self.watches_api.as_ref()
        }
    
    
    
    }
}

impl ApiClient {
    pub fn new(configuration: Configuration) -> ApiClient {
        let rc = Arc::new(configuration);

        ApiClient {



            
            block_devices_api: Box::new(crate::apis::block_devices_api::tower::client::BlockDevicesClient::new(rc.clone())),
            
            



            
            children_api: Box::new(crate::apis::children_api::tower::client::ChildrenClient::new(rc.clone())),
            
            



            
            json_grpc_api: Box::new(crate::apis::json_grpc_api::tower::client::JsonGrpcClient::new(rc.clone())),
            
            



            
            nexuses_api: Box::new(crate::apis::nexuses_api::tower::client::NexusesClient::new(rc.clone())),
            
            



            
            nodes_api: Box::new(crate::apis::nodes_api::tower::client::NodesClient::new(rc.clone())),
            
            



            
            pools_api: Box::new(crate::apis::pools_api::tower::client::PoolsClient::new(rc.clone())),
            
            



            
            replicas_api: Box::new(crate::apis::replicas_api::tower::client::ReplicasClient::new(rc.clone())),
            
            



            
            snapshots_api: Box::new(crate::apis::snapshots_api::tower::client::SnapshotsClient::new(rc.clone())),
            
            



            
            specs_api: Box::new(crate::apis::specs_api::tower::client::SpecsClient::new(rc.clone())),
            
            



            
            volumes_api: Box::new(crate::apis::volumes_api::tower::client::VolumesClient::new(rc.clone())),
            
            



            
            
            watches_api: Box::new(crate::apis::watches_api::tower::client::WatchesClient::new(rc)),
            



        }
    }




    pub fn block_devices_api(&self) -> &dyn crate::apis::block_devices_api::tower::client::BlockDevices {
        self.block_devices_api.as_ref()
    }



    pub fn children_api(&self) -> &dyn crate::apis::children_api::tower::client::Children {
        self.children_api.as_ref()
    }



    pub fn json_grpc_api(&self) -> &dyn crate::apis::json_grpc_api::tower::client::JsonGrpc {
        self.json_grpc_api.as_ref()
    }



    pub fn nexuses_api(&self) -> &dyn crate::apis::nexuses_api::tower::client::Nexuses {
        self.nexuses_api.as_ref()
    }



    pub fn nodes_api(&self) -> &dyn crate::apis::nodes_api::tower::client::Nodes {
        self.nodes_api.as_ref()
    }



    pub fn pools_api(&self) -> &dyn crate::apis::pools_api::tower::client::Pools {
        self.pools_api.as_ref()
    }



    pub fn replicas_api(&self) -> &dyn crate::apis::replicas_api::tower::client::Replicas {
        self.replicas_api.as_ref()
    }



    pub fn snapshots_api(&self) -> &dyn crate::apis::snapshots_api::tower::client::Snapshots {
        self.snapshots_api.as_ref()
    }



    pub fn specs_api(&self) -> &dyn crate::apis::specs_api::tower::client::Specs {
        self.specs_api.as_ref()
    }



    pub fn volumes_api(&self) -> &dyn crate::apis::volumes_api::tower::client::Volumes {
        self.volumes_api.as_ref()
    }



    pub fn watches_api(&self) -> &dyn crate::apis::watches_api::tower::client::Watches {
        self.watches_api.as_ref()
    }



}

/// Http Response with status and body
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub(crate) status: hyper::StatusCode,
    pub(crate) body: T,
}
impl<T> ResponseContent<T> {
    /// Get the status code
    pub fn status(&self) -> hyper::StatusCode {
        self.status
    }
    /// Get a reference to the body
    pub fn body(&self) -> &T {
        &self.body
    }
    /// Convert self into the body
    pub fn into_body(self) -> T {
        self.body
    }
}

/// Http Response with status and body as text (could not be coerced into the expected type)
#[derive(Debug, Clone)]
pub struct ResponseContentUnexpected {
    pub(crate) status: hyper::StatusCode,
    pub(crate) text: String,
}
impl ResponseContentUnexpected {
    /// Get the status code
    pub fn status(&self) -> hyper::StatusCode {
        self.status
    }
    /// Get a reference to the text
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }
}

/// Error type for all Requests with the various variants
#[derive(Debug)]
pub enum Error<T> {
    Request(RequestError),
    Response(ResponseError<T>),
}
impl<T> From<RequestError> for Error<T> {
    fn from(src: RequestError) -> Self {
        Self::Request(src)
    }
}
impl<T> From<ResponseError<T>> for Error<T> {
    fn from(src: ResponseError<T>) -> Self {
        Self::Response(src)
    }
}
impl<T: fmt::Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Request(r) => r.fmt(f),
            Error::Response(r) => r.fmt(f),
        }
    }
}
impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Request(r) => r.source(),
            Error::Response(r) => r.source(),
        }
    }
}

/// Failed to issue the request
#[derive(Debug)]
pub enum RequestError {
    /// Failed to build the http request
    BuildRequest(hyper::http::Error),
    /// Service Request call returned an error
    Request(BoxedError),
    /// Service was not ready to process the request
    NotReady(BoxedError),
    /// Failed to serialize request payload
    Serde(serde_json::Error),
    /// Failed to encode the url path
    SerdeEncoded(serde_urlencoded::ser::Error),
}
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            RequestError::BuildRequest(e) => ("build_request", e.to_string()),
            RequestError::Request(e) => ("request", e.to_string()),
            RequestError::NotReady(e) => ("not_ready", e.to_string()),
            RequestError::Serde(e) => ("serde", e.to_string()),
            RequestError::SerdeEncoded(e) => ("serde_encoding", e.to_string()),
        };
        write!(f, "error in {module}: {e}")
    }
}
impl error::Error for RequestError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            RequestError::BuildRequest(e) => e,
            RequestError::Request(e) => e.deref(),
            RequestError::NotReady(e) => e.deref(),
            RequestError::Serde(e) => e,
            RequestError::SerdeEncoded(e) => e,
        })
    }
}

/// Error type for all Requests with the various variants
#[derive(Debug)]
pub enum ResponseError<T> {
    /// The OpenAPI call returned the "expected" OpenAPI JSON content
    Expected(ResponseContent<T>),
    /// Failed to convert the response payload to bytes
    PayloadError {
        status: hyper::StatusCode,
        error: hyper::Error,
    },
    /// The OpenAPI call returned an "unexpected" JSON content
    Unexpected(ResponseContentUnexpected),
}
impl<T: fmt::Debug> fmt::Display for ResponseError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            ResponseError::Expected(e) => (
                "response",
                format!("status code '{}', content: '{:?}'", e.status, e.body),
            ),
            ResponseError::PayloadError { status, error } => (
                "response",
                format!("status code '{status}', error: '{error:?}'"),
            ),
            ResponseError::Unexpected(e) => (
                "response",
                format!("status code '{}', text '{}'", e.status, e.text),
            ),
        };
        write!(f, "error in {module}: {e}")
    }
}
impl<T: fmt::Debug> error::Error for ResponseError<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ResponseError::Expected(_) => None,
            ResponseError::PayloadError { error, .. } => Some(error),
            ResponseError::Unexpected(_) => None,
        }
    }
}
impl<T> ResponseError<T> {
    /// Get the inner status
    pub fn status(&self) -> StatusCode {
        match self {
            ResponseError::Expected(expected) => expected.status,
            ResponseError::PayloadError { status, .. } => *status,
            ResponseError::Unexpected(unexpected) => unexpected.status,
        }
    }
}

impl std::fmt::Debug for ApiClient {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        fmt::Result::Ok(())
    }
}