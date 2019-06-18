use super::{object::ApiObject, CrateMeta};
#[cfg(feature = "cli")]
use crate::error::PaperClipError;
use failure::Error;
use heck::CamelCase;
#[cfg(feature = "cli")]
use heck::SnekCase;
use itertools::Itertools;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
#[cfg(feature = "cli")]
use std::fs;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
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
    /// Module prefix for using in generated code.
    pub mod_prefix: &'static str,
    /// Base path for API.
    pub base_url: &'static str,
    /// Maps parent mod to immediate children. Used for declaring modules.
    pub(super) mod_children: Rc<RefCell<HashMap<PathBuf, HashSet<ChildModule>>>>,
    /// Holds generated struct definitions for leaf modules.
    pub(super) def_mods: Rc<RefCell<HashMap<PathBuf, ApiObject>>>,
    /// If crate metadata is specified, then `lib.rs` and `Cargo.toml` are generated
    /// along with the modules. This is gated behind `"cli"` feature.
    #[cfg(feature = "cli")]
    crate_meta: Option<Rc<RefCell<CrateMeta>>>,
    /// Unit types used by builders.
    unit_types: Rc<RefCell<HashSet<String>>>,
    /// Generated CLI code.
    cli_content: Rc<RefCell<String>>,
}

/// Indicates a child module in codegen working directory.
#[derive(Debug, Clone, Eq)]
pub(super) struct ChildModule {
    /// Name of this child module.
    pub name: String,
    /// Whether this module is the final child.
    pub is_final: bool,
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
            let mut contents = String::new();

            if rel_parent.parent().is_none() && self.is_crate() {
                mod_path = self.root_module_path();

                contents.push_str(
                    "
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
",
                );

                if mod_path.ends_with("main.rs") {
                    contents.push_str(
                        "
#[macro_use]
extern crate clap;
",
                    )
                }
            } else {
                mod_path.push("mod.rs");
            }

            for child in children.iter().sorted_by(|a, b| a.name.cmp(&b.name)) {
                writeln!(
                    contents,
                    "
pub mod {name} {{
    include!(\"./{path}\");
}}",
                    name = child.name,
                    path = if child.is_final {
                        child.name.clone() + ".rs"
                    } else {
                        child.name.clone() + "/mod.rs"
                    }
                )?;
            }

            self.write_contents(&contents, &mod_path)?;
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
        let module_prefix = format!("{}::", self.mod_prefix.trim_matches(':'));
        info!("Adding builders to definitions.");
        let mut unit_types = self.unit_types.borrow_mut();
        let def_mods = self.def_mods.borrow();
        let mut cli_content = self.cli_content.borrow_mut();

        for (mod_path, object) in &*def_mods {
            let mut builder_content = String::new();
            let mut repr = object.impl_repr();
            for builder in object.builders(&module_prefix) {
                builder
                    .struct_fields_iter()
                    .filter(|f| f.prop.is_required())
                    .for_each(|f| {
                        unit_types.insert(f.name.to_camel_case());
                    });

                builder_content.push('\n');
                let _ = write!(builder_content, "{}", builder);
                builder_content.push('\n');
                let _ = write!(builder_content, "{}", builder.impl_repr());
                repr.builders.push(builder);
            }

            repr.write_clap_yaml(&mut *cli_content)?;
            let mut impl_content = String::from("\n");
            let _ = write!(impl_content, "{}", repr);

            self.append_contents(&impl_content, mod_path)?;
            self.append_contents(&builder_content, mod_path)?;
        }

        Ok(())
    }

    /// Once the builders have been added, we can add unit types
    /// and other dependencies.
    pub(crate) fn add_deps(&self) -> Result<(), Error> {
        let mut module = self.root_module_path();
        let types = self.unit_types.borrow();
        let mut content = String::new();
        content.push_str(
            "
pub mod generics {
    include!(\"./generics.rs\");
}
",
        );
        self.append_contents(&content, &module)?;

        content.clear();
        module.set_file_name("generics.rs");
        content.push_str("pub trait Optional {}");

        for ty in &*types {
            content.push_str("\npub struct Missing");
            content.push_str(ty);
            content.push_str(";");
            content.push_str("\nimpl Optional for Missing");
            content.push_str(ty);
            content.push_str(" {}");
            content.push_str("\npub struct ");
            content.push_str(ty);
            content.push_str("Exists;");
            content.push_str("\nimpl Optional for ");
            content.push_str(ty);
            content.push_str("Exists {}");
        }

        content.push_str("\n");
        self.write_contents(&content, &module)?;
        self.add_cli_deps_if_needed()?;
        self.create_manifest()
    }

    /// Once the builders have been added, we can add API client dependencies.
    pub(crate) fn add_client_deps(&self) -> Result<(), Error> {
        let module = self.root_module_path();
        let deser = "resp.json::<Self::Output>().map_err(ApiError::Reqwest)";
        let content = format!("
pub mod client {{
    use futures::Future;

    /// Common API errors.
    #[derive(Debug, Fail)]
    pub enum ApiError {{
        #[fail(display = \"API request failed for path: {{}} (code: {{}})\", _0, _1)]
        Failure(String, reqwest::StatusCode),
        #[fail(display = \"An error has occurred while performing the API request: {{}}\", _0)]
        Reqwest(reqwest::Error),
    }}

    /// Represents an API client.
    pub trait ApiClient {{
        /// Base path for this API.
        fn base_url(&self) -> &'static str {{ \"{base_url}\" }}

        /// Consumes a method and a relative path and produces a request builder for a single API call.
        fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder;
    }}

    impl ApiClient for reqwest::r#async::Client {{
        #[inline]
        fn request_builder(&self, method: reqwest::Method, rel_path: &str) -> reqwest::r#async::RequestBuilder {{
            self.request(method, &(String::from(self.base_url()) + rel_path))
        }}
    }}

    /// A trait for indicating that the implementor can send an API call.
    pub trait Sendable {{
        /// The output object from this API request.
        type Output: serde::de::DeserializeOwned + Send + 'static;

        /// HTTP method used by this call.
        const METHOD: reqwest::Method;

        /// Relative URL for this API call formatted appropriately with parameter values.
        ///
        /// **NOTE:** This URL **must** begin with `/`.
        fn rel_path(&self) -> std::borrow::Cow<'static, str>;

        /// Modifier for this object. Builders override this method if they
        /// wish to add query parameters, set body, etc.
        fn modify(&self, req: reqwest::r#async::RequestBuilder) -> reqwest::r#async::RequestBuilder {{
            req
        }}

        /// Sends the request and returns a future for the response object.
        fn send(&self, client: &dyn ApiClient) -> Box<dyn Future<Item=Self::Output, Error=ApiError> + Send> {{
            Box::new(self.send_raw(client).and_then(|mut resp| {{
                {deserializer}
            }})) as Box<_>
        }}

        /// Convenience method for returning a raw response after sending a request.
        fn send_raw(&self, client: &dyn ApiClient) -> Box<dyn Future<Item=reqwest::r#async::Response, Error=ApiError> + Send> {{
            let rel_path = self.rel_path();
            let req = client.request_builder(Self::METHOD, &rel_path);
            Box::new(self.modify(req).send().map_err(ApiError::Reqwest).and_then(move |resp| {{
                if resp.status().is_success() {{
                    futures::future::ok(resp)
                }} else {{
                    futures::future::err(ApiError::Failure(rel_path.into_owned(), resp.status()).into())
                }}
            }})) as Box<_>
        }}
    }}
}}
", deserializer=deser, base_url=self.base_url);

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

    /// Adds CLI-related deps for the given object (if needed).
    fn add_cli_deps_if_needed(&self) -> Result<(), Error> {
        let root = self.root_module_path();
        if !root.ends_with("main.rs") {
            return Ok(());
        }

        if let Some(m) = self.infer_crate_meta()? {
            let meta = m.borrow();
            let clap_yaml = root.with_file_name("app.yaml");
            let base_content = format!(
                "
name: {}
version: {:?}

settings:
- SubcommandRequiredElseHelp

subcommands:",
                meta.name.as_ref().unwrap(),
                meta.version.as_ref().unwrap()
            );

            self.write_contents(&base_content, &clap_yaml)?;

            self.append_contents(&*self.cli_content.borrow(), &clap_yaml)?;
        }

        let content = String::from(
            "
use clap::App;

fn main() {
    let yml = load_yaml!(\"app.yaml\");
    let matches = App::from_yaml(yml).get_matches();
}
",
        );

        self.append_contents(&content, &root)
    }
}

/* Feature-specific impls */

#[cfg(feature = "cli")]
impl EmitterState {
    /// Sets the crate metadata for this session.
    pub fn set_meta(&mut self, meta: CrateMeta) {
        self.crate_meta = Some(Rc::new(RefCell::new(meta)));
    }

    /// Checks whether this session is for emitting a crate.
    fn is_crate(&self) -> bool {
        self.crate_meta.is_some()
    }

    /// Returns the path to the root module.
    fn root_module_path(&self) -> PathBuf {
        if let Some(m) = self.crate_meta.as_ref() {
            let meta = m.borrow();
            if meta.is_cli {
                self.working_dir.join("main.rs")
            } else {
                self.working_dir.join("lib.rs")
            }
        } else {
            self.working_dir.join("mod.rs")
        }
    }

    /// Creates a Cargo.toml manifest in the working directory (if it's a crate).
    fn create_manifest(&self) -> Result<(), Error> {
        let mut man_path = self.root_module_path();
        let is_cli = man_path.ends_with("main.rs");
        man_path.set_file_name("Cargo.toml");

        let m = match self.infer_crate_meta()? {
            Some(c) => c,
            None => return Ok(()),
        };

        let meta = m.borrow();
        if self.is_crate() {
            let content = format!(
                "[package]
name = {:?}
version = {:?}
authors = {:?}
edition = \"2018\"

{}

[dependencies]
failure = \"0.1\"
failure_derive = \"0.1\"
futures = \"0.1\"
reqwest = \"0.9\"
serde = \"1.0\"
serde_derive = \"1.0\"
{}
[workspace]
",
                meta.name.as_ref().unwrap(),
                meta.version.as_ref().unwrap(),
                meta.authors.as_ref().unwrap(),
                if is_cli {
                    format!(
                        "[[bin]]\nname = {:?}\npath = \"main.rs\"",
                        meta.name.as_ref().unwrap()
                    )
                } else {
                    "[lib]\npath = \"lib.rs\"".into()
                },
                if is_cli {
                    "clap = { version = \"2.33\", features = [\"yaml\"] }\n"
                } else {
                    ""
                },
            );

            self.write_contents(&content, &man_path)?;
        }

        Ok(())
    }

    /// Validates crate metadata, sets the unset fields and returns a reference.
    fn infer_crate_meta(&self) -> Result<Option<Rc<RefCell<CrateMeta>>>, Error> {
        if let Some(m) = self.crate_meta.as_ref() {
            let mut meta = m.borrow_mut();
            if meta.name.is_none() {
                meta.name = Some(
                    fs::canonicalize(&self.working_dir)?
                        .file_name()
                        .ok_or(PaperClipError::InvalidCodegenDirectory)?
                        .to_string_lossy()
                        .into_owned()
                        .to_snek_case(),
                );
            }

            if meta.version.is_none() {
                meta.version = Some("0.1.0".into());
            }

            if meta.authors.is_none() {
                let (mut name, email) = super::author::discover()?;
                if let Some(e) = email {
                    name.push_str(" <");
                    name.push_str(&e);
                    name.push_str(">");
                }

                meta.authors = Some(vec![name]);
            }
        }

        Ok(self.crate_meta.clone())
    }
}

#[cfg(not(feature = "cli"))]
impl EmitterState {
    /// This is a no-op.
    pub fn set_meta(&mut self, _: CrateMeta) {}

    /// Always returns `Ok(None)`
    fn infer_crate_meta(&self) -> Result<Option<Rc<RefCell<CrateMeta>>>, Error> {
        Ok(None)
    }

    /// Always returns the path to `mod.rs` in root.
    fn root_module_path(&self) -> PathBuf {
        self.working_dir.join("mod.rs")
    }

    /// This always returns `false`.
    fn is_crate(&self) -> bool {
        false
    }

    /// Always returns `Ok(())`
    fn create_manifest(&self) -> Result<(), Error> {
        Ok(())
    }
}

/* Other impls */

impl Default for EmitterState {
    fn default() -> EmitterState {
        EmitterState {
            working_dir: PathBuf::from("."),
            mod_prefix: "crate::",
            ns_sep: ".",
            #[cfg(feature = "cli")]
            crate_meta: None,
            base_url: "https://example.com",
            def_mods: Rc::new(RefCell::new(HashMap::new())),
            mod_children: Rc::new(RefCell::new(HashMap::new())),
            unit_types: Rc::new(RefCell::new(HashSet::new())),
            cli_content: Rc::new(RefCell::new(String::new())),
        }
    }
}

impl Hash for ChildModule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ChildModule {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
