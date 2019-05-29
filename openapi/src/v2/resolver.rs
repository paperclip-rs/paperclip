use super::im::ArcRwLock;
use super::{models::OperationMap, Schema};
use crate::error::PaperClipError;
use failure::Error;

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};
use std::mem;

// FIXME: The resolver is not in its best. It "just" works atm.

const DEF_REF_PREFIX: &str = "#/definitions/";

/// API schema resolver. This visits each definition and resolves
/// `$ref` field (if any) by finding the associated definition and
/// replacing the field with a reference to the actual definition.
pub(crate) struct Resolver<S> {
    /// Current definition being resolved.
    cur_def: RefCell<Option<String>>,
    /// Whether the current definition is cyclic.
    cur_def_cyclic: Cell<bool>,
    /// Set containing cyclic definition names.
    cyclic_defs: HashSet<String>,
    /// Actual definitions.
    pub defs: BTreeMap<String, ArcRwLock<S>>,
    /// Paths and the corresponding operations.
    pub paths: BTreeMap<String, OperationMap<S>>,
}

impl<S>
    From<(
        BTreeMap<String, ArcRwLock<S>>,
        BTreeMap<String, OperationMap<S>>,
    )> for Resolver<S>
{
    fn from(
        (defs, paths): (
            BTreeMap<String, ArcRwLock<S>>,
            BTreeMap<String, OperationMap<S>>,
        ),
    ) -> Self {
        Resolver {
            cur_def: RefCell::new(None),
            cur_def_cyclic: Cell::new(false),
            cyclic_defs: HashSet::new(),
            defs,
            paths,
        }
    }
}

impl<S> Resolver<S>
where
    S: Schema,
{
    /// Visit definitions and resolve them!
    pub fn resolve(&mut self) -> Result<(), Error> {
        // FIXME: We don't support definitions that refer another definition
        // directly from the root. Should we?
        for (name, schema) in &self.defs {
            trace!("Entering: {}", name);
            {
                // Set the name and cyclic-ness of the current definition.
                let mut s = schema.write();
                s.set_name(name);
                *self.cur_def.borrow_mut() = Some(name.clone());
                self.cur_def_cyclic.set(false);
            }

            self.resolve_definitions_no_root_ref(schema)?;
            if self.cur_def_cyclic.get() {
                self.cyclic_defs.insert(name.clone());
            }
        }

        let mut paths = mem::replace(&mut self.paths, BTreeMap::new());
        paths.iter_mut().try_for_each(|(path, map)| {
            trace!("Checking path: {}", path);

            self.resolve_operations(map)
        })?;
        self.paths = paths;

        // We're doing this separately because we may have mutably borrowed
        // definitions if they're cyclic and borrowing them again will result
        // in a deadlock.
        self.defs.iter().for_each(|(name, schema)| {
            if self.cyclic_defs.contains(name) {
                schema.write().set_cyclic(true);
            }
        });

        Ok(())
    }

    /// We've passed some definition. Resolve it assuming that it doesn't
    /// contain any reference.
    // FIXME: This means we currently don't support definitions which
    // directly refer some other definition (basically a type alias). Should we?
    fn resolve_definitions_no_root_ref(&self, schema: &ArcRwLock<S>) -> Result<(), Error> {
        let mut schema = schema.write();
        if let Some(mut inner) = schema.items_mut().take() {
            return self.resolve_definitions(&mut inner);
        }

        if let Some(props) = schema.properties_mut().take() {
            props
                .values_mut()
                .try_for_each(|s| self.resolve_definitions(s))?;
        }

        Ok(())
    }

    /// Resolve the given definition. If it contains a reference, find and assign it,
    /// otherwise traverse further.
    fn resolve_definitions(&self, schema: &mut ArcRwLock<S>) -> Result<(), Error> {
        let ref_def = {
            if let Some(ref_name) = schema.read().reference() {
                trace!("Resolving {}", ref_name);
                Some(self.resolve_definition_reference(ref_name)?)
            } else {
                None
            }
        };

        if let Some(s) = ref_def {
            *schema = s;
        } else {
            self.resolve_definitions_no_root_ref(&*schema)?;
        }

        Ok(())
    }

    /// Resolve a given operation.
    fn resolve_operations(&self, map: &mut OperationMap<S>) -> Result<(), Error> {
        map.methods.iter_mut().try_for_each(|(_, op)| {
            op.responses.iter_mut().try_for_each(|(_, response)| {
                if let Some(schema) = response.schema.as_mut() {
                    self.resolve_definitions(schema)?;
                }

                Ok(())
            })
        })

        // FIXME: Resolve parameters
    }

    /// Given a name (from `$ref` field), get a reference to the definition.
    fn resolve_definition_reference(&self, name: &str) -> Result<ArcRwLock<S>, Error> {
        if !name.starts_with(DEF_REF_PREFIX) {
            // FIXME: Bad
            return Err(PaperClipError::InvalidRefURI(name.into()))?;
        }

        let name = &name[DEF_REF_PREFIX.len()..];
        match self.cur_def.borrow().as_ref() {
            Some(n) if n == name => self.cur_def_cyclic.set(true),
            _ => (),
        }

        let schema = self
            .defs
            .get(name)
            .ok_or(PaperClipError::MissingDefinition(name.into()))?;
        Ok(schema.clone())
    }
}
