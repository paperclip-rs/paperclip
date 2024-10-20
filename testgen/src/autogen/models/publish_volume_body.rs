#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// PublishVolumeBody : Publish Volume Body








/// Publish Volume Body

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublishVolumeBody {

    /// Controller Volume Publish context
    #[serde(default, rename = "publish_context")]
    pub publish_context: ::std::collections::HashMap<String, String>,

    /// Allows reusing of the current target.
    #[serde(default, rename = "reuse_existing", skip_serializing_if = "Option::is_none")]
    pub reuse_existing: Option<bool>,

    /// The node where the target will reside in. It may be moved elsewhere during volume republish.
    #[serde(default, rename = "node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,

    /// The protocol used to connect to the front-end node.
    #[serde(default, rename = "protocol")]
    pub protocol: crate::models::VolumeShareProtocol,

    /// Allows republishing the volume on the node by shutting down the existing target first.
    #[serde(default, rename = "republish", skip_serializing_if = "Option::is_none")]
    pub republish: Option<bool>,

    /// The node where the front-end workload resides. If the workload moves then the volume must be republished.
    #[serde(default, rename = "frontend_node", skip_serializing_if = "Option::is_none")]
    pub frontend_node: Option<String>,

}

impl PublishVolumeBody {
    /// PublishVolumeBody using only the required fields
    pub fn new(publish_context: impl Into<::std::collections::HashMap<String, String>>, protocol: impl Into<crate::models::VolumeShareProtocol>) -> PublishVolumeBody {
        PublishVolumeBody {
            publish_context: publish_context.into(),
            reuse_existing: None,
            node: None,
            protocol: protocol.into(),
            republish: None,
            frontend_node: None,
            
        }
    }
    /// PublishVolumeBody using all fields
    pub fn new_all(publish_context: impl Into<::std::collections::HashMap<String, String>>, reuse_existing: impl Into<Option<bool>>, node: impl Into<Option<String>>, protocol: impl Into<crate::models::VolumeShareProtocol>, republish: impl Into<Option<bool>>, frontend_node: impl Into<Option<String>>) -> PublishVolumeBody {
        PublishVolumeBody {
            publish_context: publish_context.into(),
            reuse_existing: reuse_existing.into(),
            node: node.into(),
            protocol: protocol.into(),
            republish: republish.into(),
            frontend_node: frontend_node.into(),
            
        }
    }
}




















