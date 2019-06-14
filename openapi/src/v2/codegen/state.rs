use super::{object::ApiObject, CrateMeta};
use failure::Error;
use heck::CamelCase;
use itertools::Itertools;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
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
    /// If crate metadata is specified, then `lib.rs` and `Cargo.toml` are generated
    /// along with the modules.
    pub crate_meta: Option<CrateMeta>,
    /// Base path for API.
    pub base_url: &'static str,
    /// Whether the emitter should emit `mod.rs` (module) or `lib.rs` (crate) in root.
    pub is_crate: bool,
    /// Maps parent mod to immediate children. Used for declaring modules.
    pub(super) mod_children: Rc<RefCell<HashMap<PathBuf, HashSet<ChildModule>>>>,
    /// Holds generated struct definitions for leaf modules.
    pub(super) def_mods: Rc<RefCell<HashMap<PathBuf, ApiObject>>>,
    /// Unit types used by builders.
    unit_types: Rc<RefCell<HashSet<String>>>,
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

            if rel_parent.parent().is_none() && self.is_crate {
                contents.push_str(
                    "
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
",
                );
                mod_path.push("lib.rs");
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
        // FIXME: Fix this when we support custom prefixes.
        let module_prefix = format!("{}::", self.mod_prefix.trim_matches(':'));

        info!("Adding builders to definitions.");
        let mut unit_types = self.unit_types.borrow_mut();
        let def_mods = self.def_mods.borrow();
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
        let module = self.root_module_path();
        let types = self.unit_types.borrow();
        let mut content = String::new();
        content.push_str("\npub mod generics {");

        content.push_str("\n    pub trait Optional {}");

        for ty in &*types {
            content.push_str("\n\n    pub struct Missing");
            content.push_str(ty);
            content.push_str(";");
            content.push_str("\n    impl Optional for Missing");
            content.push_str(ty);
            content.push_str(" {}");
            content.push_str("\n    pub struct ");
            content.push_str(ty);
            content.push_str("Exists;");
            content.push_str("\n    impl Optional for ");
            content.push_str(ty);
            content.push_str("Exists {}");
        }

        content.push_str("\n}\n");
        self.append_contents(&content, &module)
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

    /// Returns the path to the root module.
    fn root_module_path(&self) -> PathBuf {
        if self.is_crate {
            self.working_dir.join("lib.rs")
        } else {
            self.working_dir.join("mod.rs")
        }
    }
}

impl Default for EmitterState {
    fn default() -> EmitterState {
        EmitterState {
            crate_meta: None,
            working_dir: PathBuf::from("."),
            mod_prefix: "crate::",
            ns_sep: ".",
            is_crate: false,
            base_url: "https://example.com",
            def_mods: Rc::new(RefCell::new(HashMap::new())),
            mod_children: Rc::new(RefCell::new(HashMap::new())),
            unit_types: Rc::new(RefCell::new(HashSet::new())),
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
