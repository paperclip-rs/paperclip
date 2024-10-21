#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// RestJsonError : Rest Json Error format








/// Rest Json Error format

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RestJsonError {

    /// detailed error information
    #[serde(default, rename = "details")]
    pub details: String,

    /// last reported error information
    #[serde(default, rename = "message")]
    pub message: String,

    /// error kind
    #[serde(default, rename = "kind")]
    pub kind: Kind,

}

impl RestJsonError {
    /// RestJsonError using only the required fields
    pub fn new(details: impl Into<String>, message: impl Into<String>, kind: impl Into<Kind>) -> RestJsonError {
        RestJsonError {
            details: details.into(),
            message: message.into(),
            kind: kind.into(),
            
        }
    }
    /// RestJsonError using all fields
    pub fn new_all(details: impl Into<String>, message: impl Into<String>, kind: impl Into<Kind>) -> RestJsonError {
        RestJsonError {
            details: details.into(),
            message: message.into(),
            kind: kind.into(),
            
        }
    }
}












/// error kind
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {


    #[serde(rename = "Timeout")]
    Timeout,

    #[serde(rename = "Deserialize")]
    Deserialize,

    #[serde(rename = "Internal")]
    Internal,

    #[serde(rename = "InvalidArgument")]
    InvalidArgument,

    #[serde(rename = "DeadlineExceeded")]
    DeadlineExceeded,

    #[serde(rename = "NotFound")]
    NotFound,

    #[serde(rename = "AlreadyExists")]
    AlreadyExists,

    #[serde(rename = "PermissionDenied")]
    PermissionDenied,

    #[serde(rename = "ResourceExhausted")]
    ResourceExhausted,

    #[serde(rename = "FailedPrecondition")]
    FailedPrecondition,

    #[serde(rename = "NotShared")]
    NotShared,

    #[serde(rename = "NotPublished")]
    NotPublished,

    #[serde(rename = "AlreadyPublished")]
    AlreadyPublished,

    #[serde(rename = "AlreadyShared")]
    AlreadyShared,

    #[serde(rename = "Aborted")]
    Aborted,

    #[serde(rename = "OutOfRange")]
    OutOfRange,

    #[serde(rename = "Unimplemented")]
    Unimplemented,

    #[serde(rename = "Unavailable")]
    Unavailable,

    #[serde(rename = "Unauthenticated")]
    Unauthenticated,

    #[serde(rename = "Unauthorized")]
    Unauthorized,

    #[serde(rename = "Conflict")]
    Conflict,

    #[serde(rename = "FailedPersist")]
    FailedPersist,

    #[serde(rename = "Deleting")]
    Deleting,

    #[serde(rename = "InUse")]
    InUse,

    #[serde(rename = "CapacityLimitExceeded")]
    CapacityLimitExceeded,

    #[serde(rename = "NotAcceptable")]
    NotAcceptable,


}

impl Default for Kind {
    fn default() -> Self {
        Self::Timeout
    }
}




