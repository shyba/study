mod assembler;

extern crate core;

use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::io::BufRead;

fn main() {
    let files = match env::args().len() {
        x if x == 1 => list_asm_files(),
        _ => args_to_files(),
    };
    let files = files.expect("Error reading directory/files.");
    if files.len() == 0 {
        println!("Nothing to assemble!");
        return;
    }
    for file in files {
        parse_file(file).expect("Error parsing file.");
    }
}

fn list_asm_files() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(Path::new("."))? {
        let entry = entry?;
        match entry.path() {
            p if p.ends_with("asm") => files.push(entry.path()),
            _ => continue,
        }
    }
    Ok(files)
}

fn args_to_files() -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for name in env::args().skip(1) {
        match Path::new(&name) {
            p if p.exists() && !p.is_dir() => {
                files.push(name.into())
            },
            _ => continue,
        }
    }
    Ok(files)
}

fn parse_file(path: PathBuf) -> io::Result<()> {
    let file = fs::File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        println!("{}", line);
        println!("{:?}", assembler::parse(&line));
        println!("---")
    }
    Ok(())
}
