#[macro_use]
extern crate log;

use failure::Error;
use structopt::StructOpt;

use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};
use paperclip::PaperClipError;

use std::fs::{self, File};
use std::path::PathBuf;

fn parse_version(s: &str) -> Result<OApiVersion, Error> {
    Ok(match s {
        "v2" => OApiVersion::V2,
        "v3" => OApiVersion::V3,
        _ => Err(PaperClipError::UnsupportedOpenAPIVersion)?,
    })
}

#[derive(Debug)]
enum OApiVersion {
    V2,
    V3,
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to OpenAPI spec in JSON/YAML format.
    #[structopt(parse(from_os_str))]
    spec: PathBuf,
    /// OpenAPI version (e.g., v2).
    #[structopt(long = "api", parse(try_from_str = "parse_version"))]
    api: OApiVersion,
    /// Output directory to write code (default: current working directory).
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn parse_args_and_run() -> Result<(), Error> {
    let opt = Opt::from_args();
    if let OApiVersion::V3 = opt.api {
        Err(PaperClipError::UnsupportedOpenAPIVersion)?;
    }

    let fd = File::open(&opt.spec)?;
    let raw: Api<DefaultSchema> = v2::from_reader(fd)?;
    let spec = raw.resolve()?;

    let mut state = EmitterState::default();
    state.crate_meta = Some(CrateMeta::default());
    if let Some(o) = opt.output {
        fs::create_dir_all(&o)?;
        state.working_dir = o;
    }

    let emitter = DefaultEmitter::from(state);
    emitter.generate(&spec)
}

fn main() {
    env_logger::init();
    if let Err(e) = parse_args_and_run() {
        error!("{}", e);
    }
}
