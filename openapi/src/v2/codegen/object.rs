//! Simplified objects for codegen.
//!
//! This contains the necessary objects for generating actual
//! API objects, their builders, impls, etc.

use crate::v2::models::HttpMethod;
use heck::{CamelCase, SnekCase};

use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Display, Write};
use std::iter;

/// Represents a (simplified) Rust struct.
#[derive(Debug, Clone)]
pub struct ApiObject {
    /// Name of the struct (camel-cased).
    pub name: String,
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
    /// Parameters required for this operation.
    pub params: Vec<Parameter>,
    /// Whether the object itself is required (in body) for this operation.
    pub body_required: bool,
}

/// Represents some parameter somewhere (header, path, query, etc.).
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Name of the parameter (snake-cased).
    pub name: String,
    /// Type of the parameter as a path.
    pub ty_path: String,
    /// Whether this parameter is required.
    pub required: bool,
}

/// Represents a struct field.
#[derive(Debug, Clone)]
pub struct ObjectField {
    /// Name of the field (snake-cased).
    pub name: String,
    /// Actual name of the field (should it be serde-renamed).
    pub rename: Option<String>,
    /// Type of the field as a path.
    pub ty_path: String,
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
            name: name.into(),
            fields: vec![],
            paths: BTreeMap::new(),
        }
    }

    /// Returns a struct representing the impl for this object.
    pub fn impl_repr(&self) -> ApiObjectImpl<'_> {
        ApiObjectImpl(self)
    }

    /// Returns the builders for this object.
    ///
    /// Each builder is bound to an operation in a path. If the object is not
    /// bound to any operation, then the builder only keeps track of the fields.
    // FIXME: Make operations generic across builders. This will reduce the
    // number of structs generated.
    pub fn builders<'a>(&'a self) -> Box<Iterator<Item = ApiObjectBuilder<'a>> + 'a> {
        if self.paths.is_empty() {
            return Box::new(iter::once(ApiObjectBuilder {
                idx: 0,
                object: &self.name,
                method: None,
                op_id: None,
                body_required: true,
                fields: &self.fields,
                global_params: &[],
                local_params: &[],
            })) as Box<_>;
        }

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
}

/// Represents the API object impl.
pub struct ApiObjectImpl<'a>(&'a ApiObject);

/// Represents a builder struct for some API object.
#[derive(Debug, Clone)]
pub struct ApiObjectBuilder<'a> {
    idx: usize,
    op_id: Option<&'a str>,
    method: Option<HttpMethod>,
    object: &'a str,
    body_required: bool,
    fields: &'a [ObjectField],
    global_params: &'a [Parameter],
    local_params: &'a [Parameter],
}

impl<'a> ApiObjectBuilder<'a> {
    /// Returns an iterator of all fields and parameters required for the Rust builder struct.
    ///
    /// **NOTE:** The names yielded by this iterator are unique for a builder.
    /// If there's a collision between a path-specific parameter and an operation-specific
    /// parameter, then the latter overrides the former. If there's a collision between a field
    /// and a parameter, then the latter overrides the former.
    pub(super) fn struct_fields_iter(
        &self,
    ) -> impl Iterator<Item = (&'a str, &'a str, Property)> + 'a {
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
                    )))
                }
            })
            .filter_map(|p| p);

        // Check parameter-field collisions.
        param_iter
            .chain(field_iter)
            .scan(HashSet::new(), |set, (name, ty, prop)| {
                if set.contains(name) {
                    Some(None)
                } else {
                    set.insert(name);
                    Some(Some((name, ty, prop)))
                }
            })
            .filter_map(|p| p)
    }

    /// Returns whether this builder needs `repr(transparent)`
    fn needs_repr_transparent(&self) -> bool {
        let zero_sized_parent = self.fields.is_empty();
        let needs_params = self.struct_fields_iter().any(|(_, _, p)| p.is_required());
        (self.body_required && !zero_sized_parent) || needs_params
    }

    /// Returns whether this builder has any parameters.
    fn has_parameters(&self) -> bool {
        !self.local_params.is_empty() || !self.global_params.is_empty()
    }

    /// Returns whether this builder will have at least one field.
    fn has_atleast_one_field(&self) -> bool {
        self.struct_fields_iter()
            .any(|(_, _, p)| p.is_parameter() || p.is_required())
    }

    /// Write this builder's name into the given formatter.
    fn write_name<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        f.write_str(&self.object)?;
        if let Some(method) = self.method {
            write!(f, "{:?}", method)?;
        }

        f.write_str("Builder")?;
        if self.idx > 0 {
            f.write_str(&self.idx.to_string())?;
        }

        Ok(())
    }

    /// Writes generic stuff to the struct definition, if needed.
    fn write_generics_if_necessary<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        let mut is_generic = false;
        // Inspect fields and parameters and write generics.
        self.struct_fields_iter()
            .filter(|(_, _, prop)| prop.is_required())
            .enumerate()
            .try_for_each(|(i, (name, _, _))| {
                if i == 0 {
                    is_generic = true;
                    f.write_str("<")?;
                } else {
                    f.write_str(", ")?;
                }

                f.write_str(&name.to_camel_case())
            })?;

        if is_generic {
            f.write_str(">")?;
        }

        Ok(())
    }

    /// Writes the body field into the formatter if required.
    fn write_body_field_if_required<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        if self.body_required {
            f.write_str("\n    inner: ")?;
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
        f.write_str("impl ")?;
        f.write_str(&self.0.name)?;
        f.write_str(" {")?;

        // self.0.builders().try_for_each(|builder| {
        //     f.write_str("\n    #[inline]\n    pub fn ")?;
        //     if let Some(_id) = builder.op_id {
        //         // TODO
        //     } else if let Some(m) = builder.method {
        //         // TODO
        //     } else {
        //         f.write_str("builder")?;
        //     }

        //     f.write_str("() -> ")?;
        //     builder.write_name(f)?;
        //     f.write_str(" {\n        Default::default()\n    }")
        // })?;

        f.write_str("\n}\n")
    }
}

impl<'a> Display for ApiObjectBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.needs_repr_transparent() {
            f.write_str("#[repr(transparent)]\n")?;
        }

        f.write_str("#[derive(Debug, Clone)]\npub struct ")?;
        self.write_name(f)?;
        self.write_generics_if_necessary(f)?;

        // If structs don't have any fields, then we go for unit structs.
        let has_fields = self.has_atleast_one_field();
        // If the builder "needs" a parameter, then we go for a separate
        // container which holds both the body (if any) and the parameters,
        // so that we can make the actual builder `#[repr(transparent)]`
        // for safe transmuting.
        let has_parameters = self.has_parameters();

        if has_fields || self.body_required || has_parameters {
            f.write_str(" {")?;
        }

        let mut container = String::new();
        if has_parameters {
            container.push_str("#[derive(Debug, Default, Clone)]\nstruct ");
            self.write_name(&mut container)?;
            container.push_str("Container {");
            self.write_body_field_if_required(&mut container)?;

            f.write_str("\n    inner: ")?;
            self.write_name(f)?;
            f.write_str("Container,")?;
        } else {
            self.write_body_field_if_required(f)?;
        }

        self.struct_fields_iter().try_for_each(|(name, ty, prop)| {
            let (cc, sk) = (name.to_camel_case(), name.to_snek_case());
            // 'container' is meant for parameters.
            self.write_parameter_if_required(prop, &sk, &ty, &mut container)?;

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

        if has_parameters {
            f.write_str("\n")?;
            f.write_str(&container)?;
            f.write_str("\n}\n")?;
        }

        Ok(())
    }
}

impl Display for ApiObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#[derive(Debug, Default, Clone, Deserialize, Serialize)]")?;
        f.write_str("\npub struct ")?;
        f.write_str(&self.name)?;
        f.write_str(" {")?;

        self.fields.iter().try_for_each(|field| {
            f.write_str("\n    ")?;
            if let Some(name) = field.rename.as_ref() {
                f.write_str("#[serde(rename = \"")?;
                f.write_str(name)?;
                f.write_str("\")]\n    ")?;
            }

            f.write_str("pub ")?;
            f.write_str(&field.name)?;
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
