use std::{collections::HashSet, path::PathBuf};

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
#[derive(Debug, thiserror::Error)]
pub enum PaperClipError {
    /// Error encountered during spec validation.
    #[error("{}", _0)]
    Validation(paperclip_core::ValidationError),
    /// The given directory cannot be used for generating code.
    #[error("Cannot generate code in the given directory")]
    InvalidCodegenDirectory,
    /// Currently, we only support OpenAPI v2, and eventually v3.
    #[error("This version of OpenAPI is unsupported.")]
    #[allow(clippy::upper_case_acronyms)]
    UnsupportedOpenAPIVersion,
    /// Paths listed in the spec must be unique.
    #[error("Path similar to {:?} already exists.", _0)]
    RelativePathNotUnique(String),
    #[error("Parameter(s) {:?} aren't defined for templated path {:?}", _1, _0)]
    MissingParametersInPath(String, HashSet<String>),
    /// Invalid host for URL.
    #[error("Cannot parse host {:?}: {}", _0, _1)]
    InvalidHost(String, url_dep::ParseError),
    /// Invalid base path URL.
    #[error("Cannot set URL {:?}: {}", _0, _1)]
    #[allow(clippy::upper_case_acronyms)]
    InvalidBasePathURL(String, url_dep::ParseError),
    /// The given schema object is an array, but the `items` field is missing.
    #[error("Mising item schema for array: {:?}", _0)]
    MissingArrayItem(Option<String>),
    /// The name for the given definition is invalid.
    #[error("Invalid name for definition: '{0}'")]
    InvalidDefinitionName(String),
    /// The name for the given definition is missing.
    #[error("Missing name for definition")]
    MissingDefinitionName,
    /// A valid path cannot be obtained for the given definition.
    #[error("Invalid path for definition: {:?}", _0)]
    InvalidDefinitionPath(PathBuf),
    /// I/O errors.
    #[error("I/O error: {}", _0)]
    Io(std::io::Error),
    /// JSON coding errors.
    #[error("JSON error: {}", _0)]
    Json(serde_json::Error),
    /// YAML coding errors.
    #[error("YAML error: {}", _0)]
    Yaml(serde_yaml::Error),
    #[cfg(feature = "codegen-fmt")]
    /// Errors from rustfmt.
    #[error("Rustfmt formatting error: {}", _0)]
    RustFmt(rustfmt_nightly::ErrorKind),
    #[cfg(feature = "codegen")]
    /// Errors in templating.
    #[error("Templating error: {}", _0)]
    Templating(tinytemplate::error::Error),
}

impl_err_from!(PaperClipError::std::io::Error > Io);
impl_err_from!(PaperClipError::serde_json::Error > Json);
impl_err_from!(PaperClipError::serde_yaml::Error > Yaml);
impl_err_from!(PaperClipError::paperclip_core::ValidationError > Validation);
#[cfg(feature = "codegen-fmt")]
impl_err_from!(PaperClipError::rustfmt_nightly::ErrorKind > RustFmt);
#[cfg(feature = "codegen")]
impl_err_from!(PaperClipError::tinytemplate::error::Error > Templating);
