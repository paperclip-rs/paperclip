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
    pub(super) fn new(template: TemplateFile, target_prefix: &str, target_postfix: &str) -> Self {
        let gen = GenTemplateFile {
            template,
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
    pub(super) fn new(template: TemplateFile, target_prefix: &str, ext: &str) -> Self {
        let gen = GenTemplateFile {
            template,
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
    pub(super) fn new(template: TemplateFile, target_prefix: &str, target_postfix: &str) -> Self {
        let gen = GenTemplateFile {
            template,
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
    BufferOwned(String),
}
impl TemplateFile {
    /// Get the template raw buffer.
    pub(super) fn buffer(&self) -> Option<&str> {
        match self {
            TemplateFile::Path(_) => None,
            TemplateFile::Buffer(buffer) => Some(buffer),
            TemplateFile::BufferOwned(buffer) => Some(buffer.as_str()),
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

macro_rules! path_or_builtin {
    ($tpl:ident, $offset:literal) => {
        match $tpl {
            None => TemplateFile::Buffer(include_str!($offset)),
            Some(tpl) => {
                let path = format!("{}/{}", tpl.display(), $offset);
                let buffer = std::fs::read_to_string(path)?;
                TemplateFile::BufferOwned(buffer)
            }
        }
    };
}

/// The template files for thes default template.
#[allow(clippy::type_complexity)]
pub(super) fn default_templates(
    tpl_path: &Option<std::path::PathBuf>,
) -> Result<
    (
        Vec<ApiTemplateFile>,
        Vec<ModelTemplateFile>,
        Vec<SuppTemplateFile>,
    ),
    std::io::Error,
> {
    let api_templates = vec![
        // Actix
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/client/api_clients.mustache"),
            "src/apis",
            "actix/client/mod.rs",
        ),
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/server/handlers.mustache"),
            "src/apis",
            "actix/server/handlers.rs",
        ),
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/mod.mustache"),
            "src/apis",
            "actix/mod.rs",
        ),
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/server/api.mustache"),
            "src/apis",
            "actix/server/mod.rs",
        ),
        // Tower-hyper
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/tower-hyper/mod.mustache"),
            "src/apis",
            "tower/mod.rs",
        ),
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/tower-hyper/client/api_clients.mustache"),
            "src/apis",
            "tower/client/mod.rs",
        ),
        // Common
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/mod.mustache"),
            "src/apis",
            "mod.rs",
        ),
        ApiTemplateFile::new(
            path_or_builtin!(tpl_path, "default/api_doc.mustache"),
            "docs/apis",
            ".md",
        )
        .with_class_case(ClassCase::PascalCase)
        .with_class(ApiTargetFileCfg::ClassName),
    ];
    let model_templates = vec![
        ModelTemplateFile::new(
            path_or_builtin!(tpl_path, "default/model.mustache"),
            "src/models",
            ".rs",
        ),
        ModelTemplateFile::new(
            path_or_builtin!(tpl_path, "default/model_doc.mustache"),
            "docs/models",
            ".md",
        )
        .with_file_case(ClassCase::PascalCase),
    ];
    let supporting_templates = vec![
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/model_mod.mustache"),
            "src/models",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/client/configuration.mustache"),
            "src/clients/actix",
            "configuration.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(
                tpl_path,
                "default/tower-hyper/client/configuration.mustache"
            ),
            "src/clients/tower",
            "configuration.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/client/client.mustache"),
            "src/clients/actix",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/tower-hyper/client/client.mustache"),
            "src/clients/tower",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/tower-hyper/client/body.mustache"),
            "src/clients/tower",
            "body.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/api_mod.mustache"),
            "src/apis",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/mod_clients.mustache"),
            "src/clients",
            "mod.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/lib.mustache"),
            "src/",
            "lib.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/actix/server/api_mod.mustache"),
            "src/apis",
            "actix_server.rs",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/Cargo.mustache"),
            "",
            "Cargo.toml",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/gitignore.mustache"),
            "",
            ".gitignore",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/openapi.mustache"),
            "src/apis",
            "openapi.yaml",
        ),
        SuppTemplateFile::new(
            path_or_builtin!(tpl_path, "default/README.mustache"),
            "",
            "README.md",
        ),
    ];
    Ok((api_templates, model_templates, supporting_templates))
}
