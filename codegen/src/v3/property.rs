use std::{collections::HashMap, fmt::Display, ops::Deref, rc::Rc};

use heck::{ToSnakeCase, ToUpperCamelCase};
use ramhorns_derive::Content;

/// The various openapi v3 property data types.
#[derive(Clone, Debug)]
pub(crate) enum PropertyDataType {
    Unknown,
    Resolved(String, Option<String>),
    Any,
    RawString,
    String(openapiv3::StringType),
    Enum(String, String),
    Boolean,
    Integer(openapiv3::IntegerType),
    Number(openapiv3::NumberType),
    Model(String),
    DiscModel(String, String),
    Map(Box<PropertyDataType>, Box<PropertyDataType>),
    Array(Box<PropertyDataType>),
    Empty,
}

impl Display for PropertyDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl PropertyDataType {
    fn as_str(&self) -> &str {
        match self {
            PropertyDataType::Unknown => "Unkown",
            PropertyDataType::Resolved(inner, _) => inner.as_ref(),
            PropertyDataType::Array(inner) => inner.as_str(),
            PropertyDataType::Any => "Any",
            PropertyDataType::Map(_, _) => "Map",
            PropertyDataType::RawString => "String",
            PropertyDataType::String(_) => "String",
            PropertyDataType::Enum(_, _) => "Enum",
            PropertyDataType::Boolean => "bool",
            PropertyDataType::Integer(_) => "integer",
            PropertyDataType::Number(_) => "number",
            PropertyDataType::Model(inner) => inner.as_str(),
            PropertyDataType::DiscModel(_, _) => "Disc",
            PropertyDataType::Empty => "Empty",
        }
    }
    fn format(&self) -> Option<&String> {
        match self {
            PropertyDataType::Resolved(_, format) => format.as_ref(),
            _ => None,
        }
    }
    fn resolve(&mut self, data_type: &str) {
        self.set_if_unresolved(Self::Resolved(data_type.into(), None));
    }
    fn resolve_format<T: Into<String>>(&mut self, data_type: &str, format: T) {
        self.set_if_unresolved(Self::Resolved(data_type.into(), Some(format.into())));
    }
    fn resolve_format_opt(&mut self, data_type: &str, format: Option<String>) {
        self.set_if_unresolved(Self::Resolved(data_type.into(), format));
    }
    fn set_string(&mut self, data_type: &openapiv3::StringType) {
        self.set_if_unresolved(Self::String(data_type.clone()));
    }
    fn set_array(&mut self, data_type: &Self) {
        self.set_if_unresolved(Self::Array(Box::new(data_type.clone())));
    }
    fn set_boolean(&mut self) {
        self.set_if_unresolved(Self::Boolean);
    }
    fn set_integer(&mut self, data_type: &openapiv3::IntegerType) {
        self.set_if_unresolved(Self::Integer(data_type.clone()));
    }
    fn set_number(&mut self, data_type: &openapiv3::NumberType) {
        self.set_if_unresolved(Self::Number(data_type.clone()));
    }
    fn set_model(&mut self, data_type: &str) {
        self.set_if_unresolved(Self::Model(data_type.to_string()));
    }
    fn set_disc_model(&mut self, parent: String, name: &str) {
        self.set_if_unresolved(Self::DiscModel(parent, name.to_string()));
    }
    fn set_map(&mut self, key: &Self, value: &Self) {
        self.set_if_unresolved(Self::Map(Box::new(key.clone()), Box::new(value.clone())));
    }
    fn set_enum(&mut self, name: &str, data_type: &str) {
        self.set_if_unresolved(Self::Enum(name.to_string(), data_type.to_string()));
    }
    fn set_any(&mut self) {
        self.set_if_unresolved(Self::Any);
    }
    fn set_if_unresolved(&mut self, to: Self) {
        if !matches!(self, Self::Resolved(_, _)) {
            *self = to;
        }
    }
}

impl Default for PropertyDataType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl ramhorns::Content for PropertyDataType {
    #[inline]
    fn is_truthy(&self) -> bool {
        !self.as_str().is_empty()
    }

    #[inline]
    fn capacity_hint(&self, _tpl: &ramhorns::Template) -> usize {
        self.as_str().len()
    }

    #[inline]
    fn render_escaped<E: ramhorns::encoding::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), E::Error> {
        encoder.write_escaped(self.as_str())
    }

    #[inline]
    fn render_unescaped<E: ramhorns::encoding::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), E::Error> {
        encoder.write_unescaped(self.as_str())
    }
}

/// A list of properties.
pub(crate) type Properties = Vec<Property>;

/// An OpenApiV3 property of a Schema Object.
/// https://spec.openapis.org/oas/v3.0.3#properties
/// Including fixed fields, composition, etc.
/// These fields are used for both managing the template generation as well as input for
/// the templates themselves.
#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub(crate) struct Property {
    // The schema name as written in the OpenAPI document.
    name: String,

    // The language-specific name of the "class" that implements this schema.
    // The name of the class is derived from the OpenAPI schema name with formatting rules applied.
    // The classname is derived from the OpenAPI schema name, with sanitization and escaping rules
    // applied.
    pub classname: String,
    schema_name: String,
    class_filename: String,

    base_name: String,
    enum_name: Option<String>,
    // The value of the 'title' attribute in the OpenAPI document.
    title: Option<String>,
    description: Option<String>,
    example: Option<String>,
    class_var_name: String,
    model_json: String,
    data_type: PropertyDataType,
    data_format: String,
    /// The type_ coming from component schema.
    type_: String,
    unescaped_description: String,

    /// Booleans for is_$-like type checking.
    is_string: bool,
    is_integer: bool,
    is_long: bool,
    is_number: bool,
    is_numeric: bool,
    is_float: bool,
    is_double: bool,
    is_date: bool,
    is_date_time: bool,
    is_password: bool,
    is_decimal: bool,
    is_binary: bool,
    is_byte: bool,
    is_short: bool,
    is_unbounded_integer: bool,
    is_primitive_type: bool,
    is_boolean: bool,
    is_uuid: bool,
    is_any_type: bool,
    is_enum: bool,
    is_array: bool,
    is_container: bool,
    is_map: bool,
    is_null: bool,
    is_var: bool,

    /// Indicates whether additional properties has defined this as an Any type.
    additional_properties_is_any_type: bool,

    /// If Self is an object, these are all its child properties.
    vars: Properties,
    /// And this? Inludes the parent properties? What does this mean?
    all_vars: Properties,

    /// These could be "special" ramhorn methods rather than fields to avoid copy.
    /// Only the required properties.
    required_vars: Properties,
    /// Only the optional properties.
    optional_vars: Properties,
    // Only the read-only properties.
    read_only_vars: Properties,
    // The read/write properties.
    read_write_vars: Properties,
    /// The Self's parent properties.
    parent_vars: Properties,

    /// If this is an enum, all the allowed values.
    allowable_values: HashMap<String, Vec<EnumValue>>,

    /// If this is an array, the inner property of each index.
    items: Option<Box<Property>>,

    /// Indicates whether Self has child variables or not.
    has_vars: bool,
    /// Indicates whether there are enpty vars? What does this mean?
    empty_vars: bool,
    has_enums: bool,
    /// Validation rules? Like patterns?
    has_validation: bool,
    /// Indicates the OAS schema specifies "nullable: true".
    is_nullable: bool,
    /// Indicates the type has at least one required property.
    has_required: bool,
    /// Indicates the type has at least one optional property.
    has_optional: bool,
    /// Indicates wether we have children vars? Or are these for inline schemas/properties?
    has_children: bool,

    is_deprecated: bool,
    has_only_read_only: bool,
    required: bool,
    max_properties: Option<usize>,
    min_properties: Option<usize>,
    unique_items: bool,
    max_items: Option<usize>,
    min_items: Option<usize>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    exclusive_minimum: bool,
    exclusive_maximum: bool,
    minimum: Option<String>,
    maximum: Option<String>,
    pattern: Option<String>,

    /// If we are a schema defined model?
    is_model: bool,
    /// If we are a component model defined in the root component schemas: #/components/schemas.
    is_component_model: bool,

    one_of: Properties,
    all_of: Properties,

    /// Inline models discovered through the schema of this very model.
    discovered_props: Rc<Properties>,

    /// The parent property of this property, if this property is defined "inline" as an Item or a class member or item.
    parent: Option<Rc<Property>>,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}.rs",
            self.data_type(),
            self.classname,
            self.class_filename
        )
    }
}

impl Property {
    /// Mutate the inner properties with the OpenAPI `openapiv3::SchemaData`.
    pub fn with_data(mut self, data: &openapiv3::SchemaData) -> Self {
        self.is_null = data.nullable;
        self.is_nullable = data.nullable;
        self.is_deprecated = data.deprecated;
        self.title = data.title.clone();
        self.description = data
            .description
            .as_ref()
            .map(|s| s.escape_default().to_string().replace("\\n", " "));
        self.example = data.example.as_ref().map(ToString::to_string);
        self
    }
    /// Set wether the property is a model or not.
    pub fn with_model(mut self, model: bool) -> Self {
        self.is_model = model;
        self
    }
    /// Set wether the property is a component model or not.
    pub fn with_component_model(mut self, root_model: bool) -> Self {
        if root_model {
            self.is_component_model = true;
        }
        self
    }
    /// Get a reference to the property type.
    pub fn type_ref(&self) -> &str {
        &self.type_
    }
    /// Get the property data type.
    pub fn data_type(&self) -> String {
        self.data_type.to_string()
    }
    /// Get the property data format.
    pub fn data_format(&self) -> String {
        self.data_type.format().map(Into::into).unwrap_or_default()
    }
    /// Get the class filename, if the property is a model.
    pub fn filename(&self) -> &str {
        self.class_filename.as_str()
    }
    /// Set the property data type.
    pub fn with_data_property(mut self, type_: &PropertyDataType) -> Self {
        self.data_type = type_.clone();
        self
    }
    /// Set the model type.
    pub fn with_model_type(mut self, type_: &str) -> Self {
        match self.parent() {
            Some(parent) if type_.is_empty() => {
                let parent_type = parent.type_.clone();
                self.data_type.set_disc_model(parent_type, &self.name);
            }
            _ => {
                self.data_type.set_model(type_);
            }
        }
        self
    }
    /// Set the data type Any, and if there's additional properties.
    fn with_data_type_any(mut self, is_add_props: bool) -> Self {
        self.data_type.set_any();
        self.is_any_type = true;
        self.additional_properties_is_any_type = is_add_props;
        self
    }
    /// Set the property type.
    pub fn with_type(mut self, type_: &str) -> Self {
        self.type_ = type_.to_string();
        self
    }
    /// The property is an OpenAPI AllOf, composed of a single property.
    /// (This is because multiple properties is not supported yet)
    pub fn with_one_all_of(self, single: Property) -> Self {
        self.with_name(&single.name)
            .with_type(&single.type_)
            .with_data_property(&single.data_type)
            .with_model(true)
            .with_parent(&Some(&single))
            .with_all_of(vec![single])
    }
    fn with_all_of(mut self, all_of: Vec<Property>) -> Self {
        self.all_of = all_of;
        self
    }
    /// Get a reference to the list of properties discovered through this property.
    fn discovered_props(&self) -> &Vec<Property> {
        &self.discovered_props
    }
    /// Similar as `discovered_props` but filters for models and applied recursively.
    pub fn discovered_models(&self) -> Vec<Property> {
        self.discovered_props()
            .iter()
            .flat_map(|m| {
                let mut v = m.discovered_models();
                v.push(m.clone());
                v
            })
            .filter(|p| !p.is_component_model && p.is_model && !p.is_all_of() && !p.is_enum)
            .collect::<Vec<_>>()
    }
}
impl From<&openapiv3::SchemaData> for Property {
    fn from(data: &openapiv3::SchemaData) -> Self {
        Self::default().with_data(data)
    }
}

impl Property {
    /// Create a `Property` from an OpenAPI schema, with some other information.
    pub fn from_schema(
        root: &super::OpenApiV3,
        parent: Option<&Property>,
        schema: &openapiv3::Schema,
        name: Option<&str>,
        type_: Option<&str>,
    ) -> Self {
        let name = name.unwrap_or_default();
        let type_ = type_.unwrap_or_default();
        trace!("PropertyFromSchema: {}/{}", name, type_);
        let prop = Property::from(&schema.schema_data)
            .with_name(name)
            .with_parent(&parent)
            .with_type(type_)
            .with_component_model(root.contains_schema(type_));

        prop.with_kind(root, schema, &schema.schema_kind, parent, name, type_)
    }

    fn with_kind(
        mut self,
        root: &super::OpenApiV3,
        schema: &openapiv3::Schema,
        schema_kind: &openapiv3::SchemaKind,
        parent: Option<&Self>,
        name: &str,
        type_: &str,
    ) -> Self {
        match schema_kind {
            openapiv3::SchemaKind::Type(t) => match t {
                openapiv3::Type::String(t) => self.with_string(root, t),
                openapiv3::Type::Number(t) => self.with_number(root, t),
                openapiv3::Type::Integer(t) => self.with_integer(root, t),
                openapiv3::Type::Object(t) => self.with_model_type(type_).with_obj(root, t),
                openapiv3::Type::Array(t) => self.with_array(root, t),
                openapiv3::Type::Boolean(_) => {
                    self.data_type.set_boolean();
                    self.is_boolean = true;
                    self.is_primitive_type = true;
                    self
                }
            },
            openapiv3::SchemaKind::OneOf { .. } => {
                panic!("OneOf: {:#?} not implemented", schema);
            }
            openapiv3::SchemaKind::AllOf { all_of } if all_of.len() != 1 => {
                unimplemented!()
            }
            openapiv3::SchemaKind::AllOf { all_of } => {
                let first = all_of.first().unwrap();
                let first_model = root
                    .resolve_reference_or(first, parent, Some(name), None)
                    .with_data(&schema.schema_data);
                Self::from(&schema.schema_data).with_one_all_of(first_model)
            }
            openapiv3::SchemaKind::AnyOf { .. } => {
                unimplemented!()
            }
            openapiv3::SchemaKind::Not { .. } => {
                unimplemented!()
            }
            // In some cases, we get Any rather than a specific kind :(
            // For more info: https://github.com/glademiller/openapiv3/pull/79
            // todo: this needs a lot of tweaking...
            openapiv3::SchemaKind::Any(any_schema) => match &any_schema.typ {
                Some(typ) => match typ.as_str() {
                    "bool" => {
                        let kind = openapiv3::SchemaKind::Type(openapiv3::Type::Boolean(
                            openapiv3::BooleanType {
                                enumeration: vec![],
                            },
                        ));
                        self.with_kind(root, schema, &kind, parent, name, type_)
                    }
                    "object" => self.with_model_type(type_).with_anyobj(root, any_schema),
                    not_handled => {
                        // See above, we must handle all types in the match :(
                        error!("BUG - must handle {not_handled} data type as AnySchema");
                        self.with_data_type_any(false)
                    }
                },
                // not sure how to handle this? default to Any for now.
                None => self.with_data_type_any(false),
            },
        }
    }

    fn assign_classnames(&mut self) {
        if self.classname.is_empty() && self.is_model && !self.is_var {
            let schema_name = self.data_type.as_str();
            self.class_filename = schema_name.to_snake_case();
            self.classname = schema_name.to_upper_camel_case();
        }
        self.assign_enumnames();
    }
    fn assign_varnames(&mut self) {
        if !self.name.is_empty() {
            self.name = self.name.to_snake_case();
        }
    }
    fn assign_enumnames(&mut self) {
        if self.is_enum {
            self.enum_name = Some(self.data_type());
        }
    }
    fn string_format_str(format: openapiv3::StringFormat) -> &'static str {
        match format {
            openapiv3::StringFormat::Date => "date",
            openapiv3::StringFormat::DateTime => "date-time",
            openapiv3::StringFormat::Password => "password",
            openapiv3::StringFormat::Byte => "byte",
            openapiv3::StringFormat::Binary => "binary",
        }
    }
    // This can be provided for a way of custumizing the types.
    fn post_process_dt(data_type: &mut PropertyDataType, is_decl: bool) {
        match data_type.clone() {
            PropertyDataType::Unknown => {}
            PropertyDataType::Resolved(_, _) => {}
            PropertyDataType::Any => data_type.resolve("serde_json::Value"),
            PropertyDataType::RawString => data_type.resolve("String"),
            PropertyDataType::String(str) => {
                match str.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(format) => {
                        // todo: handle these formats
                        data_type.resolve_format("String", Self::string_format_str(format));
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Unknown(format) => match format.as_str() {
                        "uuid" => data_type.resolve("uuid::Uuid"),
                        _ => data_type.resolve_format("String", format),
                    },
                    openapiv3::VariantOrUnknownOrEmpty::Empty => {
                        data_type.resolve("String");
                    }
                }
            }
            PropertyDataType::Enum(name, type_) if !is_decl => {
                let enum_ = if type_.is_empty() { name } else { type_ }.to_upper_camel_case();
                data_type.resolve(&format!("crate::models::{enum_}"))
            }
            PropertyDataType::Enum(name, type_) => {
                let enum_ = if type_.is_empty() { name } else { type_ }.to_upper_camel_case();
                data_type.resolve(&enum_)
            }
            PropertyDataType::Boolean => data_type.resolve("bool"),
            PropertyDataType::Integer(type_) => {
                let (signed, bits, format) = match type_.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(item) => match item {
                        openapiv3::IntegerFormat::Int32 => (true, 32, Some("int32".into())),
                        openapiv3::IntegerFormat::Int64 => (true, 64, Some("int64".into())),
                    },
                    openapiv3::VariantOrUnknownOrEmpty::Unknown(format) => match format.as_str() {
                        "uint32" => (false, 32, Some(format)),
                        "uint64" => (false, 64, Some(format)),
                        "int16" => (true, 16, Some(format)),
                        "uint16" => (false, 16, Some(format)),
                        "int8" => (true, 8, Some(format)),
                        "uint8" => (false, 8, Some(format)),
                        _ => (true, 0, Some(format)),
                    },
                    _ => (true, 0, None),
                };
                let signed = type_.minimum.map(|m| m < 0).unwrap_or(signed);

                // no format specified
                let bits = if bits == 0 {
                    "size".to_string()
                } else {
                    bits.to_string()
                };

                // todo: check min and max
                data_type.resolve_format_opt(
                    &format!("{}{}", if signed { "i" } else { "u" }, bits),
                    format,
                )
            }
            PropertyDataType::Number(type_) => {
                data_type.resolve(match type_.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Float) => {
                        "f32"
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => {
                        "f64"
                    }
                    openapiv3::VariantOrUnknownOrEmpty::Unknown(_) => "f64",
                    openapiv3::VariantOrUnknownOrEmpty::Empty => "f64",
                });
            }
            PropertyDataType::Model(model) if !is_decl => {
                data_type.resolve(&format!("crate::models::{model}"))
            }
            PropertyDataType::Model(model) => data_type.resolve(&model),
            PropertyDataType::DiscModel(parent, this) => {
                let this = this.to_upper_camel_case();
                let parent = parent.to_upper_camel_case();
                if is_decl {
                    data_type.resolve(&format!("{parent}{this}"));
                } else {
                    data_type.resolve(&format!("crate::models::{parent}{this}"));
                }
            }
            PropertyDataType::Map(key, mut value) => {
                Self::post_process_dt(&mut value, false);
                data_type.resolve(&format!(
                    "::std::collections::HashMap<{}, {}>",
                    key.as_ref(),
                    value.as_str()
                ))
            }
            PropertyDataType::Array(mut inner) => {
                Self::post_process_dt(&mut inner, is_decl);
                data_type.resolve(&format!("Vec<{}>", inner.as_str()))
            }
            PropertyDataType::Empty => data_type.resolve("()"),
        }
    }
    /// This is a specific template hack, basically pretends this is not an enum
    /// preventing it from being declared in the same module as the property where it was defined.
    pub(crate) fn uninline_enums(&mut self) {
        if self.is_var && self.is_component_model && self.is_enum {
            // this is a very specific template hack?
            self.is_enum = false;
        }
    }
    /// Processes the data type for usage.
    /// Properties which are not discovered at the top (eg: discovered via reference schema) get
    /// a code import prefix added to them.
    pub fn post_process(mut self) -> Property {
        self.post_process_refmut();
        self
    }
    /// Process the data type for a non-declaration usage.
    /// The property **will** get the code import prefix added.
    pub fn post_process_data_type(mut self) -> Property {
        Self::post_process_dt(&mut self.data_type, false);
        self
    }
    fn post_process_refmut(&mut self) {
        // 1. setup data type, eg: add crate::models:: prefix for import.
        // This is not required if the type is declared in the same module which currently is only
        // true for enums.
        let mut is_decl = !self.is_var && !self.is_container;
        if self.is_var && !self.is_component_model && self.is_enum {
            is_decl = true;
        }
        Self::post_process_dt(&mut self.data_type, is_decl);

        // 2. fixup classname/type of non-enums defined within a type using Item
        self.assign_classnames();
        // 3. setup var names to be snake case
        self.assign_varnames();

        // 4. Uninline enums to avoid inline code generation.
        // todo: template itself should do this!?
        self.uninline_enums();

        // 5. apply the same logic for variables within this object.
        for var in &mut self.vars {
            var.post_process_refmut();
        }
        for var in &mut self.required_vars {
            var.post_process_refmut();
        }
        for var in &mut self.optional_vars {
            var.post_process_refmut();
        }
        for var in &mut self.all_vars {
            var.post_process_refmut();
        }
        for var in &mut self.all_of {
            var.post_process_refmut();
        }
        for var in &mut self.one_of {
            var.post_process_refmut();
        }
        if let Some(item) = &mut self.items {
            item.post_process_refmut();
        }
    }

    fn parent(&self) -> Option<&Self> {
        match &self.parent {
            None => None,
            Some(parent) => Some(parent.deref()),
        }
    }
    /// Get a reference to the inner type of the collection.
    pub fn items(&self) -> &Option<Box<Self>> {
        &self.items
    }
    /// Extend property with a new name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.base_name = name.to_string();
        self
    }
    /// Extend property with a new is_var boolean.
    fn with_is_var(mut self, is_var: bool) -> Self {
        self.is_var = is_var;
        self
    }
    /// Get a reference to the schema's data type.
    /// # Warning: will panic if there is no data type (bug).
    pub fn schema(&self) -> &str {
        if self.data_type.as_str().is_empty() {
            panic!("Schema data type should not be empty! Schema: {:#?}", self);
        }
        self.data_type.as_str()
    }
    /// Extend property with a new is_var boolean.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    /// Extend property with a new parent property.
    pub fn with_parent(mut self, parent: &Option<&Self>) -> Self {
        self.parent = parent.map(|p| Rc::new(p.clone()));
        self
    }
    /// Check if the property is a model.
    pub fn is_model(&self) -> bool {
        self.is_model
    }
    /// Check if the property is a string.
    pub fn is_string(&self) -> bool {
        self.is_string
    }
    /// Check if the property is an array.
    pub fn is_array(&self) -> bool {
        self.is_array
    }
    /// Check if the property is a string uuid.
    pub fn is_uuid(&self) -> bool {
        self.is_uuid
    }
    /// Check if the property is a container.
    pub fn is_container(&self) -> bool {
        self.is_container
    }
    /// Check if the property is a primitive type.
    pub fn is_primitive_type(&self) -> bool {
        self.is_primitive_type
    }
    /// Check if the property is an AllOf.
    pub fn is_all_of(&self) -> bool {
        !self.all_of.is_empty()
    }
    fn with_array(mut self, _root: &super::OpenApiV3, by: &openapiv3::ArrayType) -> Self {
        self.items = by
            .items
            .clone()
            .map(|i| _root.resolve_reference_or(&i.unbox(), Some(&self), None, None))
            .map(|i| i.with_is_var(true))
            .map(Box::new);
        self.min_items = by.min_items;
        self.max_items = by.max_items;
        self.unique_items = by.unique_items;
        self.is_array = true;
        match &self.items {
            Some(items) => {
                self.data_type.set_array(&items.data_type);
            }
            None => {
                panic!("BUG: an array without an inner type: {:?}", self);
            }
        }
        self.is_container = true;
        self
    }
    fn with_anyobj(mut self, root: &super::OpenApiV3, by: &openapiv3::AnySchema) -> Self {
        self.min_properties = by.min_properties;
        self.max_properties = by.max_properties;

        self.is_model = true;

        let vars = by
            .properties
            .iter()
            .map(|(k, v)| root.resolve_reference_or(&v.clone().unbox(), Some(&self), Some(k), None))
            .map(|m| {
                let required = by.required.contains(&m.name);
                m.with_required(required)
            })
            .collect::<Vec<_>>();

        let vars = vars
            .into_iter()
            .map(|p| p.with_is_var(true))
            .collect::<Vec<_>>();

        self.required_vars = vars
            .iter()
            .filter(|m| m.required)
            .cloned()
            .collect::<Vec<_>>();
        self.optional_vars = vars
            .iter()
            .filter(|m| !m.required)
            .cloned()
            .collect::<Vec<_>>();
        self.vars = vars;

        let mut vars_ = self.vars.iter().filter(|p| !p.required).collect::<Vec<_>>();
        if vars_.len() != self.vars.len() {
            panic!("Not Supported - all vars of oneOf must be optional");
        }

        let one_of = &by.one_of;
        one_of
            .iter()
            .flat_map(|p| p.as_item())
            .map(|s| match &s.schema_kind {
                openapiv3::SchemaKind::Any(schema) => schema,
                _ => todo!(),
            })
            .filter(|o| o.required.len() == 1)
            .for_each(|o| vars_.retain(|v| v.name != o.required[0]));

        self.one_of = vec![self.clone()];
        self
    }
    fn with_obj(mut self, root: &super::OpenApiV3, by: &openapiv3::ObjectType) -> Self {
        self.min_properties = by.min_properties;
        self.max_properties = by.max_properties;

        if let Some(props) = &by.additional_properties {
            match props {
                openapiv3::AdditionalProperties::Any(any) => {
                    if *any {
                        return self.with_data_type_any(*any);
                    }
                }
                openapiv3::AdditionalProperties::Schema(ref_or) => match ref_or.deref() {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        let inner = root.resolve_schema_name(None, reference);
                        self.data_type
                            .set_map(&PropertyDataType::RawString, &inner.data_type);
                        self.discovered_props = Rc::new(vec![inner]);
                        return self;
                    }
                    openapiv3::ReferenceOr::Item(item) => {
                        let property = Self::from_schema(root, None, item, None, None);
                        self.data_type
                            .set_map(&PropertyDataType::RawString, &property.data_type);
                        return self;
                    }
                },
            }
        }

        if !root.resolving(&self) {
            let vars = by
                .properties
                .iter()
                .map(|(k, v)| {
                    root.resolve_reference_or(&v.clone().unbox(), Some(&self), Some(k), None)
                })
                .map(|m| {
                    let required = by.required.contains(&m.name);
                    m.with_required(required)
                })
                .collect::<Vec<_>>();

            if vars.is_empty() {
                return self.with_data_type_any(false);
            }
            self.is_model = true;

            self.discovered_props = Rc::new(vars.clone());
            let vars = vars
                .into_iter()
                .map(|p| p.with_is_var(true))
                .collect::<Vec<_>>();

            self.required_vars = vars
                .iter()
                .filter(|m| m.required)
                .cloned()
                .collect::<Vec<_>>();
            self.optional_vars = vars
                .iter()
                .filter(|m| !m.required)
                .cloned()
                .collect::<Vec<_>>();
            self.vars = vars;
        } else {
            // it's a circular reference, we must be a model
            self.is_model = true;
        }

        // if let Some(one_of) = &by.one_of {
        //     let mut vars_ = self.vars.iter().filter(|p| !p.required).collect::<Vec<_>>();
        //     if vars_.len() != self.vars.len() {
        //         panic!("Not Supported - all vars of oneOf must be optional");
        //     }
        //     one_of
        //         .iter()
        //         .flat_map(|p| p.as_item())
        //         .filter(|o| o.required.len() == 1)
        //         .for_each(|o| vars_.retain(|v| v.name != o.required[0]));
        //     if vars_.is_empty() {
        //         self.one_of = vec![self.clone()];
        //     } else {
        //         panic!("OneOf with incorrect combination of required fields");
        //     }
        // }
        self
    }
    fn with_integer(mut self, _root: &super::OpenApiV3, by: &openapiv3::IntegerType) -> Self {
        self.exclusive_maximum = by.exclusive_maximum;
        self.exclusive_minimum = by.exclusive_minimum;
        self.minimum = by.minimum.map(|v| v.to_string());
        self.maximum = by.maximum.map(|v| v.to_string());
        self.is_integer = true;
        self.is_primitive_type = true;
        self.data_type.set_integer(by);
        self
    }
    fn with_number(mut self, _root: &super::OpenApiV3, by: &openapiv3::NumberType) -> Self {
        self.exclusive_maximum = by.exclusive_maximum;
        self.exclusive_minimum = by.exclusive_minimum;
        self.minimum = by.minimum.map(|v| v.to_string());
        self.maximum = by.maximum.map(|v| v.to_string());
        self.data_type.set_number(by);
        self.is_primitive_type = true;
        self
    }
    fn with_string(mut self, _root: &super::OpenApiV3, by: &openapiv3::StringType) -> Self {
        self.pattern = by.pattern.clone();
        self.has_enums = !by.enumeration.is_empty();
        self.is_enum = self.has_enums;

        self.min_length = by.min_length;
        self.data_type.set_string(by);

        match &by.format {
            openapiv3::VariantOrUnknownOrEmpty::Item(item) => match item {
                openapiv3::StringFormat::Date => self.is_date = true,
                openapiv3::StringFormat::DateTime => self.is_date_time = true,
                openapiv3::StringFormat::Password => self.is_date = true,
                openapiv3::StringFormat::Byte => self.is_byte = true,
                openapiv3::StringFormat::Binary => self.is_binary = true,
            },
            openapiv3::VariantOrUnknownOrEmpty::Unknown(format) => match format.as_str() {
                "uuid" => self.is_uuid = true,
                "date" => self.is_date = true,
                "date-time" => self.is_date_time = true,
                _ => {
                    self.is_string = true;
                }
            },
            openapiv3::VariantOrUnknownOrEmpty::Empty => {
                self.is_string = true;
            }
        }

        if self.is_enum {
            let enum_vars = by
                .enumeration
                .iter()
                .flatten()
                .map(|v| EnumValue {
                    name: v.to_upper_camel_case(),
                    value: v.to_string(),
                })
                .collect::<Vec<_>>();

            self.is_model = true;
            self.allowable_values.insert("enumVars".into(), enum_vars);
            self.data_type.set_enum(&self.name, &self.type_);
        } else {
            self.is_primitive_type = true;
        }

        self
    }
}

#[derive(Default, Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub(crate) struct EnumValue {
    name: String,
    value: String,
}
