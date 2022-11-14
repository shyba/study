extern crate core;

use nand2tetris::assembler;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::collections::hash_map::Entry;
use std::io::{BufRead, Write};

fn main() {
    let files = match env::args().len() {
        x if x == 1 => list_asm_files(),
        _ => args_to_files(),
    };
    let files = files.expect("Error reading directory/files.");
    if files.is_empty() {
        println!("Nothing to assemble!");
        return;
    }
    process_file(files).unwrap();
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

fn process_file(files: Vec<PathBuf>) -> io::Result<()> {
    for file in files {
        let instructions = parse_file(file.clone()).unwrap();
        let mut first_pass: Vec<assembler::Instruction> = vec![];
        let mut label_table = std::collections::HashMap::new();
        for instruction in instructions {
            match instruction {
                assembler::Instruction::Address(_) => first_pass.push(instruction),
                assembler::Instruction::LabeledAddress(_) => first_pass.push(instruction),
                assembler::Instruction::Compute(_) => first_pass.push(instruction),
                assembler::Instruction::Label(name) => {label_table.insert(name, first_pass.len() as u16);},
                _ => ()
            }
        }
        let mut output_path = file;
        output_path.set_extension("hack");
        let mut output_file = io::BufWriter::new(fs::File::create(output_path)?);
        let mut varible_symbol_slot = 16..;
        for instruction in first_pass {
            let assemble = match instruction {
                assembler::Instruction::LabeledAddress(name) => {
		    if let Entry::Vacant(entry) = label_table.entry(name.clone()) {
                        let value = varible_symbol_slot.next().unwrap() as u16;
                        entry.insert(value);
                        assembler::assemble_address(&value)
                    } else {
                        let value = label_table.get(&name).unwrap();
                        assembler::assemble_address(value)
                    }
                }
                _ => assembler::assemble(&instruction)
            };
            writeln!(&mut output_file, "{}", assemble)?;
        }
    }
    Ok(())
}

fn parse_file(path: PathBuf) -> io::Result<Vec<assembler::Instruction>> {
    let mut instructions = vec![];
    let file = fs::File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let instruction = assembler::parse(&line).unwrap();
        instructions.push(instruction);
    }
    Ok(instructions)
}
