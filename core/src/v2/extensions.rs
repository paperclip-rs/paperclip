use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Deref;

/// Wrapper for `mime::MediaRange` to support `BTree{Set, Map}`.
#[derive(Debug, Clone)]
pub struct MediaRange(mime::MediaRange);

/// `x-encoder` and `x-decoder` global extension
/// for custom encoders and decoders.
#[derive(Debug, Default, Clone)]
pub struct Coders(pub BTreeMap<MediaRange, Coder>);

/// Represents the en/decoder for some MIME media range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coder {
    /// Path to the en/decoder function.
    pub coder_path: String,
    /// Path to the error type.
    pub error_path: String,
}

/* Common trait impls */

impl PartialEq for MediaRange {
    fn eq(&self, other: &MediaRange) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for MediaRange {}

impl PartialOrd for MediaRange {
    fn partial_cmp(&self, other: &MediaRange) -> Option<Ordering> {
        Some(self.0.as_ref().cmp(other.0.as_ref()))
    }
}

impl Ord for MediaRange {
    fn cmp(&self, other: &MediaRange) -> Ordering {
        self.0.as_ref().cmp(other.0.as_ref())
    }
}

impl Serialize for MediaRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MediaRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(MediaRange(mime::MediaRange::deserialize(deserializer)?))
    }
}

impl Deref for Coders {
    type Target = BTreeMap<MediaRange, Coder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Coders {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Coders {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Coders(BTreeMap::deserialize(deserializer)?))
    }
}
