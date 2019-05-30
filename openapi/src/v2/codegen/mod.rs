//! Code generation for OpenAPI v2.

pub mod object;
mod state;

pub use self::state::EmitterState;

use self::object::{ApiObject, ObjectField};
use super::{
    models::{Api, DataType, DataTypeFormat},
    Schema,
};
use crate::error::PaperClipError;
use failure::Error;
use heck::{CamelCase, SnekCase};

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::marker::PhantomData;
use std::path::PathBuf;

/// Common conflicting keywords in Rust. An underscore will be added
/// to fields using these keywords.
// FIXME: Fill this list!
const RUST_KEYWORDS: &[&str] = &["type", "continue", "enum", "ref"];

/// Extension to `Schema` for internal use.
pub(crate) trait SchemaExt: Schema {
    /// Checks if this definition matches a known Rust type and returns it.
    fn matching_unit_type(&self) -> Option<&'static str> {
        match self.format() {
            Some(DataTypeFormat::Int32) => Some("i32"),
            Some(DataTypeFormat::Int64) => Some("i64"),
            Some(DataTypeFormat::Float) => Some("f32"),
            Some(DataTypeFormat::Double) => Some("f64"),
            _ => match self.data_type() {
                Some(DataType::Integer) => Some("i64"),
                Some(DataType::Number) => Some("f64"),
                Some(DataType::Boolean) => Some("bool"),
                Some(DataType::String) => Some("String"),
                _ => None,
            },
        }
    }
}

impl<T: Schema> SchemaExt for T {}

/// Default emitter for anything that implements `Schema` trait.
///
/// This isn't special in any way, as `Emitter` trait takes
/// care of all the heavy load.
pub struct DefaultEmitter<S> {
    state: EmitterState,
    _schema: PhantomData<S>,
}

impl<S> From<EmitterState> for DefaultEmitter<S> {
    fn from(state: EmitterState) -> Self {
        DefaultEmitter {
            state,
            _schema: PhantomData,
        }
    }
}

impl<S: Schema + Debug> Emitter for DefaultEmitter<S> {
    type Definition = S;

    fn state(&self) -> &EmitterState {
        &self.state
    }
}

/// `Emitter` represents the interface for generating the relevant
/// modules, API object definitions and the associated calls.
pub trait Emitter {
    /// The associated `Schema` implementor.
    type Definition: Schema + Debug;

    /// Returns a reference to the underlying state.
    fn state(&self) -> &EmitterState;

    /// Entrypoint for emitter. Given an API spec, generate code
    /// inside Rust modules in the configured working directory.
    fn generate(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        // Generate file contents by accumulating definitions.
        for (name, schema) in &api.definitions {
            info!("Creating definition {}", name);
            let schema = schema.read();
            self.generate_def_from_root(&schema)?;
        }

        let state = self.state();
        state.declare_modules()?;
        state.write_definitions()?;

        Ok(())
    }

    /// Returns an iterator of path components for the given definition.
    ///
    /// **NOTE:** All components are [snake_cased](https://docs.rs/heck/*/heck/trait.SnekCase.html) (including the definition name).
    fn def_ns_name<'a>(
        &self,
        def: &'a Self::Definition,
    ) -> Result<Box<Iterator<Item = String> + 'a>, Error> {
        let state = self.state();
        def.name()
            .map(|n| n.split(state.ns_sep).map(SnekCase::to_snek_case))
            .ok_or_else(|| PaperClipError::InvalidDefinitionName.into())
            .map(|i| Box::new(i) as Box<_>)
    }

    /// Returns the [CamelCase](https://docs.rs/heck/*/heck/trait.CamelCase.html) name for the given definition.
    fn def_name(&self, def: &Self::Definition) -> Result<String, Error> {
        Ok(self
            .def_ns_name(def)?
            .last()
            .map(|s| s.to_camel_case())
            .expect("last item always exists for split?"))
    }

    /// Returns the module path (from working directory) for the given definition.
    ///
    /// **NOTE:** This doesn't (shouldn't) set any extension to the leaf component.
    fn def_mod_path(&self, def: &Self::Definition) -> Result<PathBuf, Error> {
        let state = self.state();
        let mut path = state.working_dir.clone();
        path.extend(self.def_ns_name(def)?);
        Ok(path)
    }

    /// Given a schema definition, generate the corresponding Rust definition.
    ///
    /// **NOTE:** This doesn't generate any files. It only adds the generated stuff
    /// to `EmitterState`.
    fn generate_def_from_root(&self, def: &Self::Definition) -> Result<(), Error> {
        let state = self.state();
        // Generate the object.
        let object = match self.build_def(def, true)? {
            EmittedUnit::Object(o) => o,
            // We don't care about type aliases because we resolve them anyway.
            _ => return Ok(()),
        };

        let mut full_path = self.def_mod_path(def)?;
        let dir_path = full_path
            .parent()
            .ok_or(PaperClipError::InvalidDefinitionPath(full_path.clone()))?;
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // Get the relative path to the parent dir.
        let rel_path = full_path
            .strip_prefix(&state.working_dir)
            .map_err(|_| PaperClipError::InvalidDefinitionPath(full_path.clone()))?;

        // Gather the immediate parent-children pairs for module declarations.
        let mut mods = state.mod_children.borrow_mut();
        for path in rel_path.ancestors() {
            match (path.parent(), path.file_name()) {
                (Some(parent), Some(name)) if parent.parent().is_some() => {
                    let entry = mods.entry(parent.into()).or_insert_with(HashSet::new);
                    entry.insert(name.to_string_lossy().into_owned());
                }
                _ => (),
            }
        }

        // Add generated object to state.
        let mut def_mods = state.def_mods.borrow_mut();
        full_path.set_extension("rs");
        def_mods.insert(full_path, object);

        Ok(())
    }

    /// Builds a given definition. Also takes a `bool` to specify whether we're
    /// planning to define a Rust type or whether we're reusing an existing type.
    fn build_def(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        trace!("Building definition: {:?}", def);
        if let Some(ty) = def.matching_unit_type() {
            trace!("Matches unit type: {}", ty);
            if define {
                return Ok(EmittedUnit::None);
            }

            return Ok(EmittedUnit::Known(ty.to_owned()));
        }

        match def.data_type() {
            Some(DataType::Array) => self.emit_array(def, define),
            Some(DataType::Object) => self.emit_object(def, define),
            Some(_) => unreachable!("bleh?"), // we've already handled everything else
            None => {
                if define {
                    Ok(EmittedUnit::None)
                } else {
                    Ok(EmittedUnit::Known("String".into()))
                }
            }
        }
    }

    /// Assumes that the given definition is an array and returns the corresponding
    /// vector type for it.
    fn emit_array(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        let it = def
            .items()
            .ok_or(PaperClipError::MissingArrayItem(self.def_name(def).ok()))?;

        let schema = it.read();
        let ty = self.build_def(&schema, false)?.known_type();
        Ok(EmittedUnit::Known(String::from("Vec<") + &ty + ">"))
    }

    /// Assumes that the given definition is an object and returns the corresponding
    /// Rust struct / map.
    fn emit_object(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        match self.try_emit_map(def, define)? {
            EmittedUnit::None => (),
            x => return Ok(x),
        }

        if !define {
            // Use absolute paths to save some pain.
            // FIXME: This assumes that the working directory is the immediate
            // child module of a crate. We should support custom prefixes.
            let mut ty_path = String::from("crate");
            let mut iter = self.def_ns_name(def)?.peekable();
            while let Some(mut c) = iter.next() {
                ty_path.push_str("::");
                if iter.peek().is_none() {
                    ty_path.push_str(&c);
                    ty_path.push_str("::");
                    c = c.to_camel_case();
                }

                ty_path.push_str(&c);
            }

            return Ok(EmittedUnit::Known(ty_path));
        }

        self.emit_struct(def)
    }

    /// Checks if the given definition is a simple map and returns the corresponding `BTreeMap`.
    fn try_emit_map(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        if let Some(s) = def.additional_properties() {
            let schema = s.read();
            let ty = self.build_def(&schema, false)?.known_type();
            let map = format!("std::collections::BTreeMap<String, {}>", ty);
            Ok(EmittedUnit::Known(map))
        } else {
            Ok(EmittedUnit::None)
        }
    }

    /// Helper for `emit_object` - This returns the Rust struct definition for the
    /// given schema definition.
    fn emit_struct(&self, def: &Self::Definition) -> Result<EmittedUnit, Error> {
        let mut obj = ApiObject {
            name: self.def_name(def)?,
            fields: vec![],
        };

        if let Some(props) = def.properties() {
            props
                .iter()
                .try_for_each(|(name, prop)| -> Result<(), Error> {
                    let mut new_name = name.to_snek_case();
                    // Check if the field matches a Rust keyword and add '_' suffix.
                    if RUST_KEYWORDS.iter().any(|&k| k == new_name) {
                        new_name.push('_');
                    }

                    let schema = prop.read();
                    let ty = self.build_def(&schema, false)?;

                    obj.fields.push(ObjectField {
                        // If we've modified the name, mark it for serde renaming.
                        rename: if new_name != name.as_str() {
                            Some(name.clone())
                        } else {
                            None
                        },
                        name: new_name,
                        ty_path: ty.known_type(),
                        is_required: def.is_required_property(name),
                        boxed: schema.is_cyclic(),
                    });

                    Ok(())
                })?
        }

        Ok(EmittedUnit::Object(obj))
    }
}

/// Some "thing" emitted by the emitter.
pub enum EmittedUnit {
    /// Object represented as a Rust struct.
    Object(ApiObject),
    /// Some Rust type.
    Known(String),
    /// Nothing to do.
    None,
}

impl EmittedUnit {
    #[inline]
    fn known_type(self) -> String {
        match self {
            EmittedUnit::Known(s) => s,
            _ => panic!("Emitted unit is not a known type"),
        }
    }
}
