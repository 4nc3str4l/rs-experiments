mod cache;

use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use strsim::jaro_winkler;
use walkdir::WalkDir;
use dirs_next::data_local_dir;

#[cfg(windows)]
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use crate::cache::Cache;


fn main() {

    let mut cache: Cache = Cache::new().unwrap();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("Usage: {} <folder>", args[0]);
        return;
    }
    let target_dir = args[1].to_lowercase();
    let desktop = get_desktop_folder();

    if let Some(c) =  cache.get(&target_dir) {
        println!("Found in cache: {}", c);
        open_vs_code_folder(  &c  );
        return;
    }

    println!("Searching for {} in {}...", target_dir, desktop);

    let possible_dirs: Arc<Mutex<Vec<(String, f64)>>> = Arc::new(Mutex::new(Vec::new()));
    let found_dir = Arc::new(Mutex::new(None));

    WalkDir::new(desktop)
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .par_iter()
        .find_first(|dir| {
            if dir.file_type().is_dir() {
                let file_name = dir.file_name().to_str().unwrap().to_lowercase();
                let distance = jaro_winkler(&file_name, &target_dir);
                if (distance - 1.0).abs() < f64::EPSILON {
                    let mut found_dir = found_dir.lock().unwrap();
                    *found_dir = Some(dir.path().to_path_buf());
                    return true;
                } else {
                    let mut possible_dirs = possible_dirs.lock().unwrap();
                    possible_dirs.push((dir.path().display().to_string(), distance));
                }
            }
            false
        });

    let found_dir_locked = found_dir.lock().unwrap();
    if let Some(ref found_dir) = *found_dir_locked {
        open_vs_code_folder(found_dir.to_str().unwrap());
        let _= cache.store(&target_dir, found_dir.to_str().unwrap());
    } else {
        drop(found_dir_locked);
        println!("Folder not found!");
        let mut possible_dirs = possible_dirs.lock().unwrap();
        if !possible_dirs.is_empty() {
            println!("Did you mean:");
            possible_dirs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            for (dir, _) in possible_dirs.iter().rev().take(20) {
                println!("{}", dir);
            }
        }
    }
}


fn open_vs_code_folder(found_dir: &str) {
    println!("{}", found_dir);
    let mut cmd = std::process::Command::new(get_vscode_location().unwrap().to_str().unwrap());
    cmd.arg(found_dir);
    cmd.spawn().expect("Failed to open vscode");
    println!("Done!");
}

fn get_vscode_location() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        let mut vscode_path = data_local_dir()?;

        vscode_path.push("Programs");
        vscode_path.push("Microsoft VS Code");
        vscode_path.push("Code.exe");

        if vscode_path.exists() {
            return Some(vscode_path);
        }
    }

    #[cfg(not(windows))]
    {
        let vscode_path = PathBuf::from("/usr/bin/code");
        if vscode_path.exists() {
            return Some(vscode_path);
        }
    }

    None
}

#[cfg(windows)]
fn get_desktop_folder() -> String {
    let mut key = RegKey::predef(HKEY_CURRENT_USER);
    key = key
        .open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders")
        .unwrap();
    let desktop: String = key.get_value("Desktop").unwrap();
    desktop
}

#[cfg(not(windows))]
fn get_desktop_folder() -> String {
    let mut home_dir = dirs_next::home_dir().unwrap();
    home_dir.to_str().unwrap().to_owned()
}