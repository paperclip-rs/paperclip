use super::template::{self, TEMPLATE};
use super::{object::ApiObject, CrateMeta};
use crate::error::PaperClipError;
use crate::v2::models::{Coder, Coders, MediaRange, SpecFormat};
use failure::Error;
use heck::CamelCase;
#[cfg(feature = "cli")]
use heck::SnekCase;
use itertools::Itertools;
use url::Url;

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
#[cfg(feature = "cli")]
use std::fs;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::iter;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Holds the state for your schema emitter.
#[derive(Debug)]
pub struct EmitterState {
    /// Working directory - the path in which the necessary modules are generated.
    pub working_dir: PathBuf,
    /// Namespace separation string.
    pub ns_sep: &'static str,
    /// Module prefix for using in generated code.
    pub mod_prefix: &'static str,

    /* MARK: Private fields. */
    /// Base URL for the API.
    pub(super) base_url: RefCell<Url>,
    /// Fallback encoding when we don't have a choice (obtained from `Api.spec_format`).
    default_encoding: RefCell<SpecFormat>,
    /// Additional error variants for the API client (obtained from `Coders::errors`).
    errors: Rc<RefCell<Vec<CodingError>>>,
    /// If crate metadata is specified, then `lib.rs` and `Cargo.toml` are generated
    /// along with the modules. This is gated behind `"cli"` feature.
    #[cfg(feature = "cli")]
    crate_meta: Option<Rc<RefCell<CrateMeta>>>,

    /* MARK: Internal fields that should be reset for each session. */
    /// Maps parent mod to immediate children. Used for declaring modules.
    pub(super) mod_children: RefCell<HashMap<PathBuf, HashSet<ChildModule>>>,
    /// Holds generated struct definitions for leaf modules.
    pub(super) def_mods: RefCell<HashMap<PathBuf, Vec<ApiObject>>>,
    /// Relative paths
    pub(super) rel_paths: RefCell<HashSet<String>>,
    /// Combinations of media ranges we've encountered in operation responses.
    media_combinations: RefCell<BTreeSet<BTreeSet<String>>>,
    /// Unit types used by builders.
    unit_types: RefCell<HashSet<String>>,
    /// Generated CLI YAML for clap.
    cli_yaml: RefCell<String>,
    /// Generated match arms for clap subcommands and matches.
    cli_match_arms: RefCell<String>,
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
    /// Sets the base URL for this session.
    ///
    /// **NOTE:** Once `Emitter::generate` is called, this gets overridden
    /// by `host` and `basePath` fields in spec (if they exist).
    pub fn set_url(&self, url: &str) -> Result<(), Error> {
        let u = Url::parse(url).map_err(|e| PaperClipError::InvalidBasePathURL(url.into(), e))?;
        *self.base_url.borrow_mut() = u;
        Ok(())
    }

    /// Base URL for this API.
    ///
    /// **NOTE:** Once `Emitter::generate` is called, this gets overridden
    /// by `host` and `basePath` fields in spec (if they exist).
    pub fn base_url(&self) -> String {
        self.base_url.borrow().to_string()
    }

    /// Resets internal state-related information used by the emitter.
    pub(crate) fn reset_internal_fields(&self) {
        *self.mod_children.borrow_mut() = Default::default();
        *self.def_mods.borrow_mut() = Default::default();
        *self.rel_paths.borrow_mut() = Default::default();
        *self.unit_types.borrow_mut() = Default::default();
        *self.cli_yaml.borrow_mut() = Default::default();
        *self.cli_match_arms.borrow_mut() = Default::default();
        *self.media_combinations.borrow_mut() = Default::default();
    }

    /// Sets the media type information for encoder/decoders.
    pub(crate) fn set_media_info(&self, spec_format: SpecFormat, coders: &Coders) {
        *self.default_encoding.borrow_mut() = spec_format;
        self.add_response_ranges(iter::once(spec_format.mime()));

        *self.errors.borrow_mut() = coders
            .errors()
            .map(|(m, p)| CodingError {
                media_type: m.into(),
                variant: m.replace('*', "wildcard").to_camel_case(),
                ty_path: p.into(),
            })
            .collect();
    }

    /// Adds the accepted media ranges for some response. This is called
    /// for each path operation.
    pub(crate) fn add_response_ranges<'a, I>(&self, iter: I)
    where
        I: Iterator<Item = &'a MediaRange>,
    {
        self.media_combinations
            .borrow_mut()
            .insert(iter.map(|s| s.0.as_ref().to_owned()).collect());
    }

    /// Once the emitter has generated the struct definitions,
    /// we can call this method to generate the module declarations
    /// from root.
    pub(crate) fn declare_modules(&self) -> Result<(), Error> {
        info!("Writing module declarations.");
        let is_app = self.is_cli()?;
        let mods = self.mod_children.borrow();
        for (rel_parent, children) in &*mods {
            let mut mod_path = self.working_dir.join(&rel_parent);
            let mut contents = String::new();

            if rel_parent.parent().is_none() && self.is_crate() {
                mod_path = self.root_module_path();
                contents.push_str(
                    "
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;
",
                );

                if is_app {
                    contents.push_str(
                        "
#[macro_use]
extern crate clap;

pub mod cli {
    include!(\"./cli.rs\");
}
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
        for (i, (mod_path, object)) in def_mods
            .iter()
            .flat_map(move |(p, l)| l.iter().map(move |o| (p, o)).enumerate())
        {
            let contents = object.to_string();
            if i == 0 {
                self.write_contents(&contents, mod_path)?;
            } else {
                self.append_contents(&contents, mod_path)?;
            }
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
        let mut cli_yaml = self.cli_yaml.borrow_mut();
        let mut match_arms = self.cli_match_arms.borrow_mut();
        let is_cli = self.is_cli()?;

        for (mod_path, object) in def_mods
            .iter()
            .flat_map(move |(p, l)| l.iter().map(move |o| (p, o)))
        {
            let mut builder_content = String::new();
            let repr = object.impl_repr(&module_prefix);
            for builder in &repr.builders {
                builder
                    .struct_fields_iter()
                    .filter(|f| f.prop.is_required())
                    .for_each(|f| {
                        unit_types.insert(f.name.to_camel_case());
                    });

                builder_content.push('\n');
                let _ = write!(builder_content, "{}", builder);
                builder_content.push('\n');
                let inner_repr = builder.impl_repr();
                let _ = write!(builder_content, "{}", inner_repr);
                if is_cli {
                    inner_repr.write_arg_parsing(&mut builder_content)?;
                }
            }

            if is_cli {
                repr.write_clap_yaml(&mut *cli_yaml)?;
                repr.write_arg_match_arms(&mut *match_arms)?;
            }

            let mut impl_content = String::from("\n");
            write!(impl_content, "{}", repr)?;

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

        for ty in &*types {
            content.push_str("\npub struct Missing");
            content.push_str(ty);
            content.push_str(";");
            content.push_str("\npub struct ");
            content.push_str(ty);
            content.push_str("Exists;");
        }

        content.push_str("\n");
        self.write_contents(&content, &module)?;
        self.add_cli_deps_if_needed()?;
        self.create_manifest()
    }

    /// Once the builders have been added, we can add API client dependencies.
    pub(crate) fn add_client_deps(&self) -> Result<(), Error> {
        let module = self.root_module_path();
        let contents = template::render(
            TEMPLATE::CLIENT_MOD,
            &ClientModContext {
                coder: &*self.default_encoding.borrow().coder(),
                default_range_index: self
                    .media_combinations
                    .borrow()
                    .iter()
                    .position(|r| {
                        r.len() == 1 && r.contains(self.default_encoding.borrow().mime().0.as_ref())
                    })
                    .expect("expected default media range to exist"),
                media_combinations: &*self.media_combinations.borrow(),
                errors: &*self.errors.borrow(),
                base_url: self.base_url.borrow().as_str(),
            },
        )?;

        self.append_contents(&contents, &module)
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
        if !self.is_cli()? {
            return Ok(());
        }

        if let Some(m) = self.infer_crate_meta()? {
            // Clap YAML
            let meta = m.borrow();
            let clap_yaml = root.with_file_name("app.yaml");
            let base_content = template::render(
                TEMPLATE::CLAP_YAML,
                &ClapYamlContext {
                    name: meta.name.as_ref().unwrap(),
                    version: &format!("{:?}", meta.version.as_ref().unwrap()),
                },
            )?;

            let cli_mod = root.with_file_name("cli.rs");
            self.write_contents(&base_content, &clap_yaml)?;
            self.append_contents(&*self.cli_yaml.borrow(), &clap_yaml)?;

            // CLI module
            self.write_contents(
                "
use clap::ArgMatches;
use crate::client::{ApiClient, ApiError, Sendable};

pub(super) fn response_future(client: &dyn ApiClient, _matches: &ArgMatches<'_>,
                              sub_cmd: &str, sub_matches: Option<&ArgMatches<'_>>)
                             -> Result<Box<dyn futures::Future<Item=reqwest::r#async::Response, Error=ApiError> + Send + 'static>, crate::ClientError>
{
    match sub_cmd {",
                &cli_mod,
            )?;

            let cli_content = &mut *self.cli_match_arms.borrow_mut();
            cli_content.push_str(
                "
        _ => unimplemented!(),
    }
}
",
            );
            self.append_contents(&cli_content, &cli_mod)?;
        }

        // `main.rs`
        let contents = template::render(TEMPLATE::CLI_MAIN, &EmptyContext {})?;
        self.append_contents(&contents, &root)
    }

    /// Returns if this session is for generating CLI.
    fn is_cli(&self) -> Result<bool, Error> {
        Ok(self
            .infer_crate_meta()?
            .map(|m| m.borrow().is_cli)
            .unwrap_or(false))
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
        let is_cli = self.is_cli()?;
        man_path.set_file_name("Cargo.toml");

        let m = match self.infer_crate_meta()? {
            Some(c) => c,
            None => return Ok(()),
        };

        let meta = m.borrow();
        if self.is_crate() {
            let contents = template::render(
                TEMPLATE::CARGO_MANIFEST,
                &ManifestContext {
                    name: &format!("{:?}", meta.name.as_ref().unwrap()),
                    version: &format!("{:?}", meta.version.as_ref().unwrap()),
                    authors: &format!("{:?}", meta.authors.as_ref().unwrap()),
                    is_cli,
                },
            )?;

            self.write_contents(&contents, &man_path)?;
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

impl Clone for EmitterState {
    fn clone(&self) -> EmitterState {
        EmitterState {
            working_dir: self.working_dir.clone(),
            mod_prefix: self.mod_prefix,
            ns_sep: self.ns_sep,
            #[cfg(feature = "cli")]
            crate_meta: self.crate_meta.clone(),
            base_url: self.base_url.clone(),
            default_encoding: self.default_encoding.clone(),
            errors: self.errors.clone(),
            ..Default::default()
        }
    }
}

impl Default for EmitterState {
    fn default() -> EmitterState {
        EmitterState {
            working_dir: PathBuf::from("."),
            mod_prefix: "crate::",
            ns_sep: ".",
            #[cfg(feature = "cli")]
            crate_meta: None,
            base_url: RefCell::new("https://example.com".parse().expect("invalid URL?")),
            def_mods: RefCell::new(HashMap::new()),
            rel_paths: RefCell::new(HashSet::new()),
            mod_children: RefCell::new(HashMap::new()),
            unit_types: RefCell::new(HashSet::new()),
            cli_yaml: RefCell::new(String::new()),
            cli_match_arms: RefCell::new(String::new()),
            media_combinations: RefCell::new(BTreeSet::new()),
            errors: Rc::new(RefCell::new(vec![])),
            default_encoding: RefCell::new(SpecFormat::Json),
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

/* Templating contexts */

#[cfg(feature = "cli")]
#[derive(serde::Serialize)]
struct ManifestContext<'a> {
    name: &'a str,
    version: &'a str,
    authors: &'a str,
    is_cli: bool,
}

#[derive(serde::Serialize)]
struct ClientModContext<'a> {
    base_url: &'a str,
    default_range_index: usize,
    media_combinations: &'a BTreeSet<BTreeSet<String>>,
    errors: &'a [CodingError],
    coder: &'a Coder,
}

#[derive(Debug, serde::Serialize)]
struct CodingError {
    media_type: String,
    variant: String,
    ty_path: String,
}

#[derive(serde::Serialize)]
struct ClapYamlContext<'a> {
    name: &'a str,
    version: &'a str,
}

#[derive(serde::Serialize)]
struct EmptyContext {}
