use super::emitter::ANY_GENERIC_PARAMETER;
use super::object::{ApiObject, ApiObjectBuilder, Response, StructField, TypeParameters};
use super::RUST_KEYWORDS;
use crate::v2::models::{CollectionFormat, ParameterIn, JSON_CODER, JSON_MIME};
use heck::{CamelCase, KebabCase, SnekCase};

use std::fmt::{self, Display, Write};
use std::iter;
use std::rc::Rc;

/// Represents the API object impl.
pub struct ApiObjectImpl<'a> {
    inner: &'a ApiObject,
    // NOTE: `Rc<[T]>` because we shouldn't mutate the stuff later.
    pub(super) builders: Rc<[ApiObjectBuilder<'a>]>,
}

impl ApiObject {
    /// Returns a struct representing the impl for this object. This also
    /// holds the builders generated for this object.
    ///
    /// Each builder is bound to an operation in a path. If the object is not
    /// bound to any operation, then the builder only keeps track of the fields
    /// for building the actual object.
    // FIXME: Make operations generic across builders. This will reduce the
    // number of structs generated.
    pub fn impl_repr<'a>(&'a self, helper_module_prefix: &'a str) -> ApiObjectImpl<'a> {
        let needs_any = self.fields.iter().any(|f| f.needs_any);
        // Always emit a builder for API objects (regardless of operations).
        let main_builder = ApiObjectBuilder {
            helper_module_prefix,
            object: &self.name,
            body_required: true,
            fields: &self.fields,
            encoding: None,
            needs_any,
            ..Default::default()
        };

        let path_iter = self
            .paths
            .iter()
            .enumerate()
            .flat_map(move |(idx, (path, path_ops))| {
                path_ops
                    .req
                    .iter()
                    .map(move |(&method, req)| ApiObjectBuilder {
                        idx,
                        is_list_op: req.listable,
                        multiple_builders_exist: {
                            let mut iter =
                                self.paths.values().flat_map(|path_ops| path_ops.req.iter());
                            iter.next().is_some() && iter.next().is_some()
                        },
                        helper_module_prefix,
                        rel_path: Some(path),
                        description: req.description.as_ref().map(String::as_str),
                        object: &self.name,
                        op_id: req.id.as_ref().map(String::as_str),
                        deprecated: req.deprecated,
                        method: Some(method),
                        body_required: req.body_required,
                        encoding: req.encoding.as_ref(),
                        decoding: req.decoding.as_ref(),
                        fields: &self.fields,
                        global_params: &path_ops.params,
                        local_params: &req.params,
                        needs_any: needs_any && req.body_required,
                        response: Response {
                            ty_path: req.response.ty_path.as_ref().map(String::as_str),
                            contains_any: req.response.contains_any,
                        },
                    })
            });

        ApiObjectImpl {
            inner: self,
            builders: iter::once(if main_builder.fields.is_empty() {
                None
            } else {
                Some(main_builder)
            })
            .filter_map(|b| b)
            .chain(path_iter)
            .collect::<Vec<_>>()
            .into(),
        }
    }
}

impl<'a> ApiObjectImpl<'a> {
    /// Writes the required "clap" subcommand for this object in YAML.
    pub(super) fn write_clap_yaml<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        self.with_cli_cmd_and_builder(|name, builder| {
            write!(f, "\n  - {}:", name)?;
            if let Some(desc) = builder.description {
                write!(f, "\n      about: {:?}", desc)?;
            }

            let mut iter = builder
                .struct_fields_iter()
                .filter(|f| f.prop.is_parameter())
                .peekable();

            // Has at least one argument or body.
            if iter.peek().is_some() || builder.body_required {
                f.write_str("\n      args:")?;
                if builder.body_required {
                    write!(
                        f,
                        "
        - payload:
            long: payload
            help: \"Path to payload (schema: {obj}) or pass '-' for stdin\"
            takes_value: true
            required: true",
                        obj = self.inner.name
                    )?;
                }
            }

            iter.try_for_each(|field| {
                let field_name = field.name.to_kebab_case();
                write!(f, "\n        - {}:", &field_name)?;
                f.write_str("\n            long: ")?;
                f.write_str(&field_name)?;
                if field.prop.is_required() {
                    f.write_str("\n            required: true")?;
                }

                if let Some(desc) = field.desc {
                    write!(f, "\n            help: {:?}", desc)?;
                }

                f.write_str("\n            takes_value: true")
            })
        })?;

        f.write_str("\n")
    }

    /// Writes the match arms associated with this object's operations.
    pub(super) fn write_arg_match_arms<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        self.with_cli_cmd_and_builder(|name, builder| {
            f.write_str("\n        \"")?;
            f.write_str(&name)?;
            f.write_str("\" => {\n            let builder = ")?;
            f.write_str(&builder.helper_module_prefix)?;
            f.write_str(&self.inner.path)?;
            f.write_str("::")?;
            builder.write_name(f)?;
            f.write_str(
                "::from_args(sub_matches)?;
            builder.send_raw(client).await
        },",
            )
        })
    }

    /// Helper function for calling the given closure with the kebab-case
    /// name of the builder (operation) and the actual builder.
    fn with_cli_cmd_and_builder<F, E>(&self, mut call: F) -> Result<(), E>
    where
        F: FnMut(String, &ApiObjectBuilder<'_>) -> Result<(), E>,
    {
        // Ignore objects without any operations (all objects have a default builder).
        if self.builders.len() < 2 {
            return Ok(());
        }

        for builder in &self.builders[1..] {
            let name = match builder.op_id {
                Some(n) => n.to_kebab_case(),
                None => {
                    // FIXME: Investigate what we should do in the absence of operation ID.
                    warn!(
                        "Unable to generate name for operation ({:?} {:?}). Skipping.",
                        builder.method, builder.rel_path,
                    );

                    continue;
                }
            };

            call(name, builder)?;
        }

        Ok(())
    }

    /// Writes the associated function for this object for instantiating builders.
    fn write_builder_methods<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        for builder in &*self.builders {
            let mut temp = String::new();
            let has_fields = builder.has_atleast_one_field();
            if builder.description.is_none() {
                temp.write_str("\n")?;
            }

            if builder.deprecated {
                temp.write_str("    #[deprecated]\n")?;
            }

            // All builder constructor functions are inlined.
            temp.write_str("    #[inline]\n    pub fn ")?;
            if let Some(name) = builder.constructor_fn_name() {
                temp.write_str(&name)?;
                ApiObject::write_docs(builder.description.as_ref(), f, 1)?;
            } else {
                // If we can't generate a name of a builder, then we go for a
                // simple object builder.
                f.write_str("\n    /// Create a builder for this object.")?;
                temp.write_str("builder")?;
            }

            // Now that we've written the docs, we can write the actual method signature.
            f.write_str(&temp)?;
            f.write_str("() -> ")?;
            builder.write_name(f)?;
            builder.write_generics_if_necessary(f, None, TypeParameters::ReplaceAll)?;
            f.write_str(" {\n        ")?;
            builder.write_name(f)?;

            if has_fields || builder.body_required {
                f.write_str(" {")?;
            }

            let needs_container = builder.needs_container();
            if needs_container {
                f.write_str("\n            ")?;
                f.write_str("inner: Default::default(),")?;
            } else if builder.body_required {
                f.write_str("\n            ")?;
                f.write_str("body: Default::default(),")?;
            }

            builder.struct_fields_iter().try_for_each(|field| {
                if field.prop.is_required() {
                    f.write_str("\n            ")?;
                    if field.prop.is_parameter() {
                        f.write_str("_param")?;
                    }

                    f.write_str("_")?;
                    f.write_str(&field.name.to_snek_case())?;
                    f.write_str(": core::marker::PhantomData,")?;
                // If we have a container, then we store parameters inside that.
                } else if field.prop.is_parameter() && !needs_container {
                    f.write_str("\n            param_")?;
                    f.write_str(&field.name.to_snek_case())?;
                    f.write_str(": None,")?;
                }

                Ok(())
            })?;

            if has_fields || builder.body_required {
                f.write_str("\n        }")?;
            }

            f.write_str("\n    }\n")?;
        }

        Ok(())
    }

    /// Writes the `Into` impl for fulfilled builders (if they have a body).
    fn write_into_impl<F>(&self, builder: &ApiObjectBuilder<'_>, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        if !builder.body_required {
            return Ok(());
        }

        let needs_container = builder.needs_container();
        f.write_str("\nimpl")?;
        if builder.needs_any {
            ApiObject::write_any_generic(f)?;
        }

        f.write_str(" Into<")?;
        f.write_str(&self.inner.name)?;
        if builder.needs_any {
            ApiObject::write_any_generic(f)?;
        }

        f.write_str("> for ")?;
        builder.write_name(f)?;
        builder.write_generics_if_necessary(f, None, TypeParameters::ChangeAll)?;
        f.write_str(" {\n    fn into(self) -> ")?;
        f.write_str(&self.inner.name)?;
        if builder.needs_any {
            ApiObject::write_any_generic(f)?;
        }

        f.write_str(" {\n        self.")?;

        if needs_container {
            f.write_str("inner.")?;
        }

        f.write_str("body\n    }\n}\n")
    }
}

/// Represents the API object builder impl.
pub struct ApiObjectBuilderImpl<'a, 'b>(&'a ApiObjectBuilder<'b>);

impl<'a> ApiObjectBuilder<'a> {
    /// Returns a struct representing the impl for this builder.
    pub fn impl_repr(&self) -> ApiObjectBuilderImpl<'_, '_> {
        ApiObjectBuilderImpl(self)
    }
}

impl<'a, 'b> ApiObjectBuilderImpl<'a, 'b>
where
    'b: 'a,
{
    /// Writes impl for getting args from `clap::ArgMatches`
    pub(super) fn write_arg_parsing<F>(&self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        if self.0.rel_path.is_none() && self.0.method.is_none() {
            return Ok(());
        }

        let needs_container = self.0.needs_container();
        f.write_str("\n#[allow(unused_variables)]\nimpl ")?;
        self.0.write_name(f)?;
        self.0.write_generics_if_necessary(
            f,
            Some(
                self.0
                    .encoding
                    .map(|(_, c)| c.any_value.as_str())
                    .unwrap_or_else(|| JSON_CODER.any_value.as_str()),
            ),
            TypeParameters::ChangeAll,
        )?;

        // NOTE: We're assuming that we've correctly given all the arg requirements to clap.
        f.write_str(
            " {
    pub(crate) fn from_args(matches: Option<&clap::ArgMatches<'_>>) -> Result<Self, crate::ClientError> {",
        )?;
        f.write_str("\n        let thing = ")?;
        self.0.write_name(f)?;
        f.write_str(" {")?;

        if needs_container {
            f.write_str("\n            inner: ")?;
            self.0.write_container_name(f)?;
            f.write_str(" {")?;
        }

        if self.0.body_required {
            write!(
                f,
                "
            body: crate::cli::read_from_input(matches)?,"
            )?;
        }

        let mut phantom = String::new();
        self.0.struct_fields_iter().try_for_each(|field| {
            let (sk, kk) = (field.name.to_snek_case(), field.name.to_kebab_case());
            if field.prop.is_required() {
                phantom.push_str("\n            _");
                if field.prop.is_parameter() {
                    phantom.push_str("param_");
                }
                phantom.push_str(&sk);
                phantom.push_str(": core::marker::PhantomData,");
            }

            if field.prop.is_field() {
                return Ok(());
            }

            f.write_str("\n            param_")?;
            f.write_str(&sk)?;
            let mut ty = String::new();
            ApiObjectBuilder::write_wrapped_ty(
                self.0.helper_module_prefix,
                field.ty,
                field.delimiting,
                &mut ty,
            )?;

            // We're enforcing requirements in the CLI. We can relax here.
            writeln!(
                f,
                ": matches.and_then(|m| {{
                    m.value_of(\"{arg}\").map(|_| {{
                        value_t!(m, \"{arg}\", {ty}).unwrap_or_else(|e| e.exit())
                    }})
                }}),",
                arg = kk,
                ty = ty
            )
        })?;

        if needs_container {
            f.write_str("\n            },")?;
        }

        f.write_str(&phantom)?;
        f.write_str(
            "
        };

        Ok(thing)
    }
}
",
        )
    }

    /// Builds the method parameter type using the actual field type.
    ///
    /// For example, if a field is `Vec<T>`, then we replace it (in builder method)
    /// with `impl Iterator<Item=Into<T>>`, and if we had `BTreeMap<String, T>`,
    /// then we replace it with `impl Iterator<Item = (String, T)>` and
    /// we do this... recursively.
    // FIXME: Investigate if there's a better way.
    fn write_builder_ty<F>(
        &self,
        ty: &str,
        req: &[String],
        needs_any: bool,
        f: &mut F,
    ) -> fmt::Result
    where
        F: Write,
    {
        if let Some(i) = ty.find('<') {
            if ty[..i].ends_with("Vec") {
                f.write_str("impl Iterator<Item = ")?;
                self.write_builder_ty(&ty[i + 1..ty.len() - 1], req, needs_any, f)?;
                f.write_str(">")?;
            } else if ty[..i].ends_with("std::collections::BTreeMap") {
                f.write_str("impl Iterator<Item = (String, ")?;
                self.write_builder_ty(&ty[i + 9..ty.len() - 1], req, needs_any, f)?;
                f.write_str(")>")?;
            }
        } else if ApiObject::is_simple_type(ty) {
            write!(f, "impl Into<{}", ty)?;
            if needs_any && ty != ANY_GENERIC_PARAMETER {
                ApiObject::write_any_generic(f)?;
            }

            return f.write_str(">");
        } else {
            f.write_str(ty)?;
            if !req.is_empty() {
                f.write_str("Builder<")?;
                req.iter().enumerate().try_for_each(|(i, n)| {
                    if i > 0 {
                        f.write_str(", ")?;
                    }

                    f.write_str(self.0.helper_module_prefix)?;
                    f.write_str("generics::")?;
                    f.write_str(&n.to_camel_case())?;
                    f.write_str("Exists")
                })?;

                if needs_any {
                    f.write_str(", ")?;
                    f.write_str(ANY_GENERIC_PARAMETER)?;
                }

                f.write_str(">")?;
            } else if needs_any {
                ApiObject::write_any_generic(f)?;
            }
        }

        Ok(())
    }

    /// Builds the value conversion block using the actual field type.
    ///
    /// Once we get the value from a builder method (whose type is
    /// generated by `Self::write_builder_ty`), we need to convert it
    /// appropriately. So, whenever we encounter collections, we recursively
    /// collect the iterator items and if it's not a collection, we go for
    /// `value.into()`.
    fn write_value_map<F>(ty: &str, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        if let Some(i) = ty.find('<') {
            if ty[..i].ends_with("Vec") {
                f.write_str("value.map(|value| ")?;
                Self::write_value_map(&ty[i + 1..ty.len() - 1], f)?;
                f.write_str(").collect::<Vec<_>>()")?;
            } else if ty[..i].ends_with("std::collections::BTreeMap") {
                f.write_str("value.map(|(key, value)| (key, ")?;
                Self::write_value_map(&ty[i + 9..ty.len() - 1], f)?;
                f.write_str(")).collect::<std::collections::BTreeMap<_, _>>()")?;
            }
        } else {
            f.write_str("value")?;
        }

        // Always write `into` to ease conversions.
        f.write_str(".into()")
    }

    /// Writes the property-related methods to the given formatter.
    fn write_property_method<F>(&self, field: StructField<'b>, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        let field_name = field.name.to_snek_case();
        let (prop_is_parameter, prop_is_required, needs_container) = (
            field.prop.is_parameter(),
            field.prop.is_required(),
            self.0.needs_container(),
        );
        let needs_trailing_dash =
            field.prop.is_field() && RUST_KEYWORDS.iter().any(|&k| k == field_name);

        ApiObject::write_docs(field.desc, f, 1)?;
        if field.desc.is_none() {
            f.write_str("\n")?;
        }

        // Inline property methods.
        f.write_str("    #[inline]\n    pub fn ")?;
        if RUST_KEYWORDS.iter().any(|&k| k == field_name) {
            f.write_str("r#")?;
        }

        f.write_str(&field_name)?;
        f.write_str("(mut self, value: ")?;
        self.write_builder_ty(&field.ty, &field.strict_child_fields, field.needs_any, f)?;

        f.write_str(") -> ")?;
        if prop_is_required {
            self.0.write_name(f)?;
            self.0
                .write_generics_if_necessary(f, None, TypeParameters::ChangeOne(field.name))?;
        } else {
            f.write_str("Self")?;
        }

        f.write_str(" {\n        self.")?;
        if needs_container {
            f.write_str("inner.")?;
        }

        if prop_is_parameter {
            f.write_str("param_")?;
        // If it's not a parameter, then it's definitely a body field.
        } else if self.0.body_required {
            f.write_str("body.")?;
        }

        f.write_str(&field_name)?;
        if needs_trailing_dash {
            f.write_str("_")?;
        }

        f.write_str(" = ")?;
        if prop_is_parameter || !prop_is_required {
            f.write_str("Some(")?;
        }

        // If there's a field in the body with similar name and type,
        // then override it with this value.
        if field.overridden && self.0.body_required {
            f.write_str("{\n            let val = ")?;
            Self::write_value_map(field.ty, f)?;
            f.write_str(";\n            self.")?;
            if needs_container {
                f.write_str("inner.")?;
            }

            f.write_str("body.")?;
            f.write_str(&field_name)?;
            if needs_trailing_dash {
                f.write_str("_")?;
            }

            f.write_str(" = val.clone().into();")?;
            f.write_str("\n            val\n        }")?;
        } else {
            Self::write_value_map(field.ty, f)?;
        }

        if prop_is_parameter || !prop_is_required {
            f.write_str(")")?;
        }

        f.write_str(";\n        ")?;
        // We need to transmute only if there's a required field/parameter.
        if prop_is_required {
            f.write_str("unsafe { std::mem::transmute(self) }")?;
        } else {
            f.write_str("self")?;
        }

        f.write_str("\n    }\n")
    }
}

/// Codegen for `Sendable` trait for operation builders.
struct SendableCodegen<'a, 'b> {
    builder: &'a ApiObjectBuilder<'b>,
    needs_container: bool,
    path_items: String,
    headers: String,
    form: String,
    query: String,
    multi_value_query: Vec<String>,
}

impl<'a, 'b> From<&'a ApiObjectBuilder<'b>> for SendableCodegen<'a, 'b> {
    fn from(builder: &'a ApiObjectBuilder<'b>) -> Self {
        SendableCodegen {
            builder,
            needs_container: builder.needs_container(),
            path_items: String::new(),
            headers: String::new(),
            form: String::new(),
            query: String::new(),
            multi_value_query: vec![],
        }
    }
}

impl<'a, 'b> SendableCodegen<'a, 'b> {
    /// Determine and write `Sendable` impl (if it's needed for this builder).
    fn write_impl_if_needed<F>(mut self, f: &mut F) -> fmt::Result
    where
        F: Write,
    {
        let (path, method) = match (self.builder.rel_path, self.builder.method) {
            (Some(p), Some(m)) => (p, m),
            _ => return Ok(()),
        };

        f.write_str("\nimpl<Client: ")?;
        f.write_str(self.builder.helper_module_prefix)?;
        f.write_str("client::ApiClient + Sync + 'static")?;

        if self.builder.needs_any {
            f.write_str(", Any: serde::Serialize")?;
        }

        f.write_str("> ")?;
        f.write_str(self.builder.helper_module_prefix)?;
        f.write_str("client::Sendable<Client> for ")?;
        self.builder.write_name(f)?;
        self.builder
            .write_generics_if_necessary(f, None, TypeParameters::ChangeAll)?;
        f.write_str(" {\n    type Output = ")?;
        if self.builder.is_list_op {
            f.write_str("Vec<")?;
        }

        if let Some(resp) = self.builder.response.ty_path.as_ref() {
            // If we've acquired a response type, then write that.
            f.write_str(resp)?;
        }

        // If the type has `Any` or if we don't know what we're going to get, then
        // assume we have to write `Any` type.
        let mut accepted_range = None;
        if self.builder.needs_any
            || self.builder.response.ty_path.is_none()
            || self.builder.response.contains_any
        {
            let (range, coder) = match self.builder.decoding {
                Some(&(ref r, ref c)) => (r.as_str(), c),
                None => ((*JSON_MIME).0.as_ref(), &*JSON_CODER),
            };

            accepted_range = Some(range);
            if self.builder.response.ty_path.is_some() {
                write!(f, "<{}>", coder.any_value)?;
            } else {
                f.write_str(&coder.any_value)?;
            }
        }

        if self.builder.is_list_op {
            f.write_str(">")?;
        }

        f.write_str(";\n\n    const METHOD: http::Method = http::Method::")?;
        f.write_str(&method.to_string().to_uppercase())?;
        f.write_str(";\n\n    fn rel_path(&self) -> std::borrow::Cow<'static, str> {\n        ")?;

        self.builder
            .struct_fields_iter()
            .for_each(|field| match field.param_loc {
                Some(ParameterIn::Path) => self.handle_path_param(field),
                Some(ParameterIn::Header) => self.handle_header_param(field),
                Some(ParameterIn::FormData) => self.handle_form_param(field),
                Some(ParameterIn::Query) => self.handle_query_param(field),
                _ => (),
            });

        // Determine if we need a `&'static str` or `String`
        if self.path_items.is_empty() {
            write!(f, "\"{}\".into()", path)?;
        } else {
            write!(f, "format!(\"{}\"{}).into()", path, self.path_items)?;
        }

        f.write_str("\n    }")?;

        // Check whether `modify` method needs to be overridden (i.e. body and other params).
        if self.builder.body_required
            || !self.form.is_empty()
            || !self.query.is_empty()
            || !self.multi_value_query.is_empty()
            || !self.headers.is_empty()
        {
            self.write_modify_method(f, accepted_range)?;
        }

        f.write_str("\n}\n")
    }

    /// Handle field for a path parameter.
    fn handle_path_param(&mut self, field: StructField) {
        let _ = write!(self.path_items, ", {}=self.", &field.name);
        let name = field.name.to_snek_case();
        if self.needs_container {
            self.path_items.push_str("inner.");
        }

        let _ = write!(
            self.path_items,
            "param_{name}.as_ref().expect(\"missing parameter {name}?\")",
            name = name
        );
    }

    /// Handle field for a header parameter.
    fn handle_header_param(&mut self, field: StructField) {
        let is_required = field.prop.is_required();
        let name = field.name.to_snek_case();
        let mut param_ref = String::from("&self.");
        if self.needs_container {
            param_ref.push_str("inner.");
        }

        param_ref.push_str("param_");
        param_ref.push_str(&name);
        param_ref.push_str(".as_ref().map(std::string::ToString::to_string)");
        if is_required {
            let _ = write!(param_ref, ".expect(\"missing parameter {}?\")", name);
        }

        if !is_required {
            let _ = write!(self.headers, "\n        if let Some(v) = {} {{", param_ref);
        }

        self.headers.push_str("\n        ");
        if !is_required {
            self.headers.push_str("    ");
        }

        let _ = write!(
            self.headers,
            "req = req.header({:?}, {});",
            &field.name,
            if is_required { &param_ref } else { "&v" }
        );

        if !is_required {
            self.headers.push_str("\n        }");
        }
    }

    /// Handle field for a form data parameter.
    fn handle_form_param(&mut self, field: StructField) {
        let name = field.name.to_snek_case();
        if let Some(CollectionFormat::Multi) = field.delimiting.get(0) {
            let _ = write!(
                self.form,
                "
            self.{}param_{}.as_ref().map(|v| v.iter().for_each(|v| {{
                ser.append_pair({:?}, &v.to_string());
            }}));",
                if self.needs_container { "inner." } else { "" },
                name,
                &field.name
            );

            return;
        }

        self.form.push_str("\n            self.");
        if self.needs_container {
            self.form.push_str("inner.");
        }

        let _ = write!(
            self.form,
            "{}.as_ref().map(|v| ser.append_pair({:?}, &v.to_string()));",
            name, &field.name
        );
    }

    /// Handle field for an URL query parameter.
    fn handle_query_param(&mut self, field: StructField) {
        if !self.query.is_empty() {
            self.query.push_str(",");
        }

        let name = field.name.to_snek_case();
        if let Some(CollectionFormat::Multi) = field.delimiting.get(0) {
            self.multi_value_query.push(format!(
                "
            &self.{}param_{}.as_ref().map(|v| {{
                v.iter().map(|v| ({:?}, v.to_string())).collect::<Vec<_>>()
            }}).unwrap_or_default()",
                if self.needs_container { "inner." } else { "" },
                name,
                &field.name,
            ));

            return;
        }

        let _ = write!(self.query, "\n            ({:?}, self.", &field.name);
        if self.needs_container {
            self.query.push_str("inner.");
        }

        let _ = write!(
            self.query,
            "param_{name}.as_ref().map(std::string::ToString::to_string))",
            name = name
        );
    }

    /// We have determined that we have to override the default `modify` method.
    fn write_modify_method<F>(self, f: &mut F, accepted_range: Option<&str>) -> fmt::Result
    where
        F: Write,
    {
        f.write_str("\n\n    fn modify(&self, req: Client::Request) -> Result<Client::Request, ")?;
        f.write_str(&self.builder.helper_module_prefix)?;
        f.write_str("client::ApiError<Client::Response>> {")?;
        f.write_str("\n        use ")?;
        f.write_str(&self.builder.helper_module_prefix)?;
        f.write_str("client::Request;")?;

        if !self.headers.is_empty() {
            f.write_str("\n        let mut req = req;")?;
            f.write_str(&self.headers)?;
            f.write_str("\n")?;
        }

        f.write_str("\n        Ok(req")?;
        if self.builder.body_required {
            f.write_str("\n        ")?;
            if let Some((range, coder)) = self.builder.encoding {
                write!(
                    f,
                    ".header(http::header::CONTENT_TYPE.as_str(), {:?})",
                    range
                )?;

                f.write_str(
                    "\n        .body_bytes({
            let mut vec = vec![];
            ",
                )?;
                f.write_str(&coder.encoder_path)?;
                f.write_str("(&mut vec, ")?;
            } else {
                f.write_str(".json(")?;
            }

            f.write_str("&self.")?;
            if self.needs_container {
                f.write_str("inner.")?;
            }

            f.write_str("body)")?;

            if self.builder.encoding.is_some() {
                f.write_str("?;\n            vec\n        })")?;
            }
        }

        if let Some(r) = accepted_range {
            write!(
                f,
                "\n        .header(http::header::ACCEPT.as_str(), {:?})",
                r
            )?;
        }

        if !self.form.is_empty() {
            f.write_str("\n        .body_bytes({\n            let mut ser = url::form_urlencoded::Serializer::new(String::new());")?;
            f.write_str(&self.form)?;
            f.write_str("\n            ser.finish().into_bytes()\n        })")?;
            write!(f, "\n        .header(http::header::CONTENT_TYPE.as_str(), \"application/x-www-form-urlencoded\")")?;
        }

        if !self.query.is_empty() {
            f.write_str("\n        .query(&[")?;
            f.write_str(&self.query)?;
            f.write_str("\n        ])")?;
        }

        for q in self.multi_value_query {
            f.write_str("\n        .query({")?;
            f.write_str(&q)?;
            f.write_str("\n        })")?;
        }

        f.write_str(")\n    }")
    }
}

impl<'a> Display for ApiObjectImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let needs_any = self.inner.fields.iter().any(|f| f.needs_any);
        if self.builders.is_empty() {
            return Ok(());
        }

        f.write_str("impl")?;
        if needs_any {
            f.write_str("<")?;
            f.write_str(ANY_GENERIC_PARAMETER)?;
            f.write_str(": Default")?;
            f.write_str(">")?;
        }

        f.write_str(" ")?;
        f.write_str(&self.inner.name)?;
        if needs_any {
            ApiObject::write_any_generic(f)?;
        }

        f.write_str(" {")?;
        self.write_builder_methods(f)?;
        f.write_str("}\n")?;

        for builder in &*self.builders {
            self.write_into_impl(builder, f)?;
        }

        Ok(())
    }
}

impl<'a, 'b> Display for ApiObjectBuilderImpl<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut generics = String::new();
        self.0
            .write_generics_if_necessary(&mut generics, None, TypeParameters::Generic)?;

        let mut has_fields = false;
        self.0
            .struct_fields_iter()
            .filter(|f| (self.0.body_required && f.prop.is_field()) || f.prop.is_parameter())
            .enumerate()
            .try_for_each(|(i, field)| {
                if i == 0 {
                    has_fields = true;
                    f.write_str("impl")?;
                    f.write_str(&generics)?;
                    f.write_str(" ")?;
                    self.0.write_name(f)?;
                    f.write_str(&generics)?;
                    f.write_str(" {")?;
                }

                self.write_property_method(field, f)
            })?;

        if has_fields {
            f.write_str("}\n")?;
        }

        SendableCodegen::from(self.0).write_impl_if_needed(f)
    }
}
