#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// SetVolumePropertyBody : Set Volume Properties Body








/// Set Volume Properties Body



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SetVolumePropertyBody {

    /// Max Snapshots limit per volume.
    #[serde(rename = "max_snapshots")]
    max_snapshots(u32),

}










