use super::{
    models::{Api, DataType, DataTypeFormat},
    Schema,
};
use crate::error::PaperClipError;
use failure::Error;
use heck::{CamelCase, SnekCase};

use std::fmt::Debug;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::path::PathBuf;

pub(crate) trait SchemaExt: Schema {
    fn rust_unit_type_str(&self) -> Option<&'static str> {
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
}

impl Default for Config {
    fn default() -> Config {
        Config {
            working_dir: PathBuf::from("."),
            ns_sep: ".",
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

    fn def_name(&self, def: &Self::Definition) -> Result<String, Error> {
        let config = self.config();
        let n = def
            .name()
            .and_then(|n| n.split(config.ns_sep).last())
            .ok_or(PaperClipError::InvalidDefinitionName)?;
        Ok(n.to_camel_case())
    }

    fn def_mod_path(&self, def: &Self::Definition) -> Result<PathBuf, Error> {
        let name = def.name().ok_or(PaperClipError::InvalidDefinitionName)?;
        let config = self.config();
        let mut path = config.working_dir.clone();
        path.extend(name.split(config.ns_sep).map(SnekCase::to_snek_case));
        path.pop(); // pop final component (as it's used for name)
        Ok(path)
    }

    fn build_def(&self, def: &Self::Definition, define: bool) -> Result<String, Error> {
        trace!("Definition: {:?}", def);
        let name = self.def_name(def);
        if let Some(ty) = def.rust_unit_type_str() {
            trace!("Matches unit type: {}", ty);
            if define {
                return Ok(format!("type {} = {};\n", name?, ty));
            }

            return Ok(ty.to_owned());
        }

        match def.data_type() {
            Some(DataType::Array) => {
                let it = def.items().ok_or(PaperClipError::MissingArrayItem(
                    name.as_ref().ok().map(|n| n.clone()),
                ))?;
                let schema = it.borrow();
                let ty = self.build_def(&schema, false)?; // just return the type
                let ty = String::from("Vec<") + &ty + ">";
                if define {
                    Ok(format!("type {} = {};\n", name?, ty))
                } else {
                    Ok(ty)
                }
            }
            Some(DataType::Object) => {
                // FIXME: Refactor needed.
                if let Some(s) = def.additional_properties() {
                    let schema = s.borrow();
                    let ty = self.build_def(&schema, false)?;
                    if define {
                        return Ok(format!(
                            "type {} = std::collections::BTreeMap<String, {}>",
                            name?, ty
                        ));
                    } else {
                        return Ok(format!("std::collections::BTreeMap<String, {}>", ty));
                    }
                }

                if !define {
                    return name;
                }

                let name = name?;
                let mut final_gen = String::new();
                // if let Some(s) = def.description() {
                //     final_gen.push_str("/// ");
                //     final_gen.push_str(s);
                //     final_gen.push('\n');
                // }

                final_gen.push_str("#[derive(Debug, Clone, Deserialize, Serialize)]");
                final_gen.push_str("\npub struct ");
                final_gen.push_str(&name);
                final_gen.push_str(" {");
                match def.properties() {
                    Some(props) => {
                        props
                            .iter()
                            .try_for_each(|(name, prop)| -> Result<(), Error> {
                                final_gen.push_str("\npub ");
                                final_gen.push_str(name);
                                final_gen.push_str(": ");
                                let schema = prop.borrow();
                                let ty = self.build_def(&schema, false)?;
                                final_gen.push_str(&ty);
                                final_gen.push(',');
                                Ok(())
                            })?
                    }
                    None => (),
                }

                final_gen.push_str("\n};");
                Ok(final_gen)
            }
            Some(_) => unreachable!("bleh?"), // we've already handled everything else
            None => {
                if define {
                    // default to String
                    Ok(format!("type {} = String;\n", name?))
                } else {
                    Ok("String".into())
                }
            }
        }
    }

    fn create_def_from_root(&self, def: &Self::Definition) -> Result<(), Error> {
        let abs_path = self.def_mod_path(def)?;
        if !abs_path.exists() {
            fs::create_dir_all(&abs_path)?;
        }

        let config = self.config();
        let rel_path = abs_path
            .strip_prefix(&config.working_dir)
            .ok()
            .ok_or(PaperClipError::InvalidDefinitionPath(abs_path.clone()))?;
        for (i, path) in rel_path.ancestors().enumerate() {
            let mod_path = config.working_dir.join(path).join("mod.rs");
            debug!("Touching mod: {}", mod_path.display());
            let mut fd = BufWriter::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&mod_path)?,
            );

            if i != 0 {
                continue;
            }

            let def_str = self.build_def(def, true)?;
            fd.write(def_str.as_bytes())?;
            fd.write(b"\n")?;
        }

        Ok(())
    }

    fn create_defs(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        for (name, schema) in &api.definitions {
            info!("Creating definition {}", name);
            let schema = schema.borrow();
            self.create_def_from_root(&schema)?;
        }

        Ok(())
    }
}
