use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use minijinja::{Environment, Value, context, path_loader, value::merge_maps};

// TODO: --watch: only run on template file changes

// TODO: -h/--help: rudimentary help text

fn main() {
    // TODO: simple CLI parsing with std::env::args()

    let codegen_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let template_dir = codegen_dir.join("templates");
    let parent_dir = codegen_dir.join("..");
    let config_dir = codegen_dir.join("configs");
    let mut contexts = vec![];
    for entry in fs::read_dir(config_dir).unwrap() {
        let config_path = entry.unwrap().path();
        if config_path.extension().and_then(OsStr::to_str) != Some("toml") {
            continue;
        }
        let config_name = config_path
            .file_stem()
            .expect("invalid config name")
            .to_owned()
            .into_string()
            .unwrap();
        let contents = fs::read(&config_path).unwrap();
        let contents = String::from_utf8(contents).unwrap();
        let ctx = toml::from_str(&contents).unwrap();
        contexts.push((config_name, ctx));
    }
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
        )
        .unwrap();
        watcher.watch(&template_dir, Recursive).unwrap();
        println!("Watching template files for changes, press Ctrl-C to cancel.");
        loop {
            // Duplicate paths are removed, order doesn't matter.
            for path in HashSet::<PathBuf>::from_iter(rx.read_chunk(rx.slots()).unwrap()) {
                let path = path.strip_prefix(&template_dir).unwrap();
                if path.extension().and_then(OsStr::to_str) != Some("rs") {
                    continue;
                }
                render(&parent_dir, path, &contexts);
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    } else {
        // TODO: walk templates/**/*.rs
        render(&parent_dir, Path::new("src/mod.rs"), &contexts);
    }
}

fn render(dir: &Path, name: &Path, contexts: &[(String, Value)]) {
    let mut env = Environment::empty();
    env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
    env.set_trim_blocks(true);
    //env.set_lstrip_blocks(true);
    env.set_loader(path_loader(dir.join("codegen/templates")));
    let tmpl = env
        .get_template(name.to_str().expect("invalid template name"))
        .unwrap();
    let mut iter = name.iter().map(OsStr::to_str).map(Option::unwrap);
    let subdir = iter.next().unwrap();
    assert!(["src", "tests"].contains(&subdir));
    let rest = PathBuf::from_iter(iter);
    for (name, ctx) in contexts {
        let ctx = merge_maps([context! { module => format!("rtrb::{name}") }, ctx.clone()]);
        let rendered = tmpl.render(ctx).unwrap();
        let path = dir.join(subdir).join(name).join(&rest);
        fs::write(&path, rendered)
            .unwrap_or_else(|err| panic!("unable to write {:?}: {}", &path, err));
    }
    println!("Rendered {name:?}.");
}
