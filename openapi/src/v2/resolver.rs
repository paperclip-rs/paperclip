use super::im::ArcRwLock;
use super::Schema;
use crate::error::PaperClipError;
use failure::Error;

use std::collections::BTreeMap;

// FIXME: The resolver is not in its best. It "just" works atm.

const DEF_REF_PREFIX: &str = "#/definitions/";

pub(crate) struct Resolver<S> {
    pub defs: BTreeMap<String, ArcRwLock<S>>,
}

impl<S> Resolver<S>
where
    S: Schema,
{
    pub fn resolve(&mut self) -> Result<(), Error> {
        self.defs
            .iter()
            // FIXME: We don't support definitions that refer another definition
            // directly from the root.
            .try_for_each(|(name, schema)| {
                trace!("Entering: {}", name);
                {
                    let mut s = schema.write();
                    s.set_name(name);
                }

                self.resolve_definitions_no_root_ref(schema)
            })
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
        let schema = self
            .defs
            .get(name)
            .ok_or(PaperClipError::MissingDefinition(name.into()))?;
        Ok(schema.clone())
    }
}
