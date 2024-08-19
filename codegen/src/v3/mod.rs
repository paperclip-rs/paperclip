mod operation;
mod parameter;
mod property;
mod templates;

use std::{cell::RefCell, collections::HashSet, ops::Deref};

use operation::Operation;
use parameter::Parameter;
use property::Property;
use templates::*;

use itertools::Itertools;
use ramhorns::Template;
use ramhorns_derive::Content;

/// OpenApiV3 code generator.
#[derive(Debug)]
pub struct OpenApiV3 {
    api: openapiv3::OpenAPI,

    output_path: std::path::PathBuf,
    package_info: PackageInfo,

    api_template: Vec<ApiTemplateFile>,
    model_templates: Vec<ModelTemplateFile>,
    supporting_templates: Vec<SuppTemplateFile>,

    suppress_errors: bool,
    circ_ref_checker: RefCell<CircularRefChecker>,
}
impl OpenApiV3 {
    /// Creates a new OpenApi V3 Generator.
    pub fn new(
        api: openapiv3::OpenAPI,
        output_path: Option<std::path::PathBuf>,
        package_info: PackageInfo,
    ) -> Self {
        let output_path = output_path.unwrap_or_else(|| std::path::Path::new(".").to_path_buf());
        let (api_template, model_templates, supporting_templates) = templates::default_templates();
        Self {
            api,
            output_path,
            package_info,
            api_template,
            model_templates,
            supporting_templates,
            suppress_errors: false,
            circ_ref_checker: RefCell::new(CircularRefChecker::default()),
        }
    }
}

#[derive(Debug)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub libname: String,
    pub edition: String,
}

#[derive(Clone, Content)]
struct ApiInfoTpl<'a> {
    apis: &'a Vec<OperationsApiTpl<'a>>,
}
#[derive(Clone, Content)]
#[ramhorns(rename_all = "camelCase")]
struct SupportingTpl<'a> {
    api_info: ApiInfoTpl<'a>,
    operations: OperationsTpl<'a>,
    models: ModelTpl<'a>,
    package_name: &'a str,
    package_version: &'a str,
    package_libname: &'a str,
    package_edition: &'a str,
}
#[derive(Clone, Content)]
#[ramhorns(rename_all = "camelCase")]
struct ModelsTpl<'a> {
    models: ModelTpl<'a>,
}
#[derive(Clone, Content)]
struct ModelTpl<'a> {
    model: &'a Vec<Property>,
}

#[derive(Content, Debug, Clone)]
struct OperationsTpl<'a> {
    operation: &'a Vec<Operation>,
}

#[derive(Content, Clone, Debug)]
#[ramhorns(rename_all = "camelCase")]
pub(super) struct OperationsApiTpl<'a> {
    classname: &'a str,
    class_filename: &'a str,

    operations: OperationsTpl<'a>,
}
pub(super) struct OperationsApi {
    classname: String,
    class_filename: String,

    operations: Vec<Operation>,
}

impl OpenApiV3 {
    /// Run the OpenApi V3 Code Generator.
    pub fn run(&self, models: bool, ops: bool) -> Result<(), std::io::Error> {
        let models = if models { self.models()? } else { vec![] };
        let operations = if ops { self.operations()? } else { vec![] };
        let apis = self.apis(&operations)?;
        let apis = apis
            .iter()
            .map(|o| OperationsApiTpl {
                classname: o.classname(),
                class_filename: o.class_filename(),
                operations: OperationsTpl {
                    operation: &o.operations,
                },
            })
            .collect::<Vec<_>>();

        self.ensure_templates()?;

        self.render_supporting(&models, &operations, &apis)?;
        self.render_models(&models)?;
        self.render_apis(&apis)?;

        Ok(())
    }
    fn ensure_templates(&self) -> Result<(), std::io::Error> {
        Self::ensure_path(&self.output_path, true)?;
        let templates = self
            .supporting_templates
            .iter()
            .map(Deref::deref)
            .chain(self.api_template.iter().map(Deref::deref))
            .chain(self.model_templates.iter().map(Deref::deref))
            .collect::<Vec<_>>();
        self.ensure_template(&templates)
    }
    fn ensure_template_path(
        &self,
        path: &std::path::Path,
        clean: bool,
    ) -> Result<(), std::io::Error> {
        let path = self.output_path.join(path);
        Self::ensure_path(&path, clean)
    }
    fn ensure_path(path: &std::path::Path, clean: bool) -> Result<(), std::io::Error> {
        if clean && path.exists() {
            if path.is_dir() {
                std::fs::remove_dir_all(path)?;
            } else {
                std::fs::remove_file(path)?;
            }
        }
        std::fs::create_dir_all(path)
    }
    fn ensure_template(&self, templates: &[&GenTemplateFile]) -> Result<(), std::io::Error> {
        templates
            .iter()
            .try_for_each(|template| self.ensure_template_path(template.target_prefix(), true))?;
        templates
            .iter()
            .try_for_each(|template| self.ensure_template_path(template.target_prefix(), false))
    }
    fn render_supporting(
        &self,
        models: &Vec<Property>,
        operations: &Vec<Operation>,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        self.supporting_templates
            .iter()
            .try_for_each(|e| self.render_supporting_template(e, models, operations, apis))
    }
    fn render_apis(&self, apis: &Vec<OperationsApiTpl>) -> Result<(), std::io::Error> {
        self.api_template
            .iter()
            .try_for_each(|e| self.render_template_apis(e, apis))
    }
    fn render_models(&self, models: &Vec<Property>) -> Result<(), std::io::Error> {
        for property in models {
            let model = &vec![property.clone()];
            for template in &self.model_templates {
                let tpl = self.tpl(template)?;

                let path = self.output_path.join(template.model_path(property));

                tpl.render_to_file(
                    path,
                    &ModelsTpl {
                        models: ModelTpl { model },
                    },
                )?;
            }
        }

        Ok(())
    }

    fn tpl(&self, template: &GenTemplateFile) -> Result<Template, std::io::Error> {
        let Some(mustache) = template.input().buffer() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Template from path not supported yet",
            ));
        };
        let tpl = Template::new(mustache).map_err(|error| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, error.to_string())
        })?;

        Ok(tpl)
    }

    fn render_supporting_template(
        &self,
        template: &SuppTemplateFile,
        models: &Vec<Property>,
        operations: &Vec<Operation>,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        let tpl = self.tpl(template)?;

        let path = self
            .output_path
            .join(template.target_prefix())
            .join(template.target_postfix());
        tpl.render_to_file(
            path,
            &SupportingTpl {
                api_info: ApiInfoTpl { apis },
                operations: OperationsTpl {
                    operation: operations,
                },
                models: ModelTpl { model: models },
                package_name: self.package_info.name.as_str(),
                package_version: self.package_info.version.as_str(),
                package_libname: self.package_info.libname.as_str(),
                package_edition: self.package_info.edition.as_str(),
            },
        )?;

        Ok(())
    }
    #[allow(unused)]
    fn render_template_models(
        &self,
        template: &ModelTemplateFile,
        models: &Vec<Property>,
    ) -> Result<(), std::io::Error> {
        let tpl = self.tpl(template)?;

        for model in models {
            let path = self.output_path.join(template.model_path(model));
            let model = &vec![model.clone()];
            tpl.render_to_file(
                path,
                &ModelsTpl {
                    models: ModelTpl { model },
                },
            )?;
        }

        Ok(())
    }
    fn render_template_apis(
        &self,
        template: &ApiTemplateFile,
        apis: &Vec<OperationsApiTpl>,
    ) -> Result<(), std::io::Error> {
        let tpl = self.tpl(template)?;

        for api in apis {
            let path = self.output_path.join(template.api_path(api));
            if let Some(parent) = path.parent() {
                // we already cleaned the top-level, don't clean it again as we might have other templates
                // with the form $output/$target-folder/$api-classname/$any
                Self::ensure_path(parent, false)?;
            }
            tpl.render_to_file(path, api)?;
        }

        Ok(())
    }

    fn models(&self) -> Result<Vec<Property>, std::io::Error> {
        let model = self
            .api
            .components
            .as_ref()
            .unwrap()
            .schemas
            .iter()
            //.filter(|(name, _)| name.starts_with("ReplicaSpec"))
            .map(|(name, ref_or)| {
                let model = self.resolve_reference_or(ref_or, None, None, Some(name));
                debug!("Model: {} => {}", name, model);
                model
            })
            .flat_map(|m| m.discovered_models().into_iter().chain(vec![m]))
            .filter(|m| m.is_model() && !m.data_type().is_empty())
            .map(Self::post_process)
            // todo: when discovering models we should use a cache to avoid re-processing models
            // then we won't need to do this dedup.
            .sorted_by(|a, b| a.schema().cmp(b.schema()))
            .dedup_by(|a, b| a.schema() == b.schema())
            .inspect(|model| debug!("Model => {}", model))
            .collect::<Vec<Property>>();
        Ok(model)
    }
    fn operations(&self) -> Result<Vec<Operation>, std::io::Error> {
        let operation = self
            .api
            .operations()
            .map(|(path, method, operation)| Operation::new(self, path, method, operation))
            .collect::<Vec<Operation>>();

        Ok(operation)
    }
    fn apis(&self, operations: &Vec<Operation>) -> Result<Vec<OperationsApi>, std::io::Error> {
        let mut tags = std::collections::HashMap::<String, OperationsApi>::new();
        for op in operations {
            for tag in op.tags() {
                match tags.get_mut(tag) {
                    Some(api) => {
                        api.add_op(op);
                    }
                    None => {
                        tags.insert(tag.clone(), op.into());
                    }
                }
            }
        }

        // let apis = tags
        //     .clone()
        //     .into_values()
        //     .map(|o| o.classname().to_string())
        //     .collect::<Vec<_>>();
        // debug!("apis: {:?}", apis);

        Ok(tags
            .into_values()
            .sorted_by(|l, r| l.classname().cmp(r.classname()))
            .collect::<Vec<_>>())
    }
}

impl OpenApiV3 {
    fn missing_schema_ref(&self, reference: &str) {
        if !self.suppress_errors {
            println!("Schema reference({}) not found", reference);
        }
    }
    fn contains_schema(&self, type_: &str) -> bool {
        let contains = match &self.api.components {
            None => false,
            Some(components) => components.schemas.contains_key(type_),
        };
        trace!("Contains {} => {}", type_, contains);
        contains
    }
    fn set_resolving(&self, type_name: &str) {
        let mut checker = self.circ_ref_checker.borrow_mut();
        checker.add(type_name);
    }
    fn resolving(&self, property: &Property) -> bool {
        let checker = self.circ_ref_checker.borrow();
        checker.exists(property.type_ref())
    }
    fn clear_resolving(&self, type_name: &str) {
        let mut checker = self.circ_ref_checker.borrow_mut();
        checker.remove(type_name);
    }
    fn resolve_schema_name(&self, var_name: Option<&str>, reference: &str) -> Property {
        let type_name = match reference.strip_prefix("#/components/schemas/") {
            Some(type_name) => type_name,
            None => todo!("schema not found..."),
        };
        trace!("Resolving: {:?}/{}", var_name, type_name);
        let schemas = self.api.components.as_ref().map(|c| &c.schemas);
        match schemas.and_then(|s| s.get(type_name)) {
            None => {
                panic!("Schema {} Not found!", type_name);
            }
            Some(ref_or) => self.resolve_reference_or(ref_or, None, var_name, Some(type_name)),
        }
    }
    fn resolve_schema(
        &self,
        schema: &openapiv3::Schema,
        parent: Option<&Property>,
        name: Option<&str>,
        type_: Option<&str>,
    ) -> Property {
        trace!("ResolvingSchema: {:?}/{:?}", name, type_);
        if let Some(type_) = &type_ {
            self.set_resolving(type_);
        }
        let property = Property::from_schema(self, parent, schema, name, type_);
        if let Some(type_) = &type_ {
            self.clear_resolving(type_);
        }
        property
    }

    fn resolve_reference_or(
        &self,
        reference: &openapiv3::ReferenceOr<openapiv3::Schema>,
        parent: Option<&Property>,
        name: Option<&str>,  // parameter name, only known for object vars
        type_: Option<&str>, // type, only known when walking the component schema list
    ) -> Property {
        match reference {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.resolve_schema_name(name, reference)
            }
            openapiv3::ReferenceOr::Item(schema) => {
                self.resolve_schema(schema, parent, name, type_)
            }
        }
    }
    fn resolve_reference_or_resp(
        &self,
        content: &str,
        reference: &openapiv3::ReferenceOr<openapiv3::Response>,
    ) -> Property {
        debug!("Response: {reference:?}");
        match reference {
            openapiv3::ReferenceOr::Reference { reference } => {
                self.resolve_schema_name(None, reference)
            }
            openapiv3::ReferenceOr::Item(item) => match item.content.get(content) {
                Some(media) => match &media.schema {
                    Some(schema) => self.resolve_reference_or(schema, None, None, None),
                    None => Property::default(),
                },
                None => Property::default().with_data_property(&property::PropertyDataType::Empty),
            },
        }
    }

    fn post_process(property: Property) -> Property {
        property.post_process()
    }
}

impl OperationsApiTpl<'_> {
    /// Get a reference to the api classname.
    pub fn classname(&self) -> &str {
        self.classname
    }
    /// Get a reference to the api class filename.
    pub fn class_filename(&self) -> &str {
        self.class_filename
    }
}
impl OperationsApi {
    /// Get a reference to the api classname.
    pub fn classname(&self) -> &str {
        &self.classname
    }
    /// Get a reference to the api class filename.
    pub fn class_filename(&self) -> &str {
        &self.class_filename
    }
    /// Add the given operation.
    pub(super) fn add_op(&mut self, operation: &Operation) {
        self.operations.push(operation.clone());
    }
}

impl From<&Operation> for OperationsApi {
    fn from(src: &Operation) -> OperationsApi {
        OperationsApi {
            class_filename: src.class_filename().into(),
            classname: src.classname().into(),
            operations: vec![src.clone()],
        }
    }
}

/// Circular Reference Checker
/// If a model's member variable references a model currently being resolved
/// (either parent, or another elder) then a reference check must be used
/// to break out of an infinit loop.
/// In this case we don't really need to re-resolve the entire model
/// because the model itself will resolve itself.
#[derive(Clone, Debug, Default)]
struct CircularRefChecker {
    /// List of type_names in the resolve chain.
    type_names: HashSet<String>,
    /// Current type being resolved.
    current: String,
}
impl CircularRefChecker {
    fn add(&mut self, type_name: &str) {
        if self.type_names.insert(type_name.to_string()) {
            // trace!("Added cache: {type_name}");
            self.current = type_name.to_string();
        }
    }
    fn exists(&self, type_name: &str) -> bool {
        self.current.as_str() != type_name && self.type_names.contains(type_name)
    }
    fn remove(&mut self, type_name: &str) {
        if self.type_names.remove(type_name) {
            // trace!("Removed cache: {type_name}");
        }
    }
}
