use std::env;

use walkdir::WalkDir;
use winreg::{RegKey, enums::HKEY_CURRENT_USER};

const VSCODE_LOCATION: &str = "C:\\Users\\cmuri\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe";

fn get_desktop_folder() -> String {
    let mut key = RegKey::predef(HKEY_CURRENT_USER);
    key = key.open_subkey(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders").unwrap();
    let desktop: String = key.get_value("Desktop").unwrap();
    desktop
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("Usage: {} <folder>", args[0]);
        return;
    }
    let taget_dir = &args[1];
    let desktop = get_desktop_folder();

    for dir in WalkDir::new(desktop).into_iter().filter_map(|e| e.ok()) {
        if dir.file_type().is_dir() {
            if dir.file_name().to_str().unwrap() == taget_dir {
                println!("{}", dir.path().display());
                let mut cmd = std::process::Command::new(VSCODE_LOCATION);
                cmd.arg(dir.path());
                cmd.spawn().expect("Failed to open vscode");
            }
        }
    }
}
