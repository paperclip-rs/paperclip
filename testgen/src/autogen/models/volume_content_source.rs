#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// VolumeContentSource : Volume Content Source i.e the snapshot or the volume.








/// Volume Content Source i.e the snapshot or the volume.



#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VolumeContentSource {

    /// The snapshot source for the volume content.
    #[serde(rename = "snapshot")]
    snapshot(crate::models::SnapshotAsSource),

}










