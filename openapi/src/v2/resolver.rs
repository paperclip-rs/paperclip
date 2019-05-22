use super::im::ArcRwLock;
use super::Schema;
use crate::error::PaperClipError;
use failure::Error;

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};

// FIXME: The resolver is not in its best. It "just" works atm.

const DEF_REF_PREFIX: &str = "#/definitions/";

pub(crate) struct Resolver<S> {
    cur_def: RefCell<Option<String>>,
    cur_def_cyclic: Cell<bool>,
    cyclic_defs: HashSet<String>,
    pub defs: BTreeMap<String, ArcRwLock<S>>,
}

impl<S> From<BTreeMap<String, ArcRwLock<S>>> for Resolver<S> {
    fn from(defs: BTreeMap<String, ArcRwLock<S>>) -> Self {
        Resolver {
            cur_def: RefCell::new(None),
            cur_def_cyclic: Cell::new(false),
            cyclic_defs: HashSet::new(),
            defs,
        }
    }
}

impl<S> Resolver<S>
where
    S: Schema,
{
    pub fn resolve(&mut self) -> Result<(), Error> {
        // FIXME: We don't support definitions that refer another definition
        // directly from the root. Should we?
        for (name, schema) in &self.defs {
            trace!("Entering: {}", name);
            {
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

        self.defs.iter().for_each(|(name, schema)| {
            if self.cyclic_defs.contains(name) {
                schema.write().set_cyclic(true);
            }
        });

        Ok(())
    }

    fn resolve_definitions_no_root_ref(&self, schema: &ArcRwLock<S>) -> Result<(), Error> {
        let mut schema = schema.write();
        if let Some(mut inner) = schema.items_mut().take() {
            self.resolve_definitions(&mut inner)?;
        }

        if let Some(props) = schema.properties_mut().take() {
            props
                .values_mut()
                .try_for_each(|s| self.resolve_definitions(s))?;
        }

        Ok(())
    }

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

    fn resolve_definition_reference(&self, name: &str) -> Result<ArcRwLock<S>, Error> {
        if !name.starts_with(DEF_REF_PREFIX) {
            // FIXME: Bad
            return Err(PaperClipError::InvalidURI(name.into()))?;
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
