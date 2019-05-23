//! Errors.

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
    /// Failed to resolve the schema because an invalid URI was provided for
    /// `$ref` field. Currently, we only support `#/definitions/YourType` in
    /// `$ref` field.
    #[fail(
        display = "Invalid $ref URI: {}. Only relative URIs for definitions are supported right now.",
        _0
    )]
    InvalidRefURI(String),
    /// The given schema object is an array, but the `items` field is missing.
    #[fail(display = "Mising item schema for array: {:?}", _0)]
    MissingArrayItem(Option<String>),
    /// The name for the given definition is missing or invalid.
    #[fail(display = "Invalid name for definition")]
    InvalidDefinitionName,
    /// A relative path cannot be obtained for the given defition.
    #[fail(display = "Invalid path for definition")]
    InvalidDefinitionPath(PathBuf),
    /// A definition has been referenced but it's missing.
    #[fail(display = "Definition missing: {}", _0)]
    MissingDefinition(String),
    /// I/O errors.
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error),
    /// JSON coding errors.
    #[fail(display = "JSON error: {}", _0)]
    Json(serde_json::Error),
    /// YAML coding errors.
    #[fail(display = "YAML error: {}", _0)]
    Yaml(serde_yaml::Error),
    #[cfg(feature = "codegen-fmt")]
    /// Errors from rustfmt.
    #[fail(display = "Rustfmt formatting error: {}", _0)]
    RustFmt(rustfmt_nightly::ErrorKind),
}

impl_err_from!(PaperClipError::io::Error > Io);
impl_err_from!(PaperClipError::serde_json::Error > Json);
impl_err_from!(PaperClipError::serde_yaml::Error > Yaml);
#[cfg(feature = "codegen-fmt")]
impl_err_from!(PaperClipError::rustfmt_nightly::ErrorKind > RustFmt);
