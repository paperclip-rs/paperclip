#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]

use crate::apis::{IntoOptVec, IntoVec};



/// Pool : Pool object, comprised of a spec and a state








/// Pool object, comprised of a spec and a state

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pool {

    /// storage pool identifier
    #[serde(default, rename = "id")]
    pub id: String,

    /// User specification of a pool.
    #[serde(default, rename = "spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<crate::models::PoolSpec>,

    /// State of a pool, as reported by io-engine
    #[serde(default, rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::PoolState>,

}

impl Pool {
    /// Pool using only the required fields
    pub fn new(id: impl Into<String>) -> Pool {
        Pool {
            id: id.into(),
            spec: None,
            state: None,
            
        }
    }
    /// Pool using all fields
    pub fn new_all(id: impl Into<String>, spec: impl Into<Option<crate::models::PoolSpec>>, state: impl Into<Option<crate::models::PoolState>>) -> Pool {
        Pool {
            id: id.into(),
            spec: spec.into(),
            state: state.into(),
            
        }
    }
}














