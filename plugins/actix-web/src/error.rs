use thiserror::Error;

#[derive(Debug, Error)]
pub enum SwaggerError {
    #[error("JSON specification not found.\nYou have to run with_json_spec_at(path) first.")]
    JSONSpecificationNotFound,
    #[error("An error occurred while rendering Swagger-UI")]
    SwaggerUIRenderingError,
}
