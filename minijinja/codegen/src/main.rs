use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};

use minijinja::{Environment, Value};
use serde::Deserialize;

type Context = BTreeMap<String, Value>;

// TODO: --watch: only run on template file changes

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TODO: simple CLI parsing with std::env::args()

    // TODO: get configs from arguments or scan config dir

    // TODO: scan all templates and either
    // (1) add them to the watch list
    // or
    // (2) render them with all selected configs

    let codegen_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let parent_dir = codegen_dir.join("..");
    let config_path = codegen_dir.join("configs/arc.toml");
    let contents = fs::read(config_path)?;
    let contents = String::from_utf8(contents)?;
    let data: Value = toml::from_str(&contents)?;
    let ctx: Context = Deserialize::deserialize(data)?;

    // TODO: create list of contexts
    let contexts = vec![ctx];

    // TODO: differentiate between "src" and "test" (an maybe others?)

    let template_dir = codegen_dir.join("templates");
    // TODO: only pass parent_dir, use that to find template dir? or use static template dir?
    render(&template_dir, "src", "mod.rs", &contexts, &parent_dir)
}

fn render(template_dir: &Path, subdir: &str, template: &str, contexts: &[Context], parent_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    let source = fs::read_to_string(template_dir.join(subdir).join(template))?;
    env.add_template(template, &source)?;
    // TODO: use env.set_loader(path_loader("templates/src"))?
    let tmpl = env.get_template(template).unwrap();
    // TODO: for ctx in contexts ...
    for ctx in contexts {
        let rendered = tmpl.render(ctx)?;
        // TODO: get name of context (e.g. "arc")
        fs::write(parent_dir.join(subdir).join("arc").join(template), rendered)?;
    }
    Ok(())
}
