use failure::Fail;

/// Errors related to spec validation.
#[derive(Debug, Fail)]
pub enum ValidationError {
    /// Failed to resolve the schema because an invalid URI was provided for
    /// `$ref` field.
    ///
    /// Currently, we only support `#/definitions/YourType` in `$ref` field.
    #[fail(
        display = "Invalid $ref URI {:?}. Only relative URIs for definitions are supported right now.",
        _0
    )]
    InvalidRefURI(String),
    /// A definition has been referenced but it's missing.
    #[fail(display = "Definition missing: {}", _0)]
    MissingDefinition(String),
    /// If a parameter specifies body, then schema must be specified.
    #[fail(
        display = "Parameter {:?} in path {:?} is a body but the schema is missing",
        _0, _1
    )]
    MissingSchemaForBodyParameter(String, String),
    /// If a parameter doesn't specify a body, then it must have a type.
    #[fail(display = "Parameter {:?} in path {:?} must have a type", _0, _1)]
    MissingParameterType(String, String),
}
