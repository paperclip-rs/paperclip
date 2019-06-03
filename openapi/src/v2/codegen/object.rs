//! Simplified objects for codegen.

use super::{super::models::HttpMethod, RUST_KEYWORDS};
use heck::{CamelCase, SnekCase};

use std::collections::HashMap;
use std::fmt::{self, Display};

/// Represents a (simplified) Rust struct.
#[derive(Debug, Clone)]
pub struct ApiObject {
    /// Name of the struct (camel-cased).
    pub name: String,
    /// List of fields.
    pub fields: Vec<ObjectField>,
    /// Paths with operations which address this object.
    pub paths: HashMap<String, PathOps>,
}

/// Operations in a path.
#[derive(Debug, Clone)]
pub struct PathOps {
    /// Operations for this object and their associated requirements.
    pub req: HashMap<HttpMethod, OpRequirement>,
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
            paths: HashMap::new(),
        }
    }

    /// Returns the builder struct repr if this object needs a builder.
    pub fn builder(&self) -> Option<ApiObjectBuilder<'_>> {
        let builder = Some(ApiObjectBuilder {
            name: self.name.clone() + "Builder",
            inner: self,
        });

        if self.fields.iter().any(|f| f.is_required)
            || self
                .paths
                .values()
                .next()
                .and_then(|path_ops| path_ops.params.iter().find(|p| p.required))
                .is_some()
        {
            if self.paths.len() <= 1 {
                return builder;
            }
        } else if self.paths.len() == 1 {
            return builder;
        }

        if self.paths.len() > 1 {
            // FIXME: How do we handle objects in multiple paths? Operation IDs? Our own IDs?
            warn!(
                "Skipping builder for {} because it has {} different paths",
                self.name,
                self.paths.len()
            );
        }

        None
    }
}

/// Represents a builder struct for some API object.
#[derive(Debug, Clone)]
pub struct ApiObjectBuilder<'a> {
    /// Name of the builder (camel-cased).
    pub name: String,
    inner: &'a ApiObject,
}

impl<'a> ApiObjectBuilder<'a> {
    /// Returns the impl repr for the object builder.
    pub fn impl_repr(&self) -> ApiObjectBuilderImpl<'_> {
        ApiObjectBuilderImpl(self)
    }
}

/// Represents the impl for API object builders.
#[derive(Debug, Clone)]
pub struct ApiObjectBuilderImpl<'a>(&'a ApiObjectBuilder<'a>);

// impl<'a> Display for ApiObjectBuilderImpl<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         //
//     }
// }

impl<'a> ApiObjectBuilder<'a> {
    fn required_fields_iter(&self) -> Box<Iterator<Item = (&'a str, &'a str, Required)> + 'a> {
        let field_iter = self
            .inner
            .fields
            .iter()
            .filter(|f| f.is_required)
            .map(|f| (f.name.as_str(), f.ty_path.as_str(), Required::Field));

        if let Some(path_ops) = self.inner.paths.values().next() {
            let path_iter = path_ops
                .params
                .iter()
                .filter(|p| p.required)
                .map(|p| (p.name.as_str(), p.ty_path.as_str(), Required::Parameter));
            return Box::new(field_iter.chain(path_iter)) as Box<_>;
        }

        Box::new(field_iter) as Box<_>
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Required {
    Field,
    Parameter,
}

impl<'a> Display for ApiObjectBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("#[derive(Debug, Clone)]")?;
        f.write_str("\npub struct ")?;
        f.write_str(&self.name)?;
        f.write_str("<")?;

        if !self.inner.paths.is_empty() {
            f.write_str("Operation")?;
        }

        self.required_fields_iter()
            .enumerate()
            .try_for_each(|(i, (name, _, req))| {
                if !self.inner.paths.is_empty() || i > 0 {
                    f.write_str(", ")?;
                }

                if req == Required::Parameter {
                    f.write_str("Param")?;
                }

                f.write_str(&name.to_camel_case())
            })?;

        f.write_str("> {")?;

        f.write_str("\n    ")?;
        f.write_str("pub(crate) inner: ")?;
        f.write_str(&self.inner.name)?;
        f.write_str(",")?;

        if !self.inner.paths.is_empty() {
            f.write_str("\n    _op: Operation,")?;
        }

        self.required_fields_iter()
            .try_for_each(|(name, ty, req)| {
                let (cc, sk) = (name.to_camel_case(), name.to_snek_case());
                f.write_str("\n    ")?;
                f.write_str(&sk)?;
                if RUST_KEYWORDS.iter().any(|&k| k == sk) {
                    f.write_str("_")?;
                }

                f.write_str(": ")?;
                f.write_str(&ty)?;
                f.write_str(",")?;

                f.write_str("\n    ")?;
                f.write_str("_")?;
                if req == Required::Parameter {
                    f.write_str("_param")?;
                }

                f.write_str(&sk)?;
                f.write_str(": ")?;
                f.write_str("core::marker::PhantomData<")?;
                f.write_str(&cc)?;
                f.write_str(">,")
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
            req: HashMap::new(),
            params: vec![],
        }
    }
}
