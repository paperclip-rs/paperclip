use serde::{Deserialize, Deserializer, Serialize, Serializer};

use once_cell::sync::Lazy;

use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Media range for JSON.
pub static JSON_MIME: Lazy<MediaRange> =
    Lazy::new(|| MediaRange("application/json".parse().expect("parsing mime")));
/// Default coder for JSON.
pub static JSON_CODER: Lazy<Arc<Coder>> = Lazy::new(|| {
    Arc::new(Coder {
        encoder_path: "serde_json::to_writer".into(),
        decoder_path: "serde_json::from_reader".into(),
        any_value: "serde_json::Value".into(),
        error_path: "serde_json::Error".into(),
        prefer: false,
        builtin: true,
    })
});
/// Media range for YAML.
pub static YAML_MIME: Lazy<MediaRange> =
    Lazy::new(|| MediaRange("application/yaml".parse().expect("parsing mime")));
/// Default coder for YAML.
pub static YAML_CODER: Lazy<Arc<Coder>> = Lazy::new(|| {
    Arc::new(Coder {
        encoder_path: "serde_yaml::to_writer".into(),
        decoder_path: "serde_yaml::from_reader".into(),
        any_value: "serde_yaml::Value".into(),
        error_path: "serde_yaml::Error".into(),
        prefer: false,
        builtin: true,
    })
});

/// Wrapper for `mime::MediaRange` to support `BTree{Set, Map}`.
#[derive(Debug, Clone)]
pub struct MediaRange(pub mime::Mime);

#[cfg(feature = "codegen")]
impl MediaRange {
    /// Implementation from https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L155
    fn matches_params(&self, r: &Self) -> bool {
        for (name, value) in self.0.params() {
            if name != "q" && r.0.get_param(name) != Some(value) {
                return false;
            }
        }

        true
    }
}

/// `x-rust-coders` global extension for custom encoders and decoders.
#[derive(Debug, Default, Clone)]
pub struct Coders(BTreeMap<MediaRange, Arc<Coder>>);

#[cfg(feature = "codegen")]
impl Coders {
    /// Returns the matching coder for the given media range (if any).
    ///
    /// Matching algorithm from <https://github.com/hyperium/mime/blob/65ea9c3d0cad4cb548b41124050c545120134035/src/range.rs#L126>
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
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for MediaRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MediaRange;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid media range")
            }

            fn visit_str<E>(self, value: &str) -> Result<MediaRange, E>
            where
                E: serde::de::Error,
            {
                value.parse().map_err(E::custom).map(MediaRange)
            }
        }

        deserializer.deserialize_str(Visitor)
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

/// Method used by openapiv3 crate.
/// Works when deserializing but one could still add keys that don't start with "x-".
/// todo: add own extensions map type that enforces "x-".
pub(crate) fn deserialize_extensions<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<String, serde_json::Value>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(PredicateVisitor(
        |key: &String| key.starts_with("x-"),
        std::marker::PhantomData,
    ))
}

/// Modified to BTreeMap from openapiv3 crate
/// Used to deserialize IndexMap<K, V> that are flattened within other structs.
/// This only adds keys that satisfy the given predicate.
pub(crate) struct PredicateVisitor<F, K, V>(pub F, pub std::marker::PhantomData<(K, V)>);

impl<'de, F, K, V> serde::de::Visitor<'de> for PredicateVisitor<F, K, V>
where
    F: Fn(&K) -> bool,
    K: Deserialize<'de> + Eq + std::hash::Hash + std::cmp::Ord,
    V: Deserialize<'de>,
{
    type Value = BTreeMap<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map whose fields obey a predicate")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut ret = Self::Value::default();

        loop {
            match map.next_key::<K>() {
                Err(_) => (),
                Ok(None) => break,
                Ok(Some(key)) if self.0(&key) => {
                    let _ = ret.insert(key, map.next_value()?);
                }
                Ok(Some(_)) => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        Ok(ret)
    }
}
