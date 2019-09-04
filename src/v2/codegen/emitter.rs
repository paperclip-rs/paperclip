use super::object::{ApiObject, ObjectField, OpRequirement, Parameter};
use super::state::{ChildModule, EmitterState};
use crate::error::PaperClipError;
use crate::v2::{
    im::ArcRwLock,
    models::{
        self, Api, Coder, DataType, DataTypeFormat, HttpMethod, Operation, OperationMap,
        ParameterIn, SchemaRepr,
    },
    Schema,
};
use failure::Error;
use heck::{CamelCase, SnekCase};
use itertools::Itertools;
use url::Host;

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Some "thing" emitted by the emitter.
pub enum EmittedUnit {
    /// Object represented as a Rust struct.
    Object(ApiObject),
    /// Some Rust type.
    Known(String),
    /// Nothing to do.
    None,
}

impl EmittedUnit {
    #[inline]
    fn known_type(self) -> String {
        match self {
            EmittedUnit::Known(s) => s,
            _ => panic!("Emitted unit is not a known type"),
        }
    }
}

/// `Emitter` represents the interface for generating the relevant
/// modules, API object definitions and the associated calls.
pub trait Emitter: Sized {
    /// The associated `Schema` implementor.
    type Definition: Schema + Debug;

    /// Returns a reference to the underlying state.
    fn state(&self) -> &EmitterState;

    /// Entrypoint for emitter. Given an API spec, generate code
    /// inside Rust modules in the configured working directory.
    fn generate(&self, api: &Api<Self::Definition>) -> Result<(), Error> {
        let state = self.state();
        state.reset_internal_fields();

        if let Some(h) = api.host.as_ref() {
            let mut parts = h.split(':');
            let mut u = state.base_url.borrow_mut();
            if let Some(host) = parts.next() {
                Host::parse(host).map_err(|e| PaperClipError::InvalidHost(h.into(), e))?;
                u.set_host(Some(&host))
                    .expect("expected valid host in URL?");
            }

            if let Some(port) = parts.next() {
                let p = port.parse::<u16>().map_err(|_| {
                    PaperClipError::InvalidHost(h.into(), url::ParseError::InvalidPort)
                })?;
                u.set_port(Some(p)).expect("expected valid port in URL?");
            }
        }

        if let Some(p) = api.base_path.as_ref() {
            state.base_url.borrow_mut().set_path(p);
        }

        let gen = CodegenEmitter(self);
        // Generate file contents by accumulating definitions.
        for (name, schema) in &api.definitions {
            debug!("Creating definition {}", name);
            let schema = schema.read();
            gen.generate_def_from_root(&schema)?;
        }

        state.declare_modules()?;
        state.write_definitions()?;

        for (path, map) in &api.paths {
            RequirementCollector {
                path,
                emitter: self,
                api,
                map,
                template_params: HashSet::new(),
            }
            .collect()?;
        }

        state.add_builders()?;
        state.add_client_deps()?;
        state.add_deps()?;

        Ok(())
    }

    /// Returns an iterator of path components for the given definition.
    ///
    /// **NOTE:** All components are [snake_cased](https://docs.rs/heck/*/heck/trait.SnekCase.html)
    /// (including the definition name).
    fn def_ns_name<'a>(
        &self,
        def: &'a Self::Definition,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>, Error> {
        let state = self.state();
        def.name()
            .map(|n| n.split(state.ns_sep).map(SnekCase::to_snek_case))
            .ok_or_else(|| {
                trace!("Invalid name for definition: {:?}", def);
                PaperClipError::InvalidDefinitionName.into()
            })
            .map(|i| Box::new(i) as Box<_>)
    }

    /// Returns the [CamelCase](https://docs.rs/heck/*/heck/trait.CamelCase.html)
    /// name for the given definition.
    fn def_name(&self, def: &Self::Definition) -> Result<String, Error> {
        Ok(self
            .def_ns_name(def)?
            .last()
            .map(|s| s.to_camel_case())
            .expect("last item always exists for split?"))
    }

    /// Returns the module path (from working directory) for the given definition.
    ///
    /// **NOTE:** This doesn't (shouldn't) set any extension to the leaf component.
    fn def_mod_path(&self, def: &Self::Definition) -> Result<PathBuf, Error> {
        let state = self.state();
        let mut path = state.working_dir.clone();
        path.extend(self.def_ns_name(def)?);
        path.set_extension("rs");
        Ok(path)
    }

    /// Builds a given definition. Also takes a `bool` to specify whether we're
    /// planning to define a Rust type or whether we're reusing an existing type.
    ///
    /// **NOTE:** We resolve type aliases to known types.
    fn build_def(&self, def: &Self::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if let Some(ty) = matching_unit_type(def.format(), def.data_type()) {
            trace!("Matches unit type: {}", ty);
            if define {
                return Ok(EmittedUnit::None);
            }

            return Ok(EmittedUnit::Known(ty.to_owned()));
        }

        match def.data_type() {
            Some(DataType::Array) => CodegenEmitter(self).emit_array(def, define),
            Some(DataType::Object) => CodegenEmitter(self).emit_object(def, define),
            Some(_) => unreachable!("bleh?"), // we've already handled everything else
            None => {
                if define {
                    Ok(EmittedUnit::None)
                } else {
                    Ok(EmittedUnit::Known("String".into()))
                }
            }
        }
    }
}

struct CodegenEmitter<'a, E>(&'a E)
where
    Self: Sized;

impl<'a, E> Deref for CodegenEmitter<'a, E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.0
    }
}

impl<'a, E> CodegenEmitter<'a, E>
where
    E: Emitter,
    E::Definition: Debug,
{
    /// Given a schema definition, generate the corresponding Rust definition.
    ///
    /// **NOTE:** This doesn't generate any files. It only adds the generated stuff
    /// to `EmitterState`.
    fn generate_def_from_root(&self, def: &E::Definition) -> Result<(), Error> {
        let state = self.state();
        // Generate the object.
        let mut object = match self.build_def(def, true)? {
            EmittedUnit::Object(o) => o,
            // We don't care about type aliases because we resolve them anyway.
            _ => return Ok(()),
        };

        let mod_path = self.def_mod_path(def)?;
        // Create parent dirs recursively for the leaf module.
        let dir_path = mod_path
            .parent()
            .ok_or_else(|| PaperClipError::InvalidDefinitionPath(mod_path.clone()))?;
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)?;
        }

        // Get the path without the extension.
        let full_path = dir_path.join(
            mod_path
                .file_stem()
                .ok_or_else(|| PaperClipError::InvalidDefinitionPath(mod_path.clone()))?,
        );
        // Get the relative path to the parent.
        let rel_path = full_path
            .strip_prefix(&state.working_dir)
            .map_err(|_| PaperClipError::InvalidDefinitionPath(full_path.clone()))?;

        // Gather the immediate parent-children pairs for module declarations.
        let mut mods = state.mod_children.borrow_mut();
        for (i, path) in rel_path.ancestors().enumerate() {
            if let (Some(parent), Some(name)) = (path.parent(), path.file_name()) {
                let entry = mods.entry(parent.into()).or_insert_with(HashSet::new);
                entry.insert(ChildModule {
                    name: name.to_string_lossy().into_owned(),
                    is_final: i == 0,
                });
            }
        }

        // Set the path for future reference
        object.path = rel_path.to_string_lossy().into_owned().replace('/', "::");

        // Add generated object to state.
        let mut def_mods = state.def_mods.borrow_mut();
        def_mods.insert(mod_path, object);

        Ok(())
    }

    /// Assumes that the given definition is an array and returns the corresponding
    /// vector type for it.
    fn emit_array(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        let it = def
            .items()
            .ok_or_else(|| PaperClipError::MissingArrayItem(self.def_name(def).ok()))?;

        let schema = it.read();
        let ty = self.build_def(&schema, false)?.known_type();
        Ok(EmittedUnit::Known(String::from("Vec<") + &ty + ">"))
    }

    /// Assumes that the given definition is an object and returns the corresponding
    /// Rust struct / map.
    fn emit_object(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        match self.try_emit_map(def, define)? {
            EmittedUnit::None => (),
            x => return Ok(x),
        }

        if !define {
            // Use absolute paths to save some pain.
            let mut ty_path = String::from(self.state().mod_prefix.trim_matches(':'));
            let mut iter = self.def_ns_name(def)?.peekable();
            while let Some(mut c) = iter.next() {
                ty_path.push_str("::");
                if iter.peek().is_none() {
                    ty_path.push_str(&c);
                    ty_path.push_str("::");
                    c = c.to_camel_case();
                }

                ty_path.push_str(&c);
            }

            return Ok(EmittedUnit::Known(ty_path));
        }

        self.emit_struct(def)
    }

    /// Checks if the given definition is a simple map and returns the corresponding `BTreeMap`.
    fn try_emit_map(&self, def: &E::Definition, define: bool) -> Result<EmittedUnit, Error> {
        if define {
            return Ok(EmittedUnit::None);
        }

        if let Some(s) = def.additional_properties() {
            let schema = s.read();
            let ty = self.build_def(&schema, false)?.known_type();
            let map = format!("std::collections::BTreeMap<String, {}>", ty);
            Ok(EmittedUnit::Known(map))
        } else {
            Ok(EmittedUnit::None)
        }
    }

    /// Helper for `emit_object` - This returns the Rust struct definition for the
    /// given schema definition.
    fn emit_struct(&self, def: &E::Definition) -> Result<EmittedUnit, Error> {
        let mut obj = ApiObject::with_name(self.def_name(def)?);
        obj.description = def.description().map(String::from);

        if let Some(props) = def.properties() {
            props
                .iter()
                .try_for_each(|(name, prop)| -> Result<(), Error> {
                    let schema = prop.read();
                    let ty = self.build_def(&schema, false)?;

                    obj.fields.push(ObjectField {
                        name: name.clone(),
                        description: prop.get_description(),
                        ty_path: ty.known_type(),
                        is_required: def
                            .required_properties()
                            .map(|s| s.contains(name))
                            .unwrap_or(false),
                        boxed: schema.is_cyclic(),
                        children_req: self.children_requirements(&schema),
                    });

                    Ok(())
                })?
        }

        Ok(EmittedUnit::Object(obj))
    }

    /// Returns the requirements of the "deepest" child type in the given definition.
    ///
    /// See `ObjectField.children_req` field for what it means.
    fn children_requirements(&self, schema: &E::Definition) -> Vec<String> {
        match schema.data_type() {
            Some(DataType::Object) => {
                if let Some(s) = schema.additional_properties() {
                    return self.children_requirements(&s.read());
                } else if let Some(s) = schema.required_properties() {
                    return s.iter().cloned().collect();
                }
            }
            Some(DataType::Array) => {
                if let Some(s) = schema.items() {
                    return self.children_requirements(&s.read());
                }
            }
            _ => (),
        }

        vec![]
    }
}

/// Abstraction which takes care of adding requirements for operations.
struct RequirementCollector<'a, E: Emitter> {
    path: &'a str,
    emitter: &'a E,
    api: &'a Api<E::Definition>,
    map: &'a OperationMap<SchemaRepr<E::Definition>>,
    template_params: HashSet<String>,
}

impl<'a, E> RequirementCollector<'a, E>
where
    E: Emitter,
    E::Definition: Debug,
{
    /// Given a path and an operation map, collect the stuff required
    /// for generating builders later.
    fn collect(mut self) -> Result<(), Error> {
        self.validate_path_and_add_params()?;
        debug!("Collecting builder requirement for {:?}", self.path);

        // Collect all the parameters local to some API call.
        let (unused_params, _) = self.collect_parameters(&self.map.parameters)?;
        // FIXME: What if a body is "required" globally (for all operations)?
        // This means, operations can override the body with some other schema
        // and we may need to map it to the appropriate builders.

        for (&meth, op) in &self.map.methods {
            self.collect_from_operation(meth, op, &unused_params)?;
        }

        // FIXME: If none of the parameters (local to operation or global) specify
        // a body then we should use something (say, `operationID`) to generate
        // a builder and forward `unused_params` to it?
        if self.map.methods.is_empty() {
            warn!(
                "Missing operations for path: {:?}{}",
                self.path,
                if unused_params.is_empty() {
                    ""
                } else {
                    ", but 'parameters' field is specified."
                }
            );
        }

        if !self.template_params.is_empty() {
            return Err(PaperClipError::MissingParametersInPath(
                self.path.into(),
                self.template_params,
            )
            .into());
        }

        Ok(())
    }

    /// Checks whether this path is unique (regardless of its templating)
    /// and returns the list of parameters that exist in the template.
    ///
    /// For example, `/api/{foo}` and `/api/{bar}` are the same, and we
    /// should reject it.
    fn validate_path_and_add_params(&mut self) -> Result<(), PaperClipError> {
        let path_fmt = Api::<()>::path_parameters_map(self.path, |p| {
            self.template_params.insert(p.into());
            ":".into()
        });

        let state = self.emitter.state();
        let mut paths = state.rel_paths.borrow_mut();
        let value_absent = paths.insert(path_fmt.clone().into());
        if value_absent {
            Ok(())
        } else {
            Err(PaperClipError::RelativePathNotUnique(self.path.into()))
        }
    }

    /// Collect the parameters local to an API call operation (method).
    fn collect_from_operation(
        &mut self,
        meth: HttpMethod,
        op: &Operation<SchemaRepr<E::Definition>>,
        unused_params: &[Parameter],
    ) -> Result<(), Error> {
        let (mut params, schema_path) = self.collect_parameters(&op.parameters)?;
        // If we have unused params which don't exist in the method-specific
        // params (which take higher precedence), then we can copy those inside.
        for global_param in unused_params {
            if params
                .iter()
                .find(|p| p.name == global_param.name)
                .is_none()
            {
                params.push(global_param.clone());
            }
        }

        // If there's a matching object, add the params to its operation.
        if let Some(pat) = schema_path.as_ref() {
            self.bind_schema_to_operation(pat, meth, op, params)?;
        } else {
            self.bind_operation_blindly(meth, op, params);
        }

        Ok(())
    }

    /// Given a bunch of resolved parameters, validate and collect a simplified version of them.
    fn collect_parameters(
        &mut self,
        obj_params: &[models::Parameter<SchemaRepr<E::Definition>>],
    ) -> Result<(Vec<Parameter>, Option<PathBuf>), Error> {
        let def_mods = self.emitter.state().def_mods.borrow();
        let mut schema_path = None;
        let mut params = vec![];
        for p in obj_params {
            p.check(self.path)?; // validate the parameter

            if let Some(def) = p.schema.as_ref() {
                // If a schema exists, then get its path for later use.
                let pat = self.emitter.def_mod_path(&*def.read())?;
                def_mods.get(&pat).ok_or_else(|| {
                    PaperClipError::UnsupportedParameterDefinition(p.name.clone(), self.path.into())
                })?;
                schema_path = Some(pat);
                continue;
            }

            // If this is a parameter that must exist in path, then remove it
            // from the expected list of parameters.
            if p.in_ == ParameterIn::Path {
                self.template_params.remove(&p.name);
            }

            // Enforce that the parameter is a known type and collect it.
            let ty = matching_unit_type(p.format.as_ref(), p.data_type).ok_or_else(|| {
                PaperClipError::UnknownParameterType(p.name.clone(), self.path.into())
            })?;
            params.push(Parameter {
                name: p.name.clone(),
                description: p.description.clone(),
                ty_path: ty.into(),
                presence: p.in_,
                // NOTE: parameter is required if it's in path
                required: p.required || p.in_ == ParameterIn::Path,
            });
        }

        Ok((params, schema_path))
    }

    /// Given a schema path, fetch the object and bind the given operation to it.
    fn bind_schema_to_operation(
        &self,
        schema_path: &Path,
        meth: HttpMethod,
        op: &Operation<SchemaRepr<E::Definition>>,
        params: Vec<Parameter>,
    ) -> Result<(), Error> {
        let state = self.emitter.state();
        let mut def_mods = state.def_mods.borrow_mut();
        let obj = def_mods.get_mut(schema_path).expect("bleh?");
        let ops = obj
            .paths
            .entry(self.path.into())
            .or_insert_with(Default::default);

        let (encoder, decoders) = self.get_coders(op);
        ops.req.insert(
            meth,
            OpRequirement {
                listable: false,
                id: op.operation_id.clone(),
                description: op.description.clone(),
                params,
                body_required: true,
                response_ty_path: if let Some(s) = Self::get_2xx_response_schema(&op) {
                    let schema = &*s.read();
                    Some(self.emitter.build_def(schema, false)?.known_type())
                } else {
                    None
                },
                encoder,
                decoders,
            },
        );

        Ok(())
    }

    /// We couldn't attach this operation to any object. Now, we're
    /// just attempting out of desperation.
    fn bind_operation_blindly(
        &self,
        meth: HttpMethod,
        op: &Operation<SchemaRepr<E::Definition>>,
        params: Vec<Parameter>,
    ) {
        // Let's try from the response maybe...
        let s = match Self::get_2xx_response_schema(&op) {
            Some(s) => s,
            None => return,
        };

        let schema = &*s.read();
        let state = self.emitter.state();
        let mut def_mods = state.def_mods.borrow_mut();
        let listable = schema.items().and_then(|s| s.read().data_type()) == Some(DataType::Object);
        let s = match schema.data_type() {
            // We can deal with object responses.
            Some(DataType::Object) => s.clone(),
            // We can also deal with array of objects by mapping
            // the operation to that object.
            _ if listable => (&**schema.items().unwrap()).clone(),
            // FIXME: Handle other types where we can't map an
            // operation to a known schema.
            _ => return,
        };

        let schema = &*s.read();
        let pat = self.emitter.def_mod_path(schema).ok();
        let obj = match pat.and_then(|p| def_mods.get_mut(&p)) {
            Some(o) => o,
            None => {
                warn!(
                    "Skipping unknown response schema for path {:?}: {:?}",
                    self.path, schema
                );
                return;
            }
        };

        let ops = obj
            .paths
            .entry(self.path.into())
            .or_insert_with(Default::default);

        let (encoder, decoders) = self.get_coders(op);
        ops.req.insert(
            meth,
            OpRequirement {
                id: op.operation_id.clone(),
                description: op.description.clone(),
                params,
                body_required: false,
                listable,
                response_ty_path: None,
                encoder,
                decoders,
            },
        );
    }

    /// Returns the first 2xx response schema in this operation.
    ///
    /// **NOTE:** This assumes that 2xx response schemas are the same for an operation.
    fn get_2xx_response_schema<'o>(
        op: &'o Operation<SchemaRepr<E::Definition>>,
    ) -> Option<&'o ArcRwLock<E::Definition>> {
        op.responses
            .iter()
            .filter(|(c, _)| c.starts_with('2')) // 2xx response
            .filter_map(|(_, r)| r.schema.as_ref())
            .next()
            .map(|r| &**r)
    }

    /// Returns the coders for the given operation. The former is the preferred encoder,
    /// and the latter is the list of decoders. It's a list because even though we prefer
    /// to use one encoder, we decode based on the `Content-Type` header returned by the
    /// server, so we must be prepared to face anything. Also, there's always an en/decoder
    /// because even if we don't have any matching coder for some media type, we have
    /// JSON/YAML (format used for described the spec) to fallback to.
    fn get_coders(
        &self,
        op: &Operation<SchemaRepr<E::Definition>>,
    ) -> (Arc<Coder>, Vec<Arc<Coder>>) {
        let consumes = match op.consumes.as_ref() {
            Some(s) => s,
            None => &self.api.consumes,
        };

        let mut encoders = consumes
            .iter()
            .filter_map(|r| self.api.coders.matching_coder(r))
            .sorted_by(|a, b| b.prefer.cmp(&a.prefer)); // sort based on preference.

        let produces = match op.produces.as_ref() {
            Some(s) => s,
            None => &self.api.produces,
        };

        let mut decoders = produces
            .iter()
            .filter_map(|r| self.api.coders.matching_coder(r))
            .sorted_by(|a, b| b.prefer.cmp(&a.prefer))
            .collect::<Vec<_>>();

        if decoders.is_empty() {
            decoders.push(self.api.spec_format.coder());
        }

        (
            encoders
                .next()
                .unwrap_or_else(|| self.api.spec_format.coder()),
            decoders,
        )
    }
}

/// Checks if the given type/format matches a known Rust type and returns it.
fn matching_unit_type(
    format: Option<&DataTypeFormat>,
    type_: Option<DataType>,
) -> Option<&'static str> {
    match format {
        Some(DataTypeFormat::Int32) => Some("i32"),
        Some(DataTypeFormat::Int64) => Some("i64"),
        Some(DataTypeFormat::Float) => Some("f32"),
        Some(DataTypeFormat::Double) => Some("f64"),
        _ => match type_ {
            Some(DataType::Integer) => Some("i64"),
            Some(DataType::Number) => Some("f64"),
            Some(DataType::Boolean) => Some("bool"),
            Some(DataType::String) => Some("String"),
            _ => None,
        },
    }
}
