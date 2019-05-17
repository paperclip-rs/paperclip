use super::models::{Reference, Schema};
use crate::error::PaperClipError;
use failure::Error;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::mem;
use std::rc::Rc;

// FIXME: The resolver is not in its best. It "just" works atm.

const DEF_REF_PREFIX: &str = "#/definitions/";

pub(crate) struct Resolver {
    defs: BTreeMap<String, Rc<RefCell<Schema>>>,
}

impl From<BTreeMap<String, Schema>> for Resolver {
    fn from(defs: BTreeMap<String, Schema>) -> Self {
        Resolver {
            defs: defs
                .into_iter()
                .map(|(k, v)| (k, Rc::new(RefCell::new(v))))
                .collect(),
        }
    }
}

impl Resolver {
    pub fn resolve(self) -> Result<BTreeMap<String, Rc<RefCell<Schema>>>, Error> {
        self.defs.values().try_for_each(|schema| {
            let mut schema = schema.borrow_mut();
            self.resolve_schema(&mut schema)
        })?;

        Ok(self.defs)
    }

    fn resolve_schema(&self, schema: &mut Schema) -> Result<(), Error> {
        if let Some(Reference::Identifier(ref ref_name)) = &mut schema.reference {
            let ref_schema = self.resolve_definition_reference(ref_name)?;
            *schema.reference.as_mut().unwrap() = Reference::Resolved(ref_schema);
        }

        schema.items = match schema.items.take() {
            Some(Reference::Identifier(ref ref_name)) => {
                let ref_schema = self.resolve_definition_reference(ref_name)?;
                Some(Reference::Resolved(ref_schema))
            }
            Some(Reference::Raw(mut schema)) => {
                self.resolve_schema(&mut schema)?;
                Some(Reference::Resolved(Rc::new(RefCell::new(*schema))))
            }
            value => value,
        };

        if let Some(props) = schema.properties.as_mut() {
            let old_props = mem::replace(props, BTreeMap::new());
            for (name, mut ref_schema) in old_props {
                ref_schema = match ref_schema {
                    Reference::Identifier(ref ref_name) => {
                        let new_schema = self.resolve_definition_reference(ref_name)?;
                        Reference::Resolved(new_schema)
                    }
                    Reference::Raw(mut schema) => {
                        self.resolve_schema(&mut schema)?;
                        Reference::Resolved(Rc::new(RefCell::new(*schema)))
                    }
                    value => value,
                };

                props.insert(name, ref_schema);
            }
        }

        Ok(())
    }

    fn resolve_definition_reference(&self, name: &str) -> Result<Rc<RefCell<Schema>>, Error> {
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
