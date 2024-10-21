#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// CordonDrainState : The drain state








/// The drain state



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CordonDrainState {

    /// The item is cordoned
    #[serde(rename = "cordonedstate")]
    cordonedstate(crate::models::CordonedState),

    /// The item is draining
    #[serde(rename = "drainingstate")]
    drainingstate(crate::models::DrainState),

    /// The item is draining
    #[serde(rename = "drainedstate")]
    drainedstate(crate::models::DrainState),

}














