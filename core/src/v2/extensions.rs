use lazy_static::lazy_static;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

lazy_static! {
    /// Media range for JSON.
    pub static ref JSON_MIME: MediaRange =
        MediaRange(mime::MediaRange::parse("application/json").expect("parsing mime"));
    /// Default coder for JSON.
    pub static ref JSON_CODER: Arc<Coder> = Arc::new(Coder {
        encoder_path: "serde_json::to_writer".into(),
        decoder_path: "serde_json::from_reader".into(),
        any_value: "serde_json::Value".into(),
        error_path: "serde_json::Error".into(),
        prefer: false,
        builtin: true,
    });
    /// Media range for YAML.
    pub static ref YAML_MIME: MediaRange =
        MediaRange(mime::MediaRange::parse("application/yaml").expect("parsing mime"));
    /// Default coder for YAML.
    pub static ref YAML_CODER: Arc<Coder> = Arc::new(Coder {
        encoder_path: "serde_yaml::to_writer".into(),
        decoder_path: "serde_yaml::from_reader".into(),
        any_value: "serde_yaml::Value".into(),
        error_path: "serde_yaml::Error".into(),
        prefer: false,
        builtin: true,
    });
}

/// Wrapper for `mime::MediaRange` to support `BTree{Set, Map}`.
#[derive(Debug, Clone)]
pub struct MediaRange(pub mime::MediaRange);

impl MediaRange {
    /// Implementation from https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L155
    fn matches_params(&self, r: &Self) -> bool {
        for (name, value) in self.0.params() {
            if name != "q" && r.0.param(name) != Some(value) {
                return false;
            }
        }

        true
    }
}

/// `x-coder` global extension for custom encoders and decoders.
#[derive(Debug, Default, Clone)]
pub struct Coders(BTreeMap<MediaRange, Arc<Coder>>);

#[cfg(feature = "codegen")]
impl Coders {
    /// Returns the matching coder for the given media range (if any).
    ///
    /// Matching algorithm from https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L126
    pub fn matching_coder(&self, ty: &MediaRange) -> Option<Arc<Coder>> {
        self.0
            .get(ty)
            .or_else(|| {
                let (target_t1, target_t2) = (ty.0.type_(), ty.0.subtype());
                for (r, c) in &self.0 {
                    let (source_t1, source_t2) = (r.0.type_(), r.0.subtype());
                    if target_t1 == mime::STAR && r.matches_params(ty) {
                        return Some(c);
                    }

                    if source_t1 != target_t1 {
                        continue;
                    }

                    if target_t2 == mime::STAR && r.matches_params(ty) {
                        return Some(c);
                    }

                    if source_t2 != target_t2 {
                        continue;
                    }

                    return Some(c);
                }

                None
            })
            .map(Clone::clone)
    }
}

/// Represents the en/decoder for some MIME media range.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Coder {
    /// Path to the encoding function.
    pub encoder_path: String,
    /// Path to the decoding function.
    pub decoder_path: String,
    /// Path to the error type.
    pub error_path: String,
    /// Path to the struct/enum that represents `Any` (such as `serde_json::Value`).
    pub any_value: String,
    /// Whether this media type should be preferred when multiple
    /// types are available. When multiple types are preferred,
    /// it's unspecified as to which is chosen.
    #[serde(default)]
    pub prefer: bool,
    /// Whether this en/decoder is built-in.
    #[serde(skip)]
    pub builtin: bool,
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
        Some(self.cmp(other))
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
    type Target = BTreeMap<MediaRange, Arc<Coder>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Coders {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.0
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
