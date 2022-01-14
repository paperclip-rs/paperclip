//! Code generation for OpenAPI v2.

#[cfg(feature = "cli")]
mod author;
mod emitter;
mod impls;
pub mod object;
mod state;
include!(concat!(env!("OUT_DIR"), "/template.rs"));

pub use self::{
    emitter::{EmittedUnit, Emitter},
    state::EmitterState,
};

use super::Schema;

use std::{fmt::Debug, marker::PhantomData};

/// Common conflicting keywords in Rust. An underscore will be added
/// to fields using these keywords.
const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
    "use", "where", "while", "do",
];

/// Default emitter for anything that implements `Schema` trait.
///
/// This doesn't do anything special, as `Emitter` trait methods take
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

/// Metadata for generating a crate.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub struct CrateMeta {
    /// Name of the crate. If this is not specified, then the name of the
    /// working directory is assumed to be crate name.
    pub name: Option<String>,
    /// Version (defaults to 0.1.0)
    pub version: Option<String>,
    /// List of authors for this crate. Defaults to cargo's defaults.
    pub authors: Option<Vec<String>>,
    /// Whether we're planning to emit a lib, app or module.
    pub mode: EmitMode,
    /// Whether or not to make this a root crate.
    pub no_root: bool,
}

/// Mode of codegen (module, crate or CLI app).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EmitMode {
    Module,
    Crate,
    App,
}

impl Default for EmitMode {
    fn default() -> Self {
        // NOTE: Module by default - CLI will specify other modes if needed.
        EmitMode::Module
    }
}
