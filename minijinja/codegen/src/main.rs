use std::{fs, path::Path};

use minijinja::{Environment, render};

fn main() {
    // TODO: allow passing different configurations: arc, array, bip_arc, ...
    let env = Environment::new();

    // TODO: scan templates directory

    fs::write(
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("../src/arc/mod.rs"),
        render!(
            in env,
            // TODO: use runtime dependencies?
            include_str!("../templates/src/mod.rs"),
            pow2 => false
        ),
    )
    .unwrap();
}
