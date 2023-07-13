use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
    sync::{atomic::AtomicBool, mpsc, Arc},
};

use roko_macro::style_folder;

use notify::{event::EventKind, Event, RecursiveMode, Watcher};

fn main() {
    if !PathBuf::from("Cargo.toml").exists() {
        println!("No Cargo.toml found");
        std::process::exit(1);
    }

    let (sender, recv) = mpsc::channel();

    let arg = std::env::args().nth(1);

    if let Some("watch") = arg.as_deref() {
        println!("[INFO]: Watching");

        let is_building = Arc::new(AtomicBool::new(false));

        let clonde = is_building.clone();

        let mut watcher = notify::recommended_watcher(move |res| match res {
            Ok(Event {
                kind: EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_),
                ..
            }) => {
                if !clonde.load(std::sync::atomic::Ordering::SeqCst) {
                    sender.send(()).unwrap();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
            _ => (),
        })
        .unwrap();

        let mut folder = std::env::current_dir().unwrap();
        folder.push("src");

        watcher.watch(&folder, RecursiveMode::Recursive).unwrap();

        for () in recv {
            is_building.store(true, std::sync::atomic::Ordering::SeqCst);
            println!("[INFO]: Building Roko application");
            println!("[INFO]: Building Rust");
            spawn_web_pack(&["--dev", "--target", "web"]);
            println!("[INFO]: Building CSS");
            build_css();
            println!("[INFO]: Finished building Roko application");
            is_building.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    } else if let Some("build") = arg.as_deref() {
        println!("[INFO]: Building Roko application");
        println!("[INFO]: Building Rust");
        spawn_web_pack(&[]);
        println!("[INFO]: Building CSS");
        build_css();
        println!("[INFO]: Finished building Roko application");
    } else {
        eprintln!("Please choose between 'build' and 'watch'");
        std::process::exit(1);
    }
}

fn build_css() {
    let path = style_folder!();

    let mut project_root = std::env::current_dir().unwrap();

    project_root.push("static");

    if !project_root.exists() {
        std::fs::create_dir(&project_root).unwrap();
    }

    let mut file = File::create(project_root.join("output.css")).unwrap();

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let css_file = fs::read(path).unwrap();

        file.write_all(&css_file).unwrap();
    }
}

fn spawn_web_pack(extra_args: &[&str]) {
    let mut args: Vec<&str> = vec!["--log-level", "error", "build", "--out-dir", "static"];

    args.extend(extra_args);

    Command::new("wasm-pack").args(args).status().unwrap();
}
