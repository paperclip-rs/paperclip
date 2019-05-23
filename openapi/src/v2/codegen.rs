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

pub(crate) trait SchemaExt: Schema {
    fn matching_unit_type(&self) -> Option<&'static str> {
        return match self.format() {
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
        };
    }
}

impl<T: Schema> SchemaExt for T {}

#[derive(Debug, Clone)]
pub struct Config {
    pub working_dir: PathBuf,
    pub ns_sep: &'static str,
    mod_children: Rc<RefCell<HashMap<PathBuf, HashSet<String>>>>,
    def_mods: Rc<RefCell<HashMap<PathBuf, String>>>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            working_dir: PathBuf::from("."),
            ns_sep: ".",
            def_mods: Rc::new(RefCell::new(HashMap::new())),
            mod_children: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

pub struct DefaultEmitter<S> {
    config: Config,
    _schema: PhantomData<S>,
}

impl<S> From<Config> for DefaultEmitter<S> {
    fn from(config: Config) -> Self {
        DefaultEmitter {
            config,
            _schema: PhantomData,
        }
    }
}

impl<S: Schema + Debug> SchemaEmitter for DefaultEmitter<S> {
    type Definition = S;

    fn config(&self) -> &Config {
        &self.config
    }
}

pub trait SchemaEmitter {
    type Definition: Schema + Debug;

    fn config(&self) -> &Config;

    fn write_contents(&self, contents: String, path: &Path) -> Result<(), Error> {
        let mut fd = BufWriter::new(OpenOptions::new().create(true).write(true).open(path)?);
        fd.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn create_defs(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        for (name, schema) in &api.definitions {
            info!("Creating definition {}", name);
            let schema = schema.read();
            self.generate_def_from_root(&schema)?;
        }

        let config = self.config();
        let mut mods = config.mod_children.borrow_mut();
        info!("Adding mod declarations.");
        for (rel_parent, children) in mods.drain() {
            let mut mod_path = config.working_dir.join(&rel_parent);
            mod_path.push("mod.rs");

            let mut contents = String::new();
            for child in children {
                contents.push_str("pub mod ");
                contents.push_str(&child);
                contents.push_str(";\n");
            }

            self.write_contents(contents, &mod_path)?;
        }

        let mut def_mods = config.def_mods.borrow_mut();
        info!("Writing definitions.");
        for (mod_path, contents) in def_mods.drain() {
            self.write_contents(contents, &mod_path)?;
        }

        Ok(())
    }

    fn generate_def_from_root(&self, def: &Self::Definition) -> Result<(), Error> {
        let full_path = self.def_mod_path(def)?;
        if !full_path.exists() {
            fs::create_dir_all(&full_path)?;
        }

        let config = self.config();
        let path_error = PaperClipError::InvalidDefinitionPath(full_path.clone());
        let rel_path = full_path
            .strip_prefix(&config.working_dir)
            .map_err(|_| path_error)?;

        let mut mods = config.mod_children.borrow_mut();
        for path in rel_path.ancestors() {
            match (path.parent(), path.file_name()) {
                (Some(parent), Some(name)) if parent.parent().is_some() => {
                    let entry = mods.entry(parent.into()).or_insert(HashSet::new());
                    entry.insert(name.to_string_lossy().into_owned());
                }
                _ => (),
            }
        }

        let mut def_mods = config.def_mods.borrow_mut();
        let def_str = self.build_def(def, true)?;
        let mod_path = full_path.join("mod.rs");
        let entry = def_mods.entry(mod_path).or_insert(String::new());
        entry.push_str(&def_str);
        entry.push('\n');

        Ok(())
    }

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

    fn def_ns_name<'a>(
        &self,
        def: &'a Self::Definition,
    ) -> Result<Box<Iterator<Item = String> + 'a>, Error> {
        let config = self.config();
        def.name()
            .map(|n| n.split(config.ns_sep).map(SnekCase::to_snek_case))
            .ok_or(PaperClipError::InvalidDefinitionName.into())
            .map(|i| Box::new(i) as Box<_>)
    }

    fn def_name(&self, def: &Self::Definition) -> Result<String, Error> {
        Ok(self
            .def_ns_name(def)?
            .last()
            .map(|s| s.to_camel_case())
            .expect("last item always exists for split?"))
    }

    fn def_mod_path(&self, def: &Self::Definition) -> Result<PathBuf, Error> {
        let config = self.config();
        let mut path = config.working_dir.clone();
        path.extend(self.def_ns_name(def)?);
        path.pop(); // pop final component (as it's used for name)
        Ok(path)
    }

    fn emit_array(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        let it = def.items().ok_or(PaperClipError::MissingArrayItem(
            self.def_name(def).ok().map(|n| n.clone()),
        ))?;

        let schema = it.read();
        let ty = self.build_def(&schema, false)?;
        let ty = String::from("Vec<") + &ty + ">";
        if define {
            self.emit_type_alias(def, &ty)
        } else {
            Ok(ty)
        }
    }

    fn emit_object(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        if let Some(map) = self.try_emit_map(def, define) {
            return map;
        }

        if !define {
            let mut mod_path = String::from("crate");
            let mut iter = self.def_ns_name(def)?.peekable();
            while let Some(mut c) = iter.next() {
                mod_path.push_str("::");
                if iter.peek().is_none() {
                    c = c.to_camel_case();
                }

                mod_path.push_str(&c);
            }

            return Ok(mod_path);
        }

        self.emit_struct(def)
    }

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
                    let mut snek = name.to_snek_case();
                    if RUST_KEYWORDS.iter().find(|&&k| k == snek).is_some() {
                        snek.push('_');
                    }

                    if snek != name.as_str() {
                        final_gen.push_str("\n#[serde(rename = \"");
                        final_gen.push_str(&name);
                        final_gen.push_str("\")]");
                    }

                    final_gen.push_str("\npub ");
                    final_gen.push_str(&snek);
                    final_gen.push_str(": ");
                    let schema = prop.read();
                    let ty = self.build_def(&schema, false)?;

                    if schema.is_cyclic() {
                        final_gen.push_str("Box<");
                        final_gen.push_str(&ty);
                        final_gen.push_str(">");
                    } else {
                        final_gen.push_str(&ty);
                    }

                    final_gen.push(',');
                    Ok(())
                })?
        }

        final_gen.push_str("\n}");
        Ok(final_gen)
    }

    fn emit_type_alias(&self, def: &Self::Definition, ty: &str) -> Result<String, Error> {
        self.def_name(def)
            .map(|n| format!("pub type {} = {};\n", n, ty))
    }
}
