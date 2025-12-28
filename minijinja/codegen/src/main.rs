use std::{collections::BTreeMap, fs, path::PathBuf};

use minijinja::{Environment, Value};
use serde::Deserialize;

// TODO: --watch: only run on template file changes

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: allow passing different configurations: arc, array, bip_arc, ...
    // TODO: if not given, scan ../configs/ dir.

    let codegen_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let parent_dir = codegen_dir.join("..");
    let config_path = codegen_dir.join("configs/arc.toml");
    let contents = fs::read(config_path)?;
    let contents = String::from_utf8(contents)?;
    let data: Value = toml::from_str(&contents)?;
    let ctx: BTreeMap<String, Value> = Deserialize::deserialize(data)?;
    let mut env = Environment::new();

    // TODO: scan templates directory

    let source = fs::read_to_string(codegen_dir.join("templates/src/mod.rs"))?;
    let name = "mod.rs";
    env.add_template(name, &source)?;
    // TODO: use env.set_loader(path_loader("templates/src"))?
    let tmpl = env.get_template(name).unwrap();
    let rendered = tmpl.render(ctx)?;
    fs::write(parent_dir.join("src/arc/mod.rs"), rendered)?;
    Ok(())
}
