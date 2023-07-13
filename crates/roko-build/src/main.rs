use std::{
    path::PathBuf,
    process::Command,
    sync::{atomic::AtomicBool, mpsc, Arc},
};

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
            println!("[INFO]: Finished building Roko application");
            is_building.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    } else if let Some("build") = arg.as_deref() {
        println!("[INFO]: Building Roko application");
        println!("[INFO]: Building Rust");
        spawn_web_pack(&[]);
        println!("[INFO]: Building CSS");
        println!("[INFO]: Finished building Roko application");
    } else {
        eprintln!("Please choose between 'build' and 'watch'");
        std::process::exit(1);
    }
}

fn spawn_web_pack(extra_args: &[&str]) {
    let mut args: Vec<&str> = vec!["--log-level", "error", "build", "--out-dir", "static"];

    args.extend(extra_args);

    Command::new("wasm-pack").args(args).status().unwrap();
}
