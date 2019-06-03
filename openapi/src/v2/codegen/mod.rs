//! Code generation for OpenAPI v2.

mod emitter;
pub mod object;
mod state;

pub use self::emitter::{EmittedUnit, Emitter};
pub use self::state::EmitterState;

use super::Schema;

use std::fmt::Debug;
use std::marker::PhantomData;

/// Common conflicting keywords in Rust. An underscore will be added
/// to fields using these keywords.
// FIXME: Fill this list!
const RUST_KEYWORDS: &[&str] = &["type", "continue", "enum", "ref"];

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
