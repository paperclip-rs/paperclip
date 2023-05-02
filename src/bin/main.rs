use anyhow::Error;
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
    /// Render.
    #[structopt(long)]
    models: bool,
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
}

fn parse_args_and_run() -> Result<(), Error> {
    let opt: Opt = Opt::from_args();

    if let OApiVersion::V3 = opt.api {
        let spec = parse_spec_v3(&opt.spec.to_string_lossy().to_string())?;
        paperclip::v3::OpenApiV3::new(spec, opt.output).run(opt.models)?;
        return Ok(());
    }

    let spec = parse_spec(&opt.spec.to_string_lossy().to_string())?.resolve()?;
    let mut state = EmitterState::default();

    if let Some(o) = opt.output {
        fs::create_dir_all(&o)?;
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
