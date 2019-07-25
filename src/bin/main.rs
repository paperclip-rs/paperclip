use failure::Error;
use paperclip::v2::{
    self,
    codegen::{CrateMeta, DefaultEmitter, Emitter, EmitterState},
    models::{Api, DefaultSchema},
};
use paperclip::PaperClipError;
use reqwest::Client;
use structopt::StructOpt;
use url::Url;

use std::fs::{self, File};
use std::io::Cursor;
use std::path::PathBuf;

fn parse_version(s: &str) -> Result<OApiVersion, Error> {
    Ok(match s {
        "v2" => OApiVersion::V2,
        "v3" => OApiVersion::V3,
        _ => Err(PaperClipError::UnsupportedOpenAPIVersion)?,
    })
}

fn parse_spec(s: &str) -> Result<Api<DefaultSchema>, Error> {
    if let Ok(u) = Url::parse(s) {
        let mut bytes = vec![];
        let client = Client::new();
        let mut resp = client.get(u).send()?;
        resp.copy_to(&mut bytes)?;
        Ok(v2::from_reader(Cursor::new(bytes))?)
    } else {
        let fd = File::open(s)?;
        Ok(v2::from_reader(fd)?)
    }
}

#[derive(Debug)]
enum OApiVersion {
    V2,
    V3,
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to OpenAPI spec in JSON/YAML format (also supports publicly accessible URLs).
    #[structopt(parse(try_from_str = "parse_spec"))]
    spec: Api<DefaultSchema>,
    /// OpenAPI version (e.g., v2).
    #[structopt(long = "api", parse(try_from_str = "parse_version"))]
    api: OApiVersion,
    /// Output directory to write code (default: current working directory).
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Emit CLI target instead.
    #[structopt(long = "cli")]
    cli: bool,
}

fn parse_args_and_run() -> Result<(), Error> {
    let opt = Opt::from_args();
    if let OApiVersion::V3 = opt.api {
        Err(PaperClipError::UnsupportedOpenAPIVersion)?;
    }

    let spec = opt.spec.resolve()?;
    let mut state = EmitterState::default();

    if let Some(o) = opt.output {
        fs::create_dir_all(&o)?;
        state.working_dir = o;
    }

    let mut meta = CrateMeta::default();
    if opt.cli {
        meta.is_cli = true;
    }

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
