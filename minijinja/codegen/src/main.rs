use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use minijinja::{Environment, Value, path_loader};

// TODO: --watch: only run on template file changes

// TODO: -h/--help: rudimentary help text

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: simple CLI parsing with std::env::args()

    // TODO: get configs from arguments or scan config dir

    let codegen_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let parent_dir = codegen_dir.join("..");
    let config_path = codegen_dir.join("configs/arc.toml");
    let contents = fs::read(config_path)?;
    let contents = String::from_utf8(contents)?;
    let ctx = toml::from_str(&contents)?;
    // TODO: create list of contexts
    let contexts = vec![ctx];
    let template_dir = codegen_dir.join("templates");

    let watch = true;
    if watch {
        use notify::{
            Config, Event, EventKind::Modify, RecommendedWatcher, RecursiveMode::Recursive,
            Watcher as _,
        };
        let (mut tx, mut rx) = rtrb::RingBuffer::new(128);
        let mut watcher = RecommendedWatcher::new(
            move |res| match res {
                Ok(Event {
                    kind: Modify(_),
                    paths,
                    ..
                }) => {
                    for path in paths {
                        tx.push(path).expect("queue too small");
                    }
                }
                Ok(_) => {}
                Err(error) => eprintln!("Error from notify: {error:?}"),
            },
            Config::default(),
        )?;
        watcher.watch(&template_dir, Recursive)?;
        println!("Watching template files for changes, press Ctrl-C to cancel.");
        loop {
            // Duplicate paths are removed, order doesn't matter.
            for path in HashSet::<PathBuf>::from_iter(rx.read_chunk(rx.slots()).unwrap()) {
                let path = path.strip_prefix(&template_dir)?;
                if path.extension().and_then(OsStr::to_str) != Some("rs") {
                    continue;
                }
                render(&parent_dir, path, &contexts)?;
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    } else {
        // TODO: walk templates/**/*.rs
        render(&parent_dir, Path::new("src/mod.rs"), &contexts)
    }
}

fn render(dir: &Path, name: &Path, contexts: &[Value]) -> Result<(), Box<dyn std::error::Error>> {
    let mut env = Environment::empty();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    env.set_trim_blocks(true);
    //env.set_lstrip_blocks(true);
    env.set_loader(path_loader(dir.join("codegen/templates")));
    let tmpl = env.get_template(name.to_str().expect("invalid template name"))?;
    let mut iter = name.iter().map(OsStr::to_str).map(Option::unwrap);
    let subdir = iter.next().unwrap();
    assert!(["src", "tests"].contains(&subdir));
    let rest = PathBuf::from_iter(iter);
    for ctx in contexts {
        let rendered = tmpl.render(ctx)?;
        // TODO: get name of context (e.g. "arc") == module name
        // TODO: add module name to context? or have it be part of the config?
        fs::write(dir.join(subdir).join("arc").join(&rest), rendered)?;
    }
    println!("Rendered {name:?}.");
    Ok(())
}
