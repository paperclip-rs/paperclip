//! Simplified objects for codegen.

use super::{super::models::HttpMethod, RUST_KEYWORDS};
use heck::{CamelCase, SnekCase};

use std::collections::{BTreeMap, HashSet};
use std::fmt::{self, Debug, Display};
use std::iter;

/// Represents a (simplified) Rust struct.
#[derive(Debug, Clone)]
pub struct ApiObject {
    /// Name of the struct (camel-cased).
    pub name: String,
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
            name: name.into(),
            fields: vec![],
            paths: BTreeMap::new(),
        }
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

/// Represents a builder struct for some API object.
#[derive(Debug, Clone)]
pub struct ApiObjectBuilder<'a> {
    idx: usize,
    method: Option<HttpMethod>,
    object: &'a str,
    body_required: bool,
    fields: &'a [ObjectField],
    global_params: &'a [Parameter],
    local_params: &'a [Parameter],
}

impl<'a> ApiObjectBuilder<'a> {
    fn struct_fields_iter(&self) -> impl Iterator<Item = (&'a str, &'a str, Property)> + 'a {
        let field_iter = self.fields.iter().map(|field| {
            (
                field.name.as_str(),
                field.ty_path.as_str(),
                if field.is_required {
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
                if set.contains(&param.name) {
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

        // Local parameters override global parameters.
        field_iter.chain(param_iter)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Property {
    RequiredField,
    OptionalField,
    RequiredParam,
    OptionalParam,
}

impl Property {
    /// Whether this property is required.
    fn is_required(self) -> bool {
        match self {
            Property::RequiredField | Property::RequiredParam => true,
            _ => false,
        }
    }

    /// Checks whether this property is a parameter.
    fn is_parameter(self) -> bool {
        match self {
            Property::RequiredParam | Property::OptionalParam => true,
            _ => false,
        }
    }

    /// Checks whether this property is a field.
    fn is_field(self) -> bool {
        match self {
            Property::RequiredField | Property::OptionalField => true,
            _ => false,
        }
    }
}

impl<'a> Display for ApiObjectBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#[derive(Debug, Clone)]")?;
        f.write_str("\npub struct ")?;
        f.write_str(&self.object)?;
        if let Some(method) = self.method {
            Debug::fmt(&method, f)?;
        }

        f.write_str("Builder")?;

        if self.idx > 0 {
            f.write_str(&self.idx.to_string())?;
        }

        let mut needs_fields = false;
        self.struct_fields_iter()
            .filter(|(_, _, prop)| prop.is_required())
            .enumerate()
            .try_for_each(|(i, (name, _, prop))| {
                if i > 0 {
                    f.write_str(", ")?;
                } else {
                    needs_fields = true;
                    f.write_str("<")?;
                }

                if prop.is_parameter() {
                    f.write_str("Param")?;
                }

                f.write_str(&name.to_camel_case())
            })?;

        if needs_fields {
            f.write_str(">")?;
        }

        f.write_str(" {")?;

        f.write_str("\n    ")?;
        f.write_str("pub(crate) inner: Option<")?;
        f.write_str(&self.object)?;
        f.write_str(">,")?;

        self.struct_fields_iter().try_for_each(|(name, ty, prop)| {
            let (cc, sk) = (name.to_camel_case(), name.to_snek_case());
            f.write_str("\n    ")?;
            if prop.is_parameter() {
                f.write_str("param_")?;
            }

            f.write_str(&sk)?;
            if prop.is_field() && RUST_KEYWORDS.iter().any(|&k| k == sk) {
                f.write_str("_")?;
            }

            f.write_str(": ")?;
            f.write_str("Option<")?;
            f.write_str(&ty)?;
            f.write_str(">")?;

            if prop.is_required() {
                f.write_str(",\n    ")?;
                if prop.is_parameter() {
                    f.write_str("_param")?;
                }

                f.write_str("_")?;
                f.write_str(&sk)?;
                f.write_str(": ")?;
                f.write_str("core::marker::PhantomData<")?;
                if prop.is_parameter() {
                    f.write_str("Param")?;
                }

                f.write_str(&cc)?;
                f.write_str(">")?;
            }

            f.write_str(",")
        })?;

        f.write_str("\n}\n")
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
