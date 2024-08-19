use anyhow::Error;
#[cfg(feature = "v3-poc")]
use heck::ToSnakeCase;
use paperclip::{
    v2::{
        self,
        codegen::{CrateMeta, DefaultEmitter, EmitMode, Emitter, EmitterState},
        models::{DefaultSchema, ResolvableApi},
    },
    PaperClipError,
};
use structopt::StructOpt;

use std::{
    fs::{self, File},
    path::PathBuf,
};

fn parse_version(s: &str) -> Result<OApiVersion, Error> {
    match s {
        "v2" => Ok(OApiVersion::V2),
        "v3" => Ok(OApiVersion::V3),
        _ => Err(PaperClipError::UnsupportedOpenAPIVersion.into()),
    }
}

fn parse_spec(s: &str) -> Result<ResolvableApi<DefaultSchema>, Error> {
    let fd = File::open(s)?;
    Ok(v2::from_reader(fd)?)
}
#[cfg(feature = "v3-poc")]
fn parse_spec_v3(s: &str) -> Result<openapiv3::OpenAPI, Error> {
    let fd = File::open(s)?;
    Ok(v2::from_reader_v3(fd)?)
}

#[derive(Debug)]
enum OApiVersion {
    V2,
    V3,
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to OpenAPI spec in JSON/YAML format (also supports publicly accessible URLs).
    #[structopt(long)]
    spec: std::path::PathBuf,
    /// OpenAPI version (e.g., v2).
    #[structopt(long = "api", parse(try_from_str = parse_version))]
    api: OApiVersion,
    /// Output directory to write code (default: current working directory).
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Emit CLI target instead.
    #[structopt(long = "cli")]
    cli: bool,
    #[cfg(feature = "v3-poc")]
    /// Don't Render models.
    #[structopt(long)]
    no_models: bool,
    #[cfg(feature = "v3-poc")]
    /// Don't Render operations.
    #[structopt(long)]
    no_ops: bool,
    /// Do not make the crate a root crate.
    #[structopt(long = "no-root")]
    no_root: bool,
    /// Name of the crate. If this is not specified, then the name of the
    /// working directory is assumed to be crate name.
    #[structopt(long = "name")]
    pub name: Option<String>,
    /// Version (defaults to 0.1.0)
    #[structopt(long = "version")]
    pub version: Option<String>,
    #[cfg(feature = "v3-poc")]
    /// The Edition of the crate.
    #[structopt(long = "edition", default_value = "2018")]
    pub edition: String,
}

fn parse_args_and_run() -> Result<(), Error> {
    let opt: Opt = Opt::from_args();

    if let Some(o) = &opt.output {
        fs::create_dir_all(o)?;
    }

    #[cfg(feature = "v3-poc")]
    if let OApiVersion::V3 = opt.api {
        let spec = parse_spec_v3(opt.spec.to_string_lossy().as_ref())?;
        let name = opt.name.map(Ok::<String, Error>).unwrap_or_else(|| {
            Ok(fs::canonicalize(std::path::Path::new("."))?
                .file_name()
                .ok_or(PaperClipError::InvalidCodegenDirectory)?
                .to_string_lossy()
                .into_owned()
                .to_snake_case())
        })?;
        let info = paperclip::v3::PackageInfo {
            libname: name.to_snake_case(),
            name,
            version: opt.version.unwrap_or_else(|| "0.1.0".into()),
            edition: opt.edition,
        };
        paperclip::v3::OpenApiV3::new(spec, opt.output, info).run(!opt.no_models, !opt.no_ops)?;
        return Ok(());
    }

    #[cfg(not(feature = "v3-poc"))]
    if let OApiVersion::V3 = opt.api {
        return Err(PaperClipError::UnsupportedOpenAPIVersion.into());
    }

    let spec = parse_spec(&opt.spec.to_string_lossy())?.resolve()?;
    let mut state = EmitterState::default();

    if let Some(o) = opt.output {
        state.working_dir = o;
    }

    let mut meta = CrateMeta::default();
    if opt.cli {
        meta.mode = EmitMode::App;
    } else {
        meta.mode = EmitMode::Crate;
    }
    if opt.name.is_some() {
        meta.name = opt.name;
    }
    if opt.version.is_some() {
        meta.version = opt.version;
    }

    meta.no_root = opt.no_root;

    state.set_meta(meta);
    let emitter = DefaultEmitter::from(state);
    emitter.generate(&spec)
}

fn main() {
    env_logger::init();
    if let Err(e) = parse_args_and_run() {
        eprintln!("{}", e);
    }
}
