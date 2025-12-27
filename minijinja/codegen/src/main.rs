use std::{fs, path::Path};

use minijinja::{Environment, render};

fn main() {
    let env = Environment::new();

    // render the template and write it into the file that main.rs includes.
    fs::write(
        Path::new(&std::env::var("OUT_DIR").unwrap()).join("example.rs"),
        render!(
            in env,
            // TODO: use runtime dependencies?
            include_str!("../templates/src/arc.rs"),
            build_cwd => std::env::current_dir().unwrap()
        ),
    )
    .unwrap();
}
