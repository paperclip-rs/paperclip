use std::ops::Deref;

use crate::v3::{OperationsApiTpl, Property};
use heck::{ToPascalCase, ToSnakeCase};

/// Support cases for the class names.
#[derive(Debug, Clone)]
pub(super) enum ClassCase {
    PascalCase,
    SnakeCase,
}
impl ClassCase {
    fn format(&self, name: &str) -> String {
        match self {
            ClassCase::PascalCase => name.to_pascal_case(),
            ClassCase::SnakeCase => name.to_snake_case(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum ApiTargetFileCfg {
    /// $prefix/$class_fname/$postfix
    ClassFileName,
    /// $prefix/$class/$postfix
    ClassName,
}

/// Template file specification for support files.
#[derive(Debug, Clone)]
pub(super) struct SuppTemplateFile {
    gen: GenTemplateFile,
}
impl SuppTemplateFile {
    /// Create a new `Self` by specifying the template path, the target prefix and file extension.
    pub(super) fn new(template: &'static str, target_prefix: &str, target_postfix: &str) -> Self {
        let gen = GenTemplateFile {
            template: TemplateFile::Buffer(template),
            target_prefix: std::path::PathBuf::from(target_prefix),
            target_postfix: std::path::PathBuf::from(target_postfix),
            casing: ClassCase::PascalCase,
        };
        Self { gen }
    }
}
impl Deref for SuppTemplateFile {
    type Target = GenTemplateFile;
    fn deref(&self) -> &Self::Target {
        &self.gen
    }
}

/// Template file specification for model files.
#[derive(Debug, Clone)]
pub(super) struct ModelTemplateFile {
    gen: GenTemplateFile,
    extension: String,
}
impl ModelTemplateFile {
    /// Create a new `Self` by specifying the template path, the target prefix and file extension.
    pub(super) fn new(template: &'static str, target_prefix: &str, ext: &str) -> Self {
        let gen = GenTemplateFile {
            template: TemplateFile::Buffer(template),
            target_prefix: std::path::PathBuf::from(target_prefix),
            target_postfix: std::path::PathBuf::from("."),
            casing: ClassCase::SnakeCase,
        };
        Self {
            gen,
            extension: ext.trim_start_matches('.').into(),
        }
    }
    /// Override the class file case.
    pub(super) fn with_file_case(mut self, casing: ClassCase) -> Self {
        self.gen = self.gen.with_class_case(casing);
        self
    }
    /// Generate the path for the model file.
    pub(super) fn model_path(&self, property: &Property) -> std::path::PathBuf {
        let model_fname = self.casing.format(property.filename());
        self.gen
            .target_prefix
            .join(model_fname)
            .with_extension(&self.extension)
    }
}
impl Deref for ModelTemplateFile {
    type Target = GenTemplateFile;
    fn deref(&self) -> &Self::Target {
        &self.gen
    }
}

/// Template file specification for api files.
#[derive(Debug, Clone)]
pub(super) struct ApiTemplateFile {
    gen: GenTemplateFile,
    kind: ApiTargetFileCfg,
}
impl ApiTemplateFile {
    /// Create a new `Self` by specifying the template path, the target prefix and postfix.
    pub(super) fn new(template: &'static str, target_prefix: &str, target_postfix: &str) -> Self {
        let gen = GenTemplateFile {
            template: TemplateFile::Buffer(template),
            target_prefix: std::path::PathBuf::from(target_prefix),
            target_postfix: std::path::PathBuf::from(target_postfix),
            casing: ClassCase::SnakeCase,
        };
        Self {
            gen,
            kind: ApiTargetFileCfg::ClassFileName,
        }
    }
    /// Override the class file case.
    pub(super) fn with_class_case(mut self, casing: ClassCase) -> Self {
        self.gen = self.gen.with_class_case(casing);
        self
    }
    /// Override the class type.
    pub(super) fn with_class(mut self, kind: ApiTargetFileCfg) -> Self {
        self.kind = kind;
        self
    }
    /// Generate the path for the api file.
    pub(super) fn api_path(&self, api: &OperationsApiTpl) -> std::path::PathBuf {
        let prefix = self.target_prefix();
        let postfix = self.gen.target_postfix().display().to_string();
        let class = self.casing.format(match &self.kind {
            ApiTargetFileCfg::ClassFileName => api.class_filename(),
            ApiTargetFileCfg::ClassName => api.classname(),
        });
        match postfix.starts_with('.') {
            true => prefix
                .join(class)
                .with_extension(postfix.trim_start_matches('.')),
            false => prefix.join(class).join(postfix),
        }
    }
}
impl Deref for ApiTemplateFile {
    type Target = GenTemplateFile;
    fn deref(&self) -> &Self::Target {
        &self.gen
    }
}

#[derive(Debug, Clone)]
pub(crate) enum TemplateFile {
    #[allow(unused)]
    Path(std::path::PathBuf),
    Buffer(&'static str),
}
impl TemplateFile {
    /// Get the template file path.
    #[allow(dead_code)]
    pub(super) fn path(&self) -> Option<&std::path::PathBuf> {
        match self {
            TemplateFile::Path(path) => Some(path),
            TemplateFile::Buffer(_) => None,
        }
    }
    /// Get the template raw buffer.
    pub(super) fn buffer(&self) -> Option<&'static str> {
        match self {
            TemplateFile::Path(_) => None,
            TemplateFile::Buffer(buffer) => Some(buffer),
        }
    }
}

/// A generic template file specification.
#[derive(Debug, Clone)]
pub(super) struct GenTemplateFile {
    template: TemplateFile,
    target_prefix: std::path::PathBuf,
    target_postfix: std::path::PathBuf,
    casing: ClassCase,
}
impl GenTemplateFile {
    /// Override the class file case.
    pub(super) fn with_class_case(mut self, casing: ClassCase) -> Self {
        self.casing = casing;
        self
    }
    /// Get the template input file.
    pub(super) fn input(&self) -> &TemplateFile {
        &self.template
    }
    /// Get the target path prefix.
    pub(super) fn target_prefix(&self) -> &std::path::PathBuf {
        &self.target_prefix
    }
    /// Get the target path postfix.
    pub(super) fn target_postfix(&self) -> &std::path::PathBuf {
        &self.target_postfix
    }
}

pub(super) fn default_templates() -> (
    Vec<ApiTemplateFile>,
    Vec<ModelTemplateFile>,
    Vec<SuppTemplateFile>,
) {
    let api_templates = vec![
        // Actix
        ApiTemplateFile::new(
            include_str!("default/actix/client/api_clients.mustache"),
            "apis",
            "actix/client/mod.rs",
        ),
        ApiTemplateFile::new(
            include_str!("default/actix/server/handlers.mustache"),
            "apis",
            "actix/server/handlers.rs",
        ),
        ApiTemplateFile::new(
            include_str!("default/actix/mod.mustache"),
            "apis",
            "actix/mod.rs",
        ),
        ApiTemplateFile::new(
            include_str!("default/actix/server/api.mustache"),
            "apis",
            "actix/server/mod.rs",
        ),
        // Tower-hyper
        ApiTemplateFile::new(
            include_str!("default/tower-hyper/mod.mustache"),
            "apis",
            "tower/mod.rs",
        ),
        ApiTemplateFile::new(
            include_str!("default/tower-hyper/client/api_clients.mustache",),
            "apis",
            "tower/client/mod.rs",
        ),
        // Common
        ApiTemplateFile::new(include_str!("default/mod.mustache"), "apis", "mod.rs"),
        ApiTemplateFile::new(include_str!("default/api_doc.mustache"), "docs/apis", ".md")
            .with_class_case(ClassCase::PascalCase)
            .with_class(ApiTargetFileCfg::ClassName),
    ];
    let model_templates = vec![
        ModelTemplateFile::new(include_str!("default/model.mustache"), "models", ".rs"),
        ModelTemplateFile::new(
            include_str!("default/model_doc.mustache"),
            "docs/models",
            ".md",
        )
        .with_file_case(ClassCase::PascalCase),
    ];
    let supporting_templates = vec![
        SuppTemplateFile::new(
            include_str!("default/model_mod.mustache"),
            "models",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            include_str!("default/tower-hyper/client/configuration.mustache",),
            "clients/tower",
            "configuration.rs",
        ),
        SuppTemplateFile::new(
            include_str!("default/tower-hyper/client/client.mustache"),
            "clients/tower",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            include_str!("default/tower-hyper/client/body.mustache"),
            "clients/tower",
            "body.rs",
        ),
        SuppTemplateFile::new(include_str!("default/api_mod.mustache"), "apis", "mod.rs"),
        SuppTemplateFile::new(
            include_str!("default/mod_clients.mustache"),
            "clients",
            "mod.rs",
        ),
        SuppTemplateFile::new(include_str!("default/lib.mustache"), "", "mod.rs"),
        SuppTemplateFile::new(
            include_str!("default/actix/server/api_mod.mustache"),
            "apis",
            "actix_server.rs",
        ),
        SuppTemplateFile::new(include_str!("default/Cargo.mustache"), "", "Cargo.toml"),
        SuppTemplateFile::new(include_str!("default/gitignore.mustache"), "", ".gitignore"),
        SuppTemplateFile::new(
            include_str!("default/openapi.mustache"),
            "apis",
            "openapi.yaml",
        ),
    ];
    (api_templates, model_templates, supporting_templates)
}
