use std::collections::HashMap;

use super::{OpenApiV3, Parameter, Property};

use heck::{ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use itertools::Itertools;
use ramhorns_derive::Content;

use log::debug;

#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub(crate) struct Operation {
    classname: String,
    class_filename: String,

    response_headers: Vec<Property>,

    return_type_is_primitive: bool,
    return_simple_type: bool,
    subresource_operation: bool,
    is_multipart: bool,
    is_response_binary: bool,
    is_response_file: bool,
    is_response_optional: bool,
    has_reference: bool,
    is_restful_index: bool,
    is_restful_show: bool,
    is_restful_create: bool,
    is_restful_update: bool,
    is_restful_destroy: bool,
    is_restful: bool,
    is_deprecated: Option<bool>,
    is_callback_request: bool,
    unique_items: bool,
    has_default_response: bool,
    // if 4xx, 5xx responses have at least one error object defined
    has_error_response_object: bool,

    path: String,
    operation_id: Option<String>,
    return_type: Option<String>,
    return_format: String,
    http_method: String,
    return_base_type: String,
    return_container: String,
    summary: Option<String>,
    unescaped_notes: String,
    basename: String,
    default_response: String,

    consumes: Vec<std::collections::HashMap<String, String>>,
    has_consumes: bool,
    produces: Vec<std::collections::HashMap<String, String>>,
    has_produces: bool,
    prioritized_content_types: Vec<std::collections::HashMap<String, String>>,

    all_params: Vec<Parameter>,
    has_params: bool,
    path_params: Vec<Parameter>,
    has_path_params: bool,
    query_params: Vec<Parameter>,
    has_query_params: bool,
    header_params: Vec<Parameter>,
    has_header_params: bool,
    has_body_param: bool,
    body_param: Option<Parameter>,
    implicit_headers_params: Vec<Parameter>,
    has_implicit_headers_params: bool,
    form_params: Vec<Parameter>,
    has_form_params: bool,
    required_params: Vec<Parameter>,
    has_required_params: bool,
    optional_params: Vec<Parameter>,
    has_optional_params: bool,
    auth_methods: Vec<AuthMethod>,
    pub(crate) has_auth_methods: bool,

    tags: Vec<String>,
    responses: Vec<()>,
    callbacks: Vec<()>,

    examples: Vec<HashMap<String, String>>,
    request_body_examples: Vec<HashMap<String, String>>,

    vendor_extensions: HashMap<String, String>,

    pub(crate) operation_id_original: Option<String>,
    operation_id_camel_case: Option<String>,
    operation_id_lower_case: Option<String>,
    support_multiple_responses: bool,

    description: Option<String>,

    api_doc_path: &'static str,
    model_doc_path: &'static str,
}

fn query_param(api: &OpenApiV3, value: &openapiv3::Parameter) -> Option<Parameter> {
    match value {
        openapiv3::Parameter::Query { parameter_data, .. } => {
            let parameter = Parameter::new(api, parameter_data);
            Some(parameter)
        }
        _ => None,
    }
}
fn path_param(api: &OpenApiV3, value: &openapiv3::Parameter) -> Option<Parameter> {
    match value {
        openapiv3::Parameter::Path { parameter_data, .. } => {
            let parameter = Parameter::new(api, parameter_data);
            Some(parameter)
        }
        _ => None,
    }
}
#[allow(unused)]
fn header_param(api: &OpenApiV3, value: &openapiv3::Parameter) -> Option<Parameter> {
    match value {
        openapiv3::Parameter::Header { parameter_data, .. } => {
            let parameter = Parameter::new(api, parameter_data);
            Some(parameter)
        }
        _ => None,
    }
}
fn body_param(api: &OpenApiV3, value: &openapiv3::RequestBody) -> Option<Parameter> {
    Parameter::from_body(api, value)
}

impl Operation {
    /// Create an Operation based on the deserialized openapi operation.
    pub(crate) fn new(
        root: &OpenApiV3,
        path: &str,
        method: &str,
        operation: &openapiv3::Operation,
    ) -> Self {
        debug!(
            "Operation::{id:?} => {method}::{path}::{tags:?}",
            id = operation.operation_id,
            tags = operation.tags
        );
        let mut vendor_extensions = operation
            .extensions
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect::<HashMap<_, _>>();

        vendor_extensions.insert("x-httpMethodLower".into(), method.to_ascii_lowercase());
        vendor_extensions.insert("x-httpMethodUpper".into(), method.to_ascii_uppercase());

        let query_params = operation
            .parameters
            .iter()
            .flat_map(|p| {
                match p {
                    // todo: need to handle this
                    openapiv3::ReferenceOr::Reference { .. } => todo!(),
                    openapiv3::ReferenceOr::Item(item) => query_param(root, item),
                }
            })
            .collect::<Vec<_>>();
        let path_params = operation
            .parameters
            .iter()
            .flat_map(|p| {
                match p {
                    // todo: need to handle this
                    openapiv3::ReferenceOr::Reference { .. } => todo!(),
                    openapiv3::ReferenceOr::Item(item) => path_param(root, item),
                }
            })
            .sorted_by(|a, b| b.required().cmp(&a.required()))
            .collect::<Vec<_>>();
        let body_param = operation.request_body.as_ref().and_then(|p| {
            match p {
                // todo: need to handle this
                openapiv3::ReferenceOr::Reference { .. } => todo!(),
                openapiv3::ReferenceOr::Item(item) => body_param(root, item),
            }
        });

        let mut ext_path = path.to_string();
        for param in &path_params {
            if param.vendor_extension("x-actix-tail-match") == Some("true") {
                ext_path = ext_path.replace(param.name(), &format!("{}:.*", param.base_name()));
            } else if param.data_format() == "url" {
                ext_path = ext_path.replace(param.name(), &format!("{}:.*", param.base_name()));
                vendor_extensions.insert("x-actix-query-string".into(), "true".into());
            }
        }
        vendor_extensions.insert("x-actixPath".into(), ext_path);

        let all_params = path_params
            .iter()
            .chain(
                query_params
                    .iter()
                    .sorted_by(|a, b| b.required().cmp(&a.required())),
            )
            .chain(&body_param)
            .cloned()
            .collect::<Vec<_>>();
        // todo: support multiple responses
        let return_model = match operation
            .responses
            .responses
            .get(&openapiv3::StatusCode::Code(200))
            .or(operation
                .responses
                .responses
                .get(&openapiv3::StatusCode::Code(204)))
        {
            Some(ref_or) => root.resolve_reference_or_resp("application/json", ref_or),
            None => todo!(),
        };
        // todo: should we post process after all operations are processed?
        let return_model = return_model.post_process_data_type();
        let (class, class_file) = match operation.tags.first() {
            Some(class) => (class.clone(), format!("{class}_api").to_snake_case()),
            // How should this be handled? Shuld it be required? What if there's more than 1 tag?
            None => (String::new(), String::new()),
        };
        Self {
            description: operation.description.as_ref().map(|d| d.replace('\n', " ")),
            classname: class,
            class_filename: class_file,
            summary: operation.summary.clone(),
            tags: operation.tags.clone(),
            is_deprecated: Some(operation.deprecated),
            operation_id_lower_case: operation.operation_id.as_ref().map(|o| o.to_lowercase()),
            operation_id_camel_case: operation
                .operation_id
                .as_ref()
                .map(|o| o.to_lower_camel_case()),
            operation_id: operation.operation_id.clone(),
            operation_id_original: operation.operation_id.clone(),
            has_params: !all_params.is_empty(),
            all_params,
            has_path_params: !path_params.is_empty(),
            path_params,
            has_query_params: !query_params.is_empty(),
            query_params,
            header_params: vec![],
            has_header_params: false,
            has_body_param: body_param.is_some(),
            body_param,
            path: path.to_string(),
            http_method: method.to_upper_camel_case(),
            support_multiple_responses: false,
            return_type: {
                let data_type = return_model.data_type();
                if data_type == "()" {
                    None
                } else {
                    Some(data_type)
                }
            },
            has_auth_methods: operation.security.is_some(),
            auth_methods: match &operation.security {
                None => vec![],
                Some(sec) => sec
                    .iter()
                    .flat_map(|a| {
                        a.iter()
                            .map(|(key, _)| match key.as_str() {
                                "JWT" => AuthMethod {
                                    scheme: "JWT".to_string(),
                                    is_basic: true,
                                    is_basic_bearer: true,
                                },
                                scheme => AuthMethod {
                                    scheme: scheme.to_string(),
                                    is_basic: false,
                                    is_basic_bearer: false,
                                },
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            },
            vendor_extensions,
            api_doc_path: "docs/apis/",
            model_doc_path: "docs/models/",
            ..Default::default()
        }
    }
    /// Get a reference to the operation tags list.
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    /// Get a reference to the operation class name.
    pub fn classname(&self) -> &str {
        &self.classname
    }
    /// Get a reference to the operation class filename.
    pub fn class_filename(&self) -> &str {
        &self.class_filename
    }
}

#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
struct AuthMethod {
    scheme: String,

    is_basic: bool,
    is_basic_bearer: bool,
}
