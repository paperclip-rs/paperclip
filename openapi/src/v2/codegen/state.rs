use super::object::ApiObject;
use failure::Error;
use heck::CamelCase;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Holds the state for your schema emitter.
#[derive(Debug, Clone)]
pub struct EmitterState {
    /// Working directory - the path in which the necessary modules are generated.
    pub working_dir: PathBuf,
    /// Namespace separation string.
    pub ns_sep: &'static str,
    /// Maps parent mod to immediate children. Used for declaring modules.
    pub(super) mod_children: Rc<RefCell<HashMap<PathBuf, HashSet<String>>>>,
    /// Holds generated struct definitions for leaf modules.
    pub(super) def_mods: Rc<RefCell<HashMap<PathBuf, ApiObject>>>,
    /// Unit types used by builders.
    unit_types: Rc<RefCell<HashSet<String>>>,
    /// Root module emitted by codegen.
    root_module: Rc<RefCell<Option<String>>>,
}

impl EmitterState {
    /// Once the emitter has generated the struct definitions,
    /// we can call this method to generate the module declarations
    /// from root.
    pub(crate) fn declare_modules(&self) -> Result<(), Error> {
        info!("Writing module declarations.");
        let mods = self.mod_children.borrow();
        for (rel_parent, children) in &*mods {
            let mut mod_path = self.working_dir.join(&rel_parent);
            mod_path.push("mod.rs");

            let mut contents = String::new();
            for child in children {
                contents.push_str("pub mod ");
                contents.push_str(child);
                contents.push_str(";\n");
            }

            self.write_contents(&contents, &mod_path)?;
        }

        if let Some(p) = mods.keys().next() {
            let mut some_path = PathBuf::from(p);
            loop {
                match some_path.parent() {
                    Some(p) if p.parent().is_some() => some_path = p.into(),
                    _ => break,
                }
            }

            self.root_module
                .borrow_mut()
                .replace(some_path.to_string_lossy().into_owned());
        }

        Ok(())
    }

    /// Once the emitter has generated the struct definitions,
    /// we can call this method to write the definitions to leaf modules.
    pub(crate) fn write_definitions(&self) -> Result<(), Error> {
        let def_mods = self.def_mods.borrow();
        info!("Writing definitions.");
        for (mod_path, object) in &*def_mods {
            let contents = object.to_string();
            self.write_contents(&contents, mod_path)?;
        }

        Ok(())
    }

    /// Once the emitter has collected requirements for paths,
    /// we can use this method to add builder structs and their impls.
    pub(crate) fn add_builders(&self) -> Result<(), Error> {
        info!("Adding builders to definitions.");
        let mut unit_types = self.unit_types.borrow_mut();
        let def_mods = self.def_mods.borrow();
        for (mod_path, object) in &*def_mods {
            let mut contents = String::from("\n");
            let _ = write!(contents, "{}", object.impl_repr());
            for builder in object.builders() {
                builder
                    .struct_fields_iter()
                    .filter(|(_, _, prop)| prop.is_required())
                    .for_each(|(name, _, _)| {
                        unit_types.insert(name.to_camel_case());
                    });

                contents.push('\n');
                let _ = write!(contents, "{}", builder);
            }

            self.append_contents(&contents, mod_path)?;
        }

        Ok(())
    }

    /// Once the builders have been added, we can add unit types
    /// and other dependencies.
    pub(crate) fn add_deps(&self) -> Result<(), Error> {
        let module = match &*self.root_module.borrow() {
            Some(p) => self.working_dir.join(p).join("mod.rs"),
            None => {
                error!("No root module to generate deps.");
                return Ok(());
            }
        };

        let types = self.unit_types.borrow();
        let mut content = String::new();
        content.push_str("\npub mod prelude {\n");

        for ty in &*types {
            content.push_str("    pub struct Missing");
            content.push_str(ty);
            content.push_str(";\n");
            content.push_str("    pub struct ");
            content.push_str(ty);
            content.push_str("Optional;\n");
            content.push_str("    pub struct ");
            content.push_str(ty);
            content.push_str("Exists;\n");
        }

        content.push_str("}\n");
        self.append_contents(&content, &module)
    }

    /// Writes the given contents to a file at the given path (truncating the file if it exists).
    fn write_contents(&self, contents: &str, path: &Path) -> Result<(), Error> {
        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        fd.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// Appends the given contents to an existing file at the given path.
    ///
    /// **NOTE:** This doesn't create a file if it is non-existent.
    fn append_contents(&self, contents: &str, path: &Path) -> Result<(), Error> {
        let mut fd = OpenOptions::new().append(true).open(path)?;
        fd.write_all(contents.as_bytes())?;
        Ok(())
    }
}

impl Default for EmitterState {
    fn default() -> EmitterState {
        EmitterState {
            working_dir: PathBuf::from("."),
            ns_sep: ".",
            def_mods: Rc::new(RefCell::new(HashMap::new())),
            mod_children: Rc::new(RefCell::new(HashMap::new())),
            unit_types: Rc::new(RefCell::new(HashSet::new())),
            root_module: Rc::new(RefCell::new(None)),
        }
    }
}
