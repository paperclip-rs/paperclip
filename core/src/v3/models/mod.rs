mod contact;
mod external_documentation;
mod header;
mod info;
mod license;
mod openapi;
mod operation;
mod parameter;
mod paths;
mod reference;
mod request_body;
mod response;
mod schema;
mod security_scheme;
mod tag;

pub use super::super::v2::{models as v2, models::Either};

use parameter::non_body_parameter_to_v3_parameter;
use reference::invalid_referenceor;
use response::OperationEitherResponse;
