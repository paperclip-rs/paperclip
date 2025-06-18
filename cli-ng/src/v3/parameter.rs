use heck::ToSnakeCase;
use ramhorns_derive::Content;
use std::collections::HashMap;

use super::{property::Property, OpenApiV3};

#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub(crate) struct Parameter {
    param_name: String,
    base_name: String,
    example: Option<String>,
    description: Option<String>,
    examples: Vec<String>,
    required: bool,
    deprecated: Option<bool>,
    is_nullable: bool,
    is_string: bool,
    is_array: bool,
    is_uuid: bool,
    is_uri: bool,
    is_primitive_type: bool,
    is_container: bool,
    data_type: String,
    data_format: String,
    vendor_extensions: HashMap<String, String>,
    items: Option<Box<super::Property>>,
}

impl Parameter {
    /// Create a new Parameter from a request body.
    pub(super) fn from_body(api: &OpenApiV3, body: &openapiv3::RequestBody) -> Option<Self> {
        let (_, media) = body.content.first()?;

        let schema_back;
        let mut body_type = None;
        let schema = match media.schema.as_ref()? {
            openapiv3::ReferenceOr::Reference { reference } => {
                let schema_ref = reference.replace("#/components/schemas/", "");
                let schema = api
                    .api
                    .components
                    .as_ref()
                    .and_then(|c| c.schemas.get(&schema_ref));
                body_type = Some(schema_ref);
                match schema {
                    None => {
                        api.missing_schema_ref(reference);
                        schema_back = openapiv3::Schema {
                            schema_data: Default::default(),
                            schema_kind: openapiv3::SchemaKind::Any(openapiv3::AnySchema::default()),
                        };
                        &schema_back
                    }
                    Some(ref_or) => match ref_or {
                        openapiv3::ReferenceOr::Reference { .. } => {
                            panic!("double reference not supported");
                        }
                        openapiv3::ReferenceOr::Item(schema) => schema,
                    },
                }
            }
            openapiv3::ReferenceOr::Item(schema) => schema,
        };
        let property = Property::from_schema(api, None, schema, None, body_type.as_deref());
        if property.is_any_type() {
            body_type = Some("body".to_string());
        }
        let body_type = body_type.as_deref().unwrap_or("unknown");
        let property = super::OpenApiV3::post_process(property);
        Some(Self {
            // todo: should have snake case param
            param_name: body_type.to_snake_case(),
            base_name: body_type.to_snake_case(),
            example: media.example.as_ref().map(|v| v.to_string()),
            examples: vec![],
            description: body.description.as_ref().map(|s| s.replace('\n', " ")),
            required: body.required,
            deprecated: None,
            is_nullable: schema.schema_data.nullable,
            is_string: property.is_string(),
            is_array: property.is_array(),
            is_uuid: property.is_uuid(),
            is_uri: property.is_uri(),
            is_primitive_type: property.is_primitive_type(),
            is_container: property.is_container(),
            items: property.items().clone(),
            data_type: property.data_type(),
            data_format: property.data_format(),
            vendor_extensions: body
                .extensions
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        })
    }
    /// Create a new Parameter based on the deserialized parameter data.
    pub(super) fn new(api: &OpenApiV3, param: &openapiv3::ParameterData) -> Self {
        let schema_back;
        let mut schema_type = None;
        let schema = match &param.format {
            openapiv3::ParameterSchemaOrContent::Schema(ref_s) => match ref_s {
                openapiv3::ReferenceOr::Reference { reference } => {
                    let schema_ref = reference.replace("#/components/schemas/", "");
                    let schema = api
                        .api
                        .components
                        .as_ref()
                        .and_then(|c| c.schemas.get(&schema_ref));
                    schema_type = Some(schema_ref);
                    match schema {
                        None => {
                            api.missing_schema_ref(reference);
                            schema_back = openapiv3::Schema {
                                schema_data: Default::default(),
                                schema_kind: openapiv3::SchemaKind::Any(
                                    openapiv3::AnySchema::default(),
                                ),
                            };
                            &schema_back
                        }
                        Some(ref_or) => match ref_or {
                            openapiv3::ReferenceOr::Reference { .. } => {
                                panic!("double reference not supported");
                            }
                            openapiv3::ReferenceOr::Item(schema) => schema,
                        },
                    }
                }
                openapiv3::ReferenceOr::Item(schema) => schema,
            },
            openapiv3::ParameterSchemaOrContent::Content(_) => {
                todo!()
            }
        };
        let property =
            Property::from_schema(api, None, schema, Some(&param.name), schema_type.as_deref());
        let property = super::OpenApiV3::post_process(property);
        Self {
            // todo: should have snake case param
            param_name: param.name.to_snake_case(),
            base_name: param.name.clone(),
            example: param.example.as_ref().map(|v| v.to_string()),
            examples: vec![],
            description: param.description.as_ref().map(|s| s.replace('\n', " ")),
            required: param.required,
            deprecated: param.deprecated,
            is_nullable: schema.schema_data.nullable,
            is_string: property.is_string(),
            is_array: property.is_array(),
            is_uuid: property.is_uuid(),
            is_uri: property.is_uri(),
            is_primitive_type: property.is_primitive_type(),
            is_container: property.is_container(),
            items: property.items().clone(),
            data_type: property.data_type(),
            data_format: property.data_format(),
            vendor_extensions: param
                .extensions
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        }
    }
    /// Get a reference to the parameter data type format.
    pub fn data_format(&self) -> &str {
        &self.data_format
    }
    /// Get a reference to the parameter base name (no case modifications).
    pub fn base_name(&self) -> &str {
        &self.base_name
    }
    /// Get a reference to the parameter name.
    pub fn name(&self) -> &str {
        &self.param_name
    }
    /// Is the parameter required or optional.
    pub fn required(&self) -> bool {
        self.required
    }
    /// Returns version extension by key.
    pub fn vendor_extension(&self, key: &str) -> Option<&str> {
        self.vendor_extensions.get(key).map(|x| x.as_str())
    }
}
