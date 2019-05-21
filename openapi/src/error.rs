use std::io;
use std::path::PathBuf;

macro_rules! impl_err_from {
    ($err:ident :: $type:ty > $variant:ident) => {
        impl From<$type> for $err {
            fn from(s: $type) -> Self {
                $err::$variant(s)
            }
        }
    };
}

/// Generic result used throughout this library.
pub type PaperClipResult<T> = Result<T, PaperClipError>;

/// Global error which encapsulates all related errors.
#[derive(Debug, Fail)]
pub enum PaperClipError {
    #[fail(
        display = "Invalid $ref URI: {}. Only relative URIs for definitions are supported right now.",
        _0
    )]
    InvalidURI(String),
    #[fail(display = "Mising item schema for array: {:?}", _0)]
    MissingArrayItem(Option<String>),
    #[fail(display = "Invalid name for definition")]
    InvalidDefinitionName,
    #[fail(display = "Invalid path for definition")]
    InvalidDefinitionPath(PathBuf),
    #[fail(display = "Definition missing: {}", _0)]
    MissingDefinition(String),
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error),
    #[fail(display = "JSON error: {}", _0)]
    Json(serde_json::Error),
    #[fail(display = "YAML error: {}", _0)]
    Yaml(serde_yaml::Error),
}

impl_err_from!(PaperClipError::io::Error > Io);
impl_err_from!(PaperClipError::serde_json::Error > Json);
impl_err_from!(PaperClipError::serde_yaml::Error > Yaml);
