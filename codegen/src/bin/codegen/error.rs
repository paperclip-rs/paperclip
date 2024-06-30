macro_rules! impl_err_from {
    ($err:ident :: $type:ty > $variant:ident) => {
        impl From<$type> for $err {
            fn from(s: $type) -> Self {
                $err::$variant(s)
            }
        }
    };
}

/// Global error which encapsulates all related errors.
#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    /// The given directory cannot be used for generating code.
    #[error("Cannot generate code in the given directory")]
    InvalidCodegenDirectory,
    /// I/O errors.
    #[error("I/O error: {}", _0)]
    Io(std::io::Error),
    /// JSON coding errors.
    #[error("JSON error: {}", _0)]
    Json(serde_json::Error),
    /// YAML coding errors.
    #[error("YAML error: {}", _0)]
    Yaml(serde_yaml::Error),
}

impl_err_from!(Error::std::io::Error > Io);
impl_err_from!(Error::serde_json::Error > Json);
impl_err_from!(Error::serde_yaml::Error > Yaml);
