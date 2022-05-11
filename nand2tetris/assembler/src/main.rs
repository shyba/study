extern crate core;

use std::{env, fs, io};
use std::path::{Path, PathBuf};

fn main() {
    let files = match env::args().len() {
        x if x == 1 => list_asm_files(),
        _ => args_to_files(),
    };
    if files.expect("Error reading directory/files.").len() == 0 {
        println!("Nothing to assemble!");
        return;
    }
}

fn list_asm_files() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(Path::new("."))? {
        let entry = entry?;
        match entry.path() {
            p if p.ends_with("asm") => files.push(entry.path()),
            _ => continue
        }
    }
    Ok(files)
}

fn args_to_files() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for name in env::args().skip(1) {
        match Path::new(&name) {
            p if p.exists() && !p.is_dir() => files.push(p.to_path_buf()),
            _ => continue
        }
    }
    Ok(files)
}