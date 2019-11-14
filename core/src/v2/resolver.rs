use super::{
    models::{Either, HttpMethod, Resolvable, ResolvableParameter, ResolvablePathItem},
    Schema,
};
use crate::error::ValidationError;
use heck::CamelCase;

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};
use std::mem;

// FIXME: The resolver is not in its best. It "just" works atm.

const DEF_REF_PREFIX: &str = "#/definitions/";

type DefinitionsMap<S> = BTreeMap<String, Resolvable<S>>;
type OperationsMap<S> = BTreeMap<String, ResolvablePathItem<S>>;

/// API schema resolver. This visits each definition and resolves
/// `$ref` field (if any) by finding the associated definition and
/// replacing the field with a reference to the actual definition.
// FIXME: Move all validation to resolver.
pub(crate) struct Resolver<S> {
    /// Current definition being resolved.
    cur_def: RefCell<Option<String>>,
    /// Whether the current definition is cyclic.
    cur_def_cyclic: Cell<bool>,
    /// Set containing cyclic definition names.
    cyclic_defs: HashSet<String>,
    /// Actual definitions.
    pub defs: DefinitionsMap<S>,
    /// Paths and the corresponding operations.
    pub paths: OperationsMap<S>,
}

impl<S> From<(DefinitionsMap<S>, OperationsMap<S>)> for Resolver<S> {
    fn from((defs, paths): (DefinitionsMap<S>, OperationsMap<S>)) -> Self {
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
    S: Schema + Default,
{
    /// Visit definitions and resolve them!
    pub fn resolve(&mut self) -> Result<(), ValidationError> {
        // Resolve path operations first. We may encounter anonymous
        // definitions along the way, which we'll insert into `self.defs`
        // and we'll have to resolve them anyway.
        let mut paths = mem::replace(&mut self.paths, BTreeMap::new());
        paths.iter_mut().try_for_each(|(path, map)| {
            trace!("Checking path: {}", path);

            self.resolve_operations(path, map)
        })?;
        self.paths = paths;

        // FIXME: We don't support definitions that refer another definition
        // directly from the root (i.e., alias). Should we?
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
    fn resolve_definitions_no_root_ref(
        &self,
        schema: &Resolvable<S>,
    ) -> Result<(), ValidationError> {
        let mut schema = schema.write();
        if let Some(inner) = schema.items_mut().take() {
            match inner {
                Either::Left(inner) => return self.resolve_definitions(inner),
                Either::Right(v) => {
                    return v.iter_mut().try_for_each(|s| self.resolve_definitions(s))
                }
            }
        }

        if let Some(props) = schema.properties_mut().take() {
            props
                .values_mut()
                .try_for_each(|s| self.resolve_definitions(s))?;
        }

        if let Some(props) = schema
            .additional_properties_mut()
            .take()
            .and_then(|s| s.right_mut())
        {
            self.resolve_definitions(props)?;
        }

        Ok(())
    }

    /// Resolve the given definition. If it contains a reference, find and assign it,
    /// otherwise traverse further.
    fn resolve_definitions(&self, schema: &mut Resolvable<S>) -> Result<(), ValidationError> {
        let ref_def = {
            if let Some(ref_name) = schema.read().reference() {
                trace!("Resolving {}", ref_name);
                Some(self.resolve_definition_reference(ref_name)?)
            } else {
                None
            }
        };

        if let Some(new) = ref_def {
            *schema = match schema {
                Resolvable::Raw(old) => Resolvable::Resolved {
                    old: old.clone(),
                    new: (&*new).clone(),
                },
                _ => unimplemented!("schema already resolved?"),
            };
        } else {
            self.resolve_definitions_no_root_ref(&*schema)?;
        }

        Ok(())
    }

    /// Resolve a given operation.
    fn resolve_operations(
        &mut self,
        path: &str,
        map: &mut ResolvablePathItem<S>,
    ) -> Result<(), ValidationError> {
        for (&method, op) in &mut map.methods {
            self.resolve_parameters(Some(method), path, &mut op.parameters)?;
            for response in op.responses.values_mut() {
                self.resolve_operation_schema(
                    &mut response.schema,
                    Some(method),
                    path,
                    "Response",
                )?;
            }
        }

        self.resolve_parameters(None, path, &mut map.parameters)
    }

    /// Resolve the given bunch of parameters.
    fn resolve_parameters(
        &mut self,
        method: Option<HttpMethod>,
        path: &str,
        params: &mut Vec<ResolvableParameter<S>>,
    ) -> Result<(), ValidationError> {
        for p in params.iter_mut() {
            let mut param = p.write();
            self.resolve_operation_schema(&mut param.schema, method, path, "Body")?;
        }

        Ok(())
    }

    /// Resolves request/response schema in operation.
    fn resolve_operation_schema(
        &mut self,
        s: &mut Option<Resolvable<S>>,
        method: Option<HttpMethod>,
        path: &str,
        suffix: &str,
    ) -> Result<(), ValidationError> {
        let schema = match s.as_mut() {
            Some(s) => s,
            _ => return Ok(()),
        };

        if schema.read().reference().is_none() {
            // We've encountered an anonymous schema definition in some
            // parameter/response. Give it a name and add it to global definitions.
            let prefix = method.map(|s| s.to_string()).unwrap_or_default();
            let def_name = (prefix + path + suffix).to_camel_case();
            let mut ref_schema = S::default();
            ref_schema.set_reference(format!("{}{}", DEF_REF_PREFIX, def_name));
            let old_schema = mem::replace(schema, ref_schema.into());
            self.defs.insert(def_name, old_schema);
        }

        self.resolve_definitions(schema)?;
        Ok(())
    }

    /// Given a name (from `$ref` field), get a reference to the definition.
    fn resolve_definition_reference(&self, name: &str) -> Result<Resolvable<S>, ValidationError> {
        if !name.starts_with(DEF_REF_PREFIX) {
            // FIXME: Bad
            return Err(ValidationError::InvalidRefURI(name.into()));
        }

        let name = &name[DEF_REF_PREFIX.len()..];
        match self.cur_def.borrow().as_ref() {
            Some(n) if n == name => self.cur_def_cyclic.set(true),
            _ => (),
        }

        let schema = self
            .defs
            .get(name)
            .ok_or_else(|| ValidationError::MissingDefinition(name.into()))?;
        Ok(schema.clone())
    }
}
