//! Interior mutability stuff.

#[cfg(feature = "v2")]
use crate::v2::models::Resolvable;
#[cfg(feature = "v2")]
use serde::{Serialize, Serializer};

#[cfg(feature = "v2")]
impl<T> Serialize for Resolvable<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Resolvable::Raw(s) => s.serialize(serializer),
            Resolvable::Resolved { new, .. } => new.serialize(serializer),
        }
    }
}
