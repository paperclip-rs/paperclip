//! Code generation for OpenAPI v2.

use super::{
    models::{Api, DataType, DataTypeFormat},
    Schema,
};
use crate::error::PaperClipError;
use failure::Error;
use heck::{CamelCase, SnekCase};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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

/// Holds the state for your schema emitter.
#[derive(Debug, Clone)]
pub struct EmitterState {
    /// Working directory - the path in which the necessary modules are generated.
    pub working_dir: PathBuf,
    /// Namespace separation string.
    pub ns_sep: &'static str,
    /// Maps parent mod to immediate children. Used for declaring modules.
    mod_children: Rc<RefCell<HashMap<PathBuf, HashSet<String>>>>,
    /// Holds generated code for leaf modules.
    def_mods: Rc<RefCell<HashMap<PathBuf, String>>>,
}

impl Default for EmitterState {
    fn default() -> EmitterState {
        EmitterState {
            working_dir: PathBuf::from("."),
            ns_sep: ".",
            def_mods: Rc::new(RefCell::new(HashMap::new())),
            mod_children: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

/// Default emitter for anything that implements `Schema` trait.
///
/// This isn't special in any way, as `SchemaEmitter` trait takes
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

impl<S: Schema + Debug> SchemaEmitter for DefaultEmitter<S> {
    type Definition = S;

    fn state(&self) -> &EmitterState {
        &self.state
    }
}

/// `SchemaEmitter` is the first phase in codegen. It generates the relevant
/// modules and API objects required for the next phase.
pub trait SchemaEmitter {
    /// The associated `Schema` implementor.
    type Definition: Schema + Debug;

    /// Returns a reference to the underlying state.
    fn state(&self) -> &EmitterState;

    /// Entrypoint for emitter. Given an API spec, generate the definitions
    /// inside Rust modules in the configured working directory.
    fn create_defs(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        // Generate file contents by accumulating definitions.
        for (name, schema) in &api.definitions {
            info!("Creating definition {}", name);
            let schema = schema.read();
            self.generate_def_from_root(&schema)?;
        }

        let state = self.state();
        let mut mods = state.mod_children.borrow_mut();
        info!("Adding mod declarations.");
        // Now we know everything about the tree containing the modules.
        // Let's generate the module declarations.
        for (rel_parent, children) in mods.drain() {
            let mut mod_path = state.working_dir.join(&rel_parent);
            mod_path.push("mod.rs");

            let mut contents = String::new();
            for child in children {
                contents.push_str("pub mod ");
                contents.push_str(&child);
                contents.push_str(";\n");
            }

            self.write_contents(contents, &mod_path)?;
        }

        // Write the definitions to leaf modules.
        let mut def_mods = state.def_mods.borrow_mut();
        info!("Writing definitions.");
        for (mod_path, contents) in def_mods.drain() {
            self.write_contents(contents, &mod_path)?;
        }

        Ok(())
    }

    /// Writes the given contents to a file at the given path.
    fn write_contents(&self, contents: String, path: &Path) -> Result<(), Error> {
        let mut fd = BufWriter::new(OpenOptions::new().create(true).write(true).open(path)?);
        fd.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// Given a schema definition, generate the corresponding Rust definition.
    ///
    /// **NOTE:** This doesn't generate any files. It only generates the code
    /// and writes it to `EmitterState`.
    fn generate_def_from_root(&self, def: &Self::Definition) -> Result<(), Error> {
        let mut full_path = self.def_mod_path(def)?;
        let dir_path = full_path
            .parent()
            .ok_or(PaperClipError::InvalidDefinitionPath(full_path.clone()))?;
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // Get the relative path to the parent dir.
        let state = self.state();
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

        // Generate the code and add the content to the state.
        let mut def_mods = state.def_mods.borrow_mut();
        let mut def_str = self.build_def(def, true)?;
        def_str.push('\n');
        full_path.set_extension("rs");
        def_mods.insert(full_path, def_str);

        Ok(())
    }

    /// Builds a given definition. Also takes a `bool` to specify whether we're
    /// planning to define a Rust type or whether we're reusing an existing type.
    fn build_def(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        trace!("Building definition: {:?}", def);
        if let Some(ty) = def.matching_unit_type() {
            trace!("Matches unit type: {}", ty);
            if define {
                return self.emit_type_alias(def, ty);
            }

            return Ok(ty.to_owned());
        }

        match def.data_type() {
            Some(DataType::Array) => self.emit_array(def, define),
            Some(DataType::Object) => self.emit_object(def, define),
            Some(_) => unreachable!("bleh?"), // we've already handled everything else
            None => {
                if define {
                    // default to String
                    self.emit_type_alias(def, "String")
                } else {
                    Ok("String".into())
                }
            }
        }
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

    /// Assumes that the given definition is an array and returns the corresponding
    /// vector type for it. Also takes a `bool` to specify whether we're defining the
    /// vector as a type alias or just fetching the type of the vector.
    fn emit_array(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        let it = def
            .items()
            .ok_or(PaperClipError::MissingArrayItem(self.def_name(def).ok()))?;

        let schema = it.read();
        let ty = self.build_def(&schema, false)?;
        let ty = String::from("Vec<") + &ty + ">";
        if define {
            self.emit_type_alias(def, &ty)
        } else {
            Ok(ty)
        }
    }

    /// Assumes that the given definition is an object and returns the corresponding
    /// Rust struct. Also takes a `bool` to specify whether we're defining the struct
    /// or just reusing the type.
    fn emit_object(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        if let Some(map) = self.try_emit_map(def, define) {
            return map;
        }

        if !define {
            // Use absolute paths to save some pain.
            // FIXME: This assumes that the working directory is the immediate
            // child module of a crate. We should support custom prefixes.
            let mut mod_path = String::from("crate");
            let mut iter = self.def_ns_name(def)?.peekable();
            while let Some(mut c) = iter.next() {
                mod_path.push_str("::");
                if iter.peek().is_none() {
                    mod_path.push_str(&c);
                    mod_path.push_str("::");
                    c = c.to_camel_case();
                }

                mod_path.push_str(&c);
            }

            return Ok(mod_path);
        }

        self.emit_struct(def)
    }

    /// Checks if the given definition is a simple map and returns the corresponding
    /// `BTreeMap`. Also takes a `bool` to specify whether we're defining the
    /// map as a type alias or just fetching the type of the map.
    fn try_emit_map(&self, def: &Self::Definition, define: bool) -> Option<Result<String, Error>> {
        def.additional_properties().map(|s| {
            let schema = s.read();
            let ty = self.build_def(&schema, false)?;
            if define {
                self.emit_type_alias(def, &format!("std::collections::BTreeMap<String, {}>", ty))
            } else {
                Ok(format!("std::collections::BTreeMap<String, {}>", ty))
            }
        })
    }

    /// Helper for `emit_object` - This returns the Rust struct definition for the
    /// given schema definition.
    fn emit_struct(&self, def: &Self::Definition) -> Result<String, Error> {
        let name = self.def_name(def)?;
        let mut final_gen = String::new();
        final_gen.push_str("\n#[derive(Debug, Clone, Deserialize, Serialize)]");
        final_gen.push_str("\npub struct ");
        final_gen.push_str(&name);
        final_gen.push_str(" {");

        if let Some(props) = def.properties() {
            props
                .iter()
                .try_for_each(|(name, prop)| -> Result<(), Error> {
                    let is_required = def.is_required_property(name);
                    let mut new_name = name.to_snek_case();
                    // Check if the field matches a Rust keyword and add '_' suffix.
                    if RUST_KEYWORDS.iter().any(|&k| k == new_name) {
                        new_name.push('_');
                    }

                    // If we've modified the name, add a serde attribute for renaming.
                    if new_name != name.as_str() {
                        final_gen.push_str("\n    #[serde(rename = \"");
                        final_gen.push_str(&name);
                        final_gen.push_str("\")]");
                    }

                    final_gen.push_str("\n    pub ");
                    final_gen.push_str(&new_name);
                    final_gen.push_str(": ");
                    if !is_required {
                        final_gen.push_str("Option<");
                    }

                    let schema = prop.read();
                    let ty = self.build_def(&schema, false)?;

                    // If it's a cyclic type, box it.
                    if schema.is_cyclic() {
                        final_gen.push_str("Box<");
                        final_gen.push_str(&ty);
                        final_gen.push_str(">");
                    } else {
                        final_gen.push_str(&ty);
                    }

                    if !is_required {
                        final_gen.push_str(">");
                    }

                    final_gen.push(',');
                    Ok(())
                })?
        }

        final_gen.push_str("\n}");
        Ok(final_gen)
    }

    /// Given a definition and a type, emit the type alias using the definition's name.
    fn emit_type_alias(&self, def: &Self::Definition, ty: &str) -> Result<String, Error> {
        self.def_name(def)
            .map(|n| format!("pub type {} = {};\n", n, ty))
    }
}

///
pub trait OperationEmitter: SchemaEmitter {
    // fn create_def_exts(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
    //     //
    // }
}
