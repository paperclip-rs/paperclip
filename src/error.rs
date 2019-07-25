use std::collections::HashSet;
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
    /// Error encountered during spec validation.
    #[fail(display = "{}", _0)]
    Validation(paperclip_core::ValidationError),
    /// The given directory cannot be used for generating code.
    #[fail(display = "Cannot generate code in the given directory")]
    InvalidCodegenDirectory,
    /// Currently, we only support OpenAPI v2, and eventually v3.
    #[fail(display = "This version of OpenAPI is unsupported.")]
    UnsupportedOpenAPIVersion,
    /// Paths listed in the spec must be unique.
    #[fail(display = "Path similar to {:?} already exists.", _0)]
    RelativePathNotUnique(String),
    #[fail(
        display = "Parameter(s) {:?} aren't defined for templated path {:?}",
        _1, _0
    )]
    MissingParametersInPath(String, HashSet<String>),
    /// Invalid host for URL.
    #[fail(display = "Cannot parse host {:?}: {}", _0, _1)]
    InvalidHost(String, url::ParseError),
    /// Invalid base path URL.
    #[fail(display = "Cannot set URL {:?}: {}", _0, _1)]
    InvalidBasePathURL(String, url::ParseError),
    /// The given schema object is an array, but the `items` field is missing.
    #[fail(display = "Mising item schema for array: {:?}", _0)]
    MissingArrayItem(Option<String>),
    /// The name for the given definition is missing or invalid.
    #[fail(display = "Invalid name for definition")]
    InvalidDefinitionName,
    /// A valid path cannot be obtained for the given definition.
    #[fail(display = "Invalid path for definition: {:?}", _0)]
    InvalidDefinitionPath(PathBuf),
    /// If a parameter uses a schema, then we expect it to exist in
    /// the map of definitions (for now).
    #[fail(
        display = "Parameter {:?} in path {:?} defines a new schema, which is unsupported at this point.",
        _0, _1
    )]
    UnsupportedParameterDefinition(String, String),
    /// The type of this parameter is not known.
    #[fail(
        display = "Parameter {:?} in path {:?} doesn't have a known type",
        _0, _1
    )]
    UnknownParameterType(String, String),
    /// I/O errors.
    #[fail(display = "I/O error: {}", _0)]
    Io(std::io::Error),
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

impl_err_from!(PaperClipError::std::io::Error > Io);
impl_err_from!(PaperClipError::serde_json::Error > Json);
impl_err_from!(PaperClipError::serde_yaml::Error > Yaml);
impl_err_from!(PaperClipError::paperclip_core::ValidationError > Validation);
#[cfg(feature = "codegen-fmt")]
impl_err_from!(PaperClipError::rustfmt_nightly::ErrorKind > RustFmt);
