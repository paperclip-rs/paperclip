use heck::ToSnakeCase;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};
use structopt::StructOpt;

mod error;
use error::Error;

/// Deserialize the schema from the given reader. Currently, this only supports
/// JSON and YAML formats.
fn from_reader_v3<R>(mut reader: R) -> Result<openapiv3::OpenAPI, Error>
where
    R: Read,
{
    let mut buf = [b' '];
    while buf[0].is_ascii_whitespace() {
        reader.read_exact(&mut buf)?;
    }
    let reader = buf.as_ref().chain(reader);

    Ok(if buf[0] == b'{' {
        serde_json::from_reader::<_, openapiv3::OpenAPI>(reader)?
    } else {
        serde_yaml::from_reader::<_, openapiv3::OpenAPI>(reader)?
    })
}
fn parse_spec_v3(s: &str) -> Result<openapiv3::OpenAPI, Error> {
    let fd = File::open(s)?;
    from_reader_v3(fd)
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to OpenAPI spec in JSON/YAML format (also supports publicly accessible URLs).
    #[structopt(long)]
    spec: std::path::PathBuf,
    /// Output directory to write code (default: current working directory).
    #[structopt(short = "o", long = "out", parse(from_os_str))]
    output: Option<PathBuf>,
    /// Don't Render models.
    #[structopt(long)]
    no_models: bool,
    /// Don't Render operations.
    #[structopt(long)]
    no_ops: bool,
    /// Name of the crate. If this is not specified, then the name of the
    /// working directory is assumed to be crate name.
    #[structopt(long = "name")]
    pub name: Option<String>,
    /// The Version of the crate.
    #[structopt(long = "version", default_value = "0.1.0")]
    pub version: String,
    /// The Edition of the crate.
    #[structopt(long = "edition", default_value = "2018")]
    pub edition: String,
}

fn parse_args_and_run() -> Result<(), Error> {
    let opt: Opt = Opt::from_args();

    if let Some(o) = &opt.output {
        fs::create_dir_all(o)?;
    }

    let spec = parse_spec_v3(opt.spec.to_string_lossy().as_ref())?;
    let name = opt.name.map(Ok::<String, Error>).unwrap_or_else(|| {
        Ok(fs::canonicalize(std::path::Path::new("."))?
            .file_name()
            .ok_or(Error::InvalidCodegenDirectory)?
            .to_string_lossy()
            .into_owned()
            .to_snake_case())
    })?;
    let info = paperclip_codegen::v3_03::PackageInfo {
        libname: name.to_snake_case(),
        name,
        version: opt.version,
        edition: opt.edition,
    };

    paperclip_codegen::v3_03::OpenApiV3::new(spec, opt.output, info)
        .run(!opt.no_models, !opt.no_ops)?;
    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(e) = parse_args_and_run() {
        eprintln!("{}", e);
    }
}
