use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

fn load_file(p: PathBuf) -> String {
    let mut string = String::new();
    let mut fd = File::open(p).unwrap();
    fd.read_to_string(&mut string).unwrap();
    string
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let files = &[
        ("CARGO_MANIFEST", "src/build/manifest.hbs"),
        ("CLIENT_MOD", "src/build/client_mod.hbs"),
        ("CLAP_YAML", "src/build/clap_yaml.hbs"),
        ("CLI_MAIN", "src/build/cli_main.hbs"),
        ("UTIL_MOD", "src/build/util_mod.hbs"),
        ("CLI_UTIL", "src/build/cli_util.hbs"),
    ];

    let mut contents = String::from(
        "
#[cfg(feature = \"codegen\")]
mod template {
    use tinytemplate::TinyTemplate;

    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code, non_camel_case_types)]
    pub enum TEMPLATE {",
    );

    for (name, _) in files {
        contents.push_str(
            "
        ",
        );
        contents.push_str(name);
        contents.push(',');
    }

    contents.push_str(
        "
    }",
    );

    let source_path = PathBuf::from(&out_dir).join("template.rs");
    for (name, file) in files {
        println!("cargo:rerun-if-changed={}", file);
        let thing = load_file(root.join(&file));
        contents.push_str(&format!(
            "
    pub const {}: &str = {:?};
",
            name, thing
        ));
    }

    contents.push_str(
        "
    pub fn render<C>(t: TEMPLATE, context: &C) -> tinytemplate::error::Result<String>
        where C: serde::Serialize
    {
        let mut temp = TinyTemplate::new();
        temp.add_template(\"file\", match t {",
    );

    for (name, _) in files {
        contents.push_str(&format!(
            "
            TEMPLATE::{name} => {name},",
            name = name
        ));
    }

    contents.push_str(
        "
        })?;

        temp.render(\"file\", context)
    }
}
",
    );

    let mut fd = File::create(&source_path).unwrap();
    fd.write_all(contents.as_bytes()).unwrap();
}
