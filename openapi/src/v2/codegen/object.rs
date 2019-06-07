//! Simplified objects for codegen.
//!
//! This contains the necessary objects for generating actual
//! API objects, their builders, impls, etc.

use super::RUST_KEYWORDS;
use crate::v2::models::HttpMethod;
use heck::{CamelCase, SnekCase};
use regex::{Captures, Regex};

use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Display, Write};
use std::iter;

lazy_static! {
    /// Regex for appropriate escaping in docs.
    static ref DOC_REGEX: Regex = Regex::new(r"\[|\]").expect("invalid doc regex?");
}

/// Represents a (simplified) Rust struct.
#[derive(Debug, Clone)]
pub struct ApiObject {
    /// Name of the struct (camel-cased).
    pub name: String,
    /// Description for this object (if any), to be used for docs.
    pub description: Option<String>,
    /// Path to this object from (generated) root module.
    pub path: String,
    /// List of fields.
    pub fields: Vec<ObjectField>,
    /// Paths with operations which address this object.
    pub paths: BTreeMap<String, PathOps>,
}

/// Operations in a path.
#[derive(Debug, Clone)]
pub struct PathOps {
    /// Operations for this object and their associated requirements.
    pub req: BTreeMap<HttpMethod, OpRequirement>,
    /// Parameters required for all operations in this path.
    pub params: Vec<Parameter>,
}

/// Requirement for an object corresponding to some operation.
#[derive(Debug, Clone)]
pub struct OpRequirement {
    /// Operation ID (if it's provided in the schema).
    ///
    /// If there are multiple operations for the same path, then we
    /// attempt to use this.
    pub id: Option<String>,
    /// Description of this operation (if any), to be used for docs.
    pub description: Option<String>,
    /// Parameters required for this operation.
    pub params: Vec<Parameter>,
    /// Whether the object itself is required (in body) for this operation.
    pub body_required: bool,
}

/// Represents some parameter somewhere (header, path, query, etc.).
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Name of the parameter.
    pub name: String,
    /// Description of this operation (if any), to be used for docs.
    pub description: Option<String>,
    /// Type of the parameter as a path.
    pub ty_path: String,
    /// Whether this parameter is required.
    pub required: bool,
}

/// Represents a struct field.
#[derive(Debug, Clone)]
pub struct ObjectField {
    /// Name of the field.
    pub name: String,
    /// Type of the field as a path.
    pub ty_path: String,
    /// Description of this operation (if any), to be used for docs.
    pub description: Option<String>,
    /// Whether this field is required (i.e., not optional).
    pub is_required: bool,
    /// Whether this field should be boxed.
    pub boxed: bool,
}

impl ApiObject {
    /// Create an object with the given name.
    pub fn with_name<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        ApiObject {
            // NOTE: Even though it's empty, it'll be replaced by the emitter.
            path: String::new(),
            description: None,
            name: name.into(),
            fields: vec![],
            paths: BTreeMap::new(),
        }
    }

    /// Returns a struct representing the impl for this object.
    pub fn impl_repr(&self) -> ApiObjectImpl<'_> {
        ApiObjectImpl {
            inner: self,
            builders: vec![],
        }
    }

    /// Returns the builders for this object.
    ///
    /// Each builder is bound to an operation in a path. If the object is not
    /// bound to any operation, then the builder only keeps track of the fields.
    // FIXME: Make operations generic across builders. This will reduce the
    // number of structs generated.
    pub fn builders<'a>(
        &'a self,
        helper_module_prefix: &'a str,
    ) -> Box<Iterator<Item = ApiObjectBuilder<'a>> + 'a> {
        if self.paths.is_empty() {
            return Box::new(iter::once(ApiObjectBuilder {
                idx: 0,
                multiple_builders_exist: false,
                helper_module_prefix,
                description: None,
                object: &self.name,
                method: None,
                op_id: None,
                body_required: true,
                fields: &self.fields,
                global_params: &[],
                local_params: &[],
            })) as Box<_>;
        }

        let count = self
            .paths
            .values()
            .flat_map(|path_ops| path_ops.req.iter())
            .count();

        Box::new(
            self.paths
                .values()
                .enumerate()
                .flat_map(move |(idx, path_ops)| {
                    path_ops
                        .req
                        .iter()
                        .map(move |(&method, req)| ApiObjectBuilder {
                            idx,
                            multiple_builders_exist: count > 1,
                            helper_module_prefix,
                            description: req.description.as_ref().map(String::as_str),
                            object: &self.name,
                            op_id: req.id.as_ref().map(String::as_str),
                            method: Some(method),
                            body_required: req.body_required,
                            fields: &self.fields,
                            global_params: &path_ops.params,
                            local_params: &req.params,
                        })
                }),
        ) as Box<_>
    }

    /// Writes the given string (if any) as Rust documentation into
    /// the given formatter.
    fn write_docs<F, S>(stuff: Option<S>, f: &mut F, levels: usize) -> fmt::Result
    where
        F: Write,
        S: AsRef<str>,
    {
        let indent = iter::repeat(' ').take(levels * 4).collect::<String>();
        if let Some(desc) = stuff.as_ref() {
            desc.as_ref().split('\n').try_for_each(|line| {
                f.write_str("\n")?;
                f.write_str(&indent)?;
                f.write_str("///")?;
                if line.is_empty() {
                    return Ok(());
                }

                f.write_str(" ")?;
                f.write_str(
                    &DOC_REGEX
                        .replace_all(line, |c: &Captures| match &c[0] {
                            "[" => "\\[",
                            "]" => "\\]",
                            _ => unreachable!(),
                        })
                        .trim_end(),
                )
            })?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

/// Represents the API object impl.
pub struct ApiObjectImpl<'a> {
    inner: &'a ApiObject,
    pub(super) builders: Vec<ApiObjectBuilder<'a>>,
}

impl<'a> ApiObjectImpl<'a> {
    fn write_builder_methods<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        for builder in &self.builders {
            let mut temp = String::new();
            let has_fields = builder.has_atleast_one_field();
            if builder.description.is_none() {
                temp.write_str("\n")?;
            }

            temp.write_str("    #[inline]\n    pub fn ")?;
            if let Some(name) = builder.con_fn_name() {
                temp.write_str(&name)?;
                ApiObject::write_docs(builder.description.as_ref(), f, 1)?;
            } else {
                // If we don't know, then we go for a simple object builder.
                f.write_str("\n    /// Create a builder for this object.")?;
                temp.write_str("builder")?;
            }

            // Now that we've written the docs, we can write the actual method signature.
            f.write_str(&temp)?;
            f.write_str("() -> ")?;
            builder.write_name(f)?;
            builder.write_generics_if_necessary(f, TypeParameters::ReplaceAll)?;
            f.write_str(" {\n        ")?;
            builder.write_name(f)?;

            if has_fields || builder.body_required {
                f.write_str(" {")?;
            }

            let needs_container = builder.needs_container();
            if needs_container {
                f.write_str("\n            ")?;
                f.write_str("inner: Default::default(),")?;
            } else if builder.body_required {
                f.write_str("\n            ")?;
                f.write_str("body: Default::default(),")?;
            }

            builder
                .struct_fields_iter()
                .try_for_each(|(name, _, prop, _)| {
                    if prop.is_required() {
                        f.write_str("\n            ")?;
                        if prop.is_parameter() {
                            f.write_str("_param")?;
                        }

                        f.write_str("_")?;
                        f.write_str(&name.to_snek_case())?;
                        f.write_str(": core::marker::PhantomData,")?;
                    } else if prop.is_parameter() && !needs_container {
                        f.write_str("\n            param_")?;
                        f.write_str(&name.to_snek_case())?;
                        f.write_str(": None,")?;
                    }

                    Ok(())
                })?;

            if has_fields || builder.body_required {
                f.write_str("\n        }")?;
            }

            f.write_str("\n    }\n")?;
        }

        Ok(())
    }
}

/// Represents a builder struct for some API object.
#[derive(Debug, Clone)]
pub struct ApiObjectBuilder<'a> {
    idx: usize,
    multiple_builders_exist: bool,
    helper_module_prefix: &'a str,
    op_id: Option<&'a str>,
    method: Option<HttpMethod>,
    description: Option<&'a str>,
    object: &'a str,
    body_required: bool,
    fields: &'a [ObjectField],
    global_params: &'a [Parameter],
    local_params: &'a [Parameter],
}

impl<'a> ApiObjectBuilder<'a> {
    /// Returns a struct representing the impl for this builder.
    pub fn impl_repr(&self) -> ApiObjectBuilderImpl<'_, '_> {
        ApiObjectBuilderImpl(self)
    }

    /// Name of the constructor function which creates this builder.
    pub fn con_fn_name(&self) -> Option<String> {
        match (self.op_id, self.method) {
            // If there's a method and we *don't* have any collisions
            // (i.e., two or more paths for same object), then we default
            // to using the method ...
            (_, Some(meth)) if !self.multiple_builders_exist => {
                Some(meth.to_string().to_snek_case())
            }
            // If there's an operation ID, then we go for that ...
            (Some(id), _) => Some(id.to_snek_case()),
            // If there's a method, then we go for numbered functions ...
            (_, Some(meth)) => {
                let mut name = meth.to_string().to_snek_case();
                if self.idx > 0 {
                    name.push('_');
                    name.push_str(&self.idx.to_string());
                }

                Some(name)
            }
            // We don't know what to do ...
            _ => None,
        }
    }

    /// Returns an iterator of all fields and parameters required for the Rust builder struct.
    ///
    /// **NOTE:** The names yielded by this iterator are unique for a builder.
    /// If there's a collision between a path-specific parameter and an operation-specific
    /// parameter, then the latter overrides the former. If there's a collision between a field
    /// and a parameter, then the latter overrides the former.
    pub(super) fn struct_fields_iter(
        &self,
    ) -> impl Iterator<Item = (&'a str, &'a str, Property, Option<&'a str>)> + 'a {
        let body_required = self.body_required;
        let field_iter = self.fields.iter().map(move |field| {
            (
                field.name.as_str(),
                field.ty_path.as_str(),
                // We "require" the object fields only if the object itself is required.
                if body_required && field.is_required {
                    Property::RequiredField
                } else {
                    Property::OptionalField
                },
                field.description.as_ref().map(String::as_str),
            )
        });

        let param_iter = self
            .global_params
            .iter()
            .chain(self.local_params.iter())
            .scan(HashSet::new(), |set, param| {
                // Local parameters override global parameters.
                if set.contains(&param.name) {
                    // Workaround because `scan` stops when it encounters
                    // `None`, but we want filtering.
                    Some(None)
                } else {
                    set.insert(&param.name);
                    Some(Some((
                        param.name.as_str(),
                        param.ty_path.as_str(),
                        if param.required {
                            Property::RequiredParam
                        } else {
                            Property::OptionalParam
                        },
                        param.description.as_ref().map(String::as_str),
                    )))
                }
            })
            .filter_map(|p| p);

        // Check parameter-field collisions.
        param_iter
            .chain(field_iter)
            .scan(HashSet::new(), |set, (name, ty, prop, desc)| {
                if set.contains(name) {
                    Some(None)
                } else {
                    set.insert(name);
                    Some(Some((name, ty, prop, desc)))
                }
            })
            .filter_map(|p| p)
    }

    /// Returns whether a separate container is needed for the builder struct.
    fn needs_container(&self) -> bool {
        self.local_params
            .iter()
            .chain(self.global_params.iter())
            .any(|p| p.required)
            || (self.body_required && self.fields.iter().any(|f| f.is_required))
    }

    /// Returns whether this builder will have at least one field.
    fn has_atleast_one_field(&self) -> bool {
        self.struct_fields_iter()
            .any(|(_, _, p, _)| p.is_parameter() || p.is_required())
    }

    /// Write this builder's name into the given formatter.
    fn write_name<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        f.write_str(&self.object)?;
        if let Some(method) = self.method {
            write!(f, "{}", method)?;
        }

        f.write_str("Builder")?;
        if self.idx > 0 {
            f.write_str(&self.idx.to_string())?;
        }

        Ok(())
    }

    /// Write this builder's container name into the given formatter.
    fn write_container_name<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        self.write_name(f)?;
        f.write_str("Container")
    }

    /// Writes generic parameters, if needed.
    ///
    /// Also takes an enum to specify whether the one/all/none of the parameters
    /// should make use of actual types.
    fn write_generics_if_necessary<F>(
        &self,
        f: &mut F,
        params: TypeParameters<'_>,
    ) -> Result<usize, fmt::Error>
    where
        F: Write,
    {
        let mut num_generics = 0;
        // Inspect fields and parameters and write generics.
        self.struct_fields_iter()
            .filter(|(_, _, prop, _)| prop.is_required())
            .enumerate()
            .try_for_each(|(i, (name, _, _, _))| {
                num_generics += 1;
                if i == 0 {
                    f.write_str("<")?;
                } else {
                    f.write_str(", ")?;
                }

                match params {
                    TypeParameters::ChangeOne(n) if name == n => {
                        f.write_str(self.helper_module_prefix)?;
                        f.write_str(&name.to_camel_case())?;
                        return f.write_str("Exists");
                    }
                    TypeParameters::ReplaceAll => {
                        f.write_str(self.helper_module_prefix)?;
                        f.write_str("Missing")?;
                    }
                    _ => (),
                }

                f.write_str(&name.to_camel_case())
            })?;

        if num_generics > 0 {
            f.write_str(">")?;
        }

        Ok(num_generics)
    }

    /// Writes the body field into the formatter if required.
    fn write_body_field_if_required<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        if self.body_required {
            f.write_str("\n    body: ")?;
            f.write_str(&self.object)?;
            f.write_str(",")?;
        }

        Ok(())
    }

    /// Writes the parameter into the formatter if required.
    fn write_parameter_if_required<F>(
        &self,
        prop: Property,
        name: &str,
        ty: &str,
        f: &mut F,
    ) -> fmt::Result
    where
        F: Write,
    {
        if prop.is_parameter() {
            f.write_str("\n    param_")?;
            f.write_str(&name)?;
            f.write_str(": Option<")?;
            f.write_str(&ty)?;
            f.write_str(">,")?;
        }

        Ok(())
    }
}

/// Represents the API object builder impl.
pub struct ApiObjectBuilderImpl<'a, 'b>(&'a ApiObjectBuilder<'b>);

impl<'a, 'b> ApiObjectBuilderImpl<'a, 'b> {
    /// Writes the property-related methods to the given formatter.
    fn write_property_method<F>(
        &self,
        name: &str,
        ty: &str,
        prop: Property,
        desc: Option<&str>,
        f: &mut F,
    ) -> fmt::Result
    where
        F: Write,
    {
        let field_name = name.to_snek_case();
        let known_type = !ty.contains("::");
        let (prop_is_parameter, prop_is_required) = (prop.is_parameter(), prop.is_required());
        ApiObject::write_docs(desc, f, 1)?;

        f.write_str("    #[inline]\n    pub fn ")?;
        if RUST_KEYWORDS.iter().any(|&k| k == field_name) {
            f.write_str("r#")?;
        }

        f.write_str(&field_name)?;
        f.write_str("(mut self, value: ")?;
        if known_type {
            write!(f, "impl Into<{}>", ty)?;
        } else {
            f.write_str(ty)?;
        }

        f.write_str(") -> ")?;
        if prop_is_required {
            self.0.write_name(f)?;
            self.0
                .write_generics_if_necessary(f, TypeParameters::ChangeOne(name))?;
        } else {
            f.write_str("Self")?;
        }

        f.write_str(" {\n        self.")?;
        if self.0.needs_container() {
            f.write_str("inner.")?;
        }

        if prop_is_parameter {
            f.write_str("param_")?;
        } else if self.0.body_required {
            // parameter won't be in body, for sure.
            f.write_str("body.")?;
        }

        f.write_str(&field_name)?;
        if prop.is_field() && RUST_KEYWORDS.iter().any(|&k| k == field_name) {
            f.write_str("_")?;
        }

        f.write_str(" = ")?;
        if prop_is_parameter || !prop_is_required {
            f.write_str("Some(")?;
        }

        f.write_str("value.into()")?;
        if prop_is_parameter || !prop_is_required {
            f.write_str(")")?;
        }

        f.write_str(";\n        ")?;
        if prop_is_required {
            f.write_str("unsafe { std::mem::transmute(self) }")?;
        } else {
            f.write_str("self")?;
        }

        f.write_str("\n    }\n")
    }
}

enum TypeParameters<'a> {
    Generic,
    ChangeOne(&'a str),
    ReplaceAll,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Property {
    RequiredField,
    OptionalField,
    RequiredParam,
    OptionalParam,
}

#[allow(dead_code)]
impl Property {
    /// Whether this property is required.
    pub(super) fn is_required(self) -> bool {
        match self {
            Property::RequiredField | Property::RequiredParam => true,
            _ => false,
        }
    }

    /// Checks whether this property is a parameter.
    pub(super) fn is_parameter(self) -> bool {
        match self {
            Property::RequiredParam | Property::OptionalParam => true,
            _ => false,
        }
    }

    /// Checks whether this property is a field.
    pub(super) fn is_field(self) -> bool {
        match self {
            Property::RequiredField | Property::OptionalField => true,
            _ => false,
        }
    }
}

impl<'a> Display for ApiObjectImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.builders.is_empty() {
            return Ok(());
        }

        f.write_str("impl ")?;
        f.write_str(&self.inner.name)?;
        f.write_str(" {")?;
        self.write_builder_methods(f)?;
        f.write_str("}\n")
    }
}

impl<'a> Display for ApiObjectBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("/// Builder ")?;
        if let (Some(name), Some(m)) = (self.con_fn_name(), self.method) {
            f.write_str("created by [`")?;
            f.write_str(&self.object)?;
            f.write_str("::")?;
            f.write_str(&name)?;
            f.write_str("`](./struct.")?;
            f.write_str(&self.object)?;
            f.write_str(".html#method.")?;
            f.write_str(&name)?;
            f.write_str(") method for a `")?;
            f.write_str(&m.to_string().to_uppercase())?;
            f.write_str("` operation associated with `")?;
            f.write_str(&self.object)?;
            f.write_str("`.\n")?;
        } else {
            f.write_str("for [`")?;
            f.write_str(&self.object)?;
            f.write_str("`](./struct.")?;
            f.write_str(&self.object)?;
            f.write_str(".html) object.\n")?;
        }

        // If the builder "needs" parameters/fields, then we go for a separate
        // container which holds both the body (if any) and the parameters,
        // so that we can make the actual builder `#[repr(transparent)]`
        // for safe transmuting.
        let needs_container = self.needs_container();
        if needs_container {
            f.write_str("#[repr(transparent)]\n")?;
        }

        f.write_str("#[derive(Debug, Clone)]\npub struct ")?;
        self.write_name(f)?;
        self.write_generics_if_necessary(f, TypeParameters::Generic)?;

        // If structs don't have any fields, then we go for unit structs.
        let has_fields = self.has_atleast_one_field();

        if has_fields || self.body_required || needs_container {
            f.write_str(" {")?;
        }

        let mut container = String::new();
        if needs_container {
            container.push_str("#[derive(Debug, Default, Clone)]\nstruct ");
            self.write_container_name(&mut container)?;
            container.push_str(" {");
            self.write_body_field_if_required(&mut container)?;

            f.write_str("\n    inner: ")?;
            self.write_container_name(f)?;
            f.write_str(",")?;
        } else {
            self.write_body_field_if_required(f)?;
        }

        self.struct_fields_iter()
            .try_for_each(|(name, ty, prop, _)| {
                let (cc, sk) = (name.to_camel_case(), name.to_snek_case());
                if needs_container {
                    self.write_parameter_if_required(prop, &sk, &ty, &mut container)?;
                } else {
                    self.write_parameter_if_required(prop, &sk, &ty, f)?;
                }

                if prop.is_required() {
                    f.write_str("\n    ")?;
                    if prop.is_parameter() {
                        f.write_str("_param")?;
                    }

                    f.write_str("_")?;
                    f.write_str(&sk)?;
                    f.write_str(": ")?;
                    f.write_str("core::marker::PhantomData<")?;
                    f.write_str(&cc)?;
                    f.write_str(">,")?;
                }

                Ok(())
            })?;

        if has_fields || self.body_required {
            f.write_str("\n}\n")?;
        } else {
            f.write_str(";\n")?;
        }

        if needs_container {
            f.write_str("\n")?;
            f.write_str(&container)?;
            f.write_str("\n}\n")?;
        }

        Ok(())
    }
}

impl<'a, 'b> Display for ApiObjectBuilderImpl<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut generics = String::new();
        self.0
            .write_generics_if_necessary(&mut generics, TypeParameters::Generic)?;

        let mut has_fields = false;
        self.0
            .struct_fields_iter()
            .filter(|(_, _, p, _)| (self.0.body_required && p.is_field()) || p.is_parameter())
            .enumerate()
            .try_for_each(|(i, (name, ty, prop, desc))| {
                if i == 0 {
                    has_fields = true;
                    f.write_str("impl")?;
                    f.write_str(&generics)?;
                    f.write_str(" ")?;
                    self.0.write_name(f)?;
                    f.write_str(&generics)?;
                    f.write_str(" {")?;
                }

                self.write_property_method(name, ty, prop, desc, f)
            })?;

        if has_fields {
            f.write_str("}\n")?;
        }

        Ok(())
    }
}

impl Display for ApiObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ApiObject::write_docs(self.description.as_ref(), f, 0)?;

        f.write_str("#[derive(Debug, Default, Clone, Deserialize, Serialize)]")?;
        f.write_str("\npub struct ")?;
        f.write_str(&self.name)?;
        f.write_str(" {")?;

        self.fields.iter().try_for_each(|field| {
            let mut new_name = field.name.to_snek_case();
            // Check if the field matches a Rust keyword and add '_' suffix.
            if RUST_KEYWORDS.iter().any(|&k| k == new_name) {
                new_name.push('_');
            }

            ApiObject::write_docs(field.description.as_ref(), f, 1)?;
            if field.description.is_none() {
                f.write_str("\n")?;
            }

            f.write_str("    ")?;
            if new_name != field.name.as_str() {
                f.write_str("#[serde(rename = \"")?;
                f.write_str(&field.name)?;
                f.write_str("\")]\n    ")?;
            }

            f.write_str("pub ")?;
            f.write_str(&new_name)?;
            f.write_str(": ")?;
            if !field.is_required {
                f.write_str("Option<")?;
            }

            if field.boxed {
                f.write_str("Box<")?;
            }

            f.write_str(&field.ty_path)?;

            if field.boxed {
                f.write_str(">")?;
            }

            if !field.is_required {
                f.write_str(">")?;
            }

            f.write_str(",")?;
            Ok(())
        })?;

        if !self.fields.is_empty() {
            f.write_str("\n")?;
        }

        f.write_str("}\n")
    }
}

impl Default for PathOps {
    fn default() -> Self {
        PathOps {
            req: BTreeMap::new(),
            params: vec![],
        }
    }
}
