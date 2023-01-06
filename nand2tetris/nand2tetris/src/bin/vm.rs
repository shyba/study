use std::fs;
use std::fs::ReadDir;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use nand2tetris::assembler::{ComputeFields, ComputeOp, DestOp, Instruction, JumpOp};
use nand2tetris::assembler::generate_instruction;
use nand2tetris::vm::instructions::Parser;
use nand2tetris::vm::codegen::CodeGenerator;



fn write_boot(output_file: &mut std::fs::File) {
    println!("BOOT");
    let mut instructions = vec![];
    let values = vec![
        ("SP", 261),
        ("LCL", 261),
        ("ARG", 256),
        ("THIS", 10_002),
        ("THAT", 10_003),
    ];
    for (label, value) in values {
        instructions.extend(vec![
            Instruction::Address(value),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(false),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::D,
            }),
            Instruction::LabeledAddress(label.to_string()),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]);
    }

    instructions.extend(vec![
        Instruction::LabeledAddress("Sys.init".to_string()),
        Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::Zero,
            jump_op: JumpOp::Unconditional,
            destination_op: DestOp::Nothing,
        }),
    ]);
    for instruction in instructions {
        dbg!(&instruction);
        let string_instruction = generate_instruction(&instruction);
        dbg!(&string_instruction);
        writeln!(output_file, "{}", &string_instruction).unwrap();
    }
}

fn main() {
    if std::env::args().len() > 1 {
        for argument in std::env::args().skip(1) {
            process(&argument);
        }
    } else {
        for file_path in ValidFiles::new(None) {
            process_single_file(file_path);
        }
    }
}

fn process(argument: &String) {
    let path = Path::new(argument);
    match path.is_file() {
        true => process_single_file(path.to_path_buf()),
        false => process_directory(path),
    }
}

fn process_directory(path: &Path) {
    let directory_name = path.file_name().unwrap().to_str().unwrap();
    let mut translator = CodeGenerator::new(directory_name.to_string());
    let output_file_name = format!("{}.asm", directory_name);
    let output_file_path = path.join(Path::new(&output_file_name));
    let mut output_file =
        std::fs::File::create(output_file_path).expect("Error opening output file");
    write_boot(&mut output_file);

    for file_path in ValidFiles::new(Some(&path.to_str().unwrap().to_string())) {
        let file = std::fs::File::open(&file_path).expect("Error opening input file");
        translator.static_namespace = file_path.file_name().unwrap().to_str().unwrap().to_string();
        let mut parser = Parser::new();
        for line in std::io::BufReader::new(file).lines() {
            let line = &line.expect("IO error reading line.");
            let parsed_line = parser.parse_line(line);
            dbg!(&parsed_line);
            for instruction in translator.translate(&parsed_line) {
                dbg!(&instruction);
                let string_instruction = generate_instruction(&instruction);
                dbg!(&string_instruction);
                writeln!(output_file, "{} // {}", &string_instruction, &line).unwrap();
            }
        }
    }
}

fn process_single_file(file_path: PathBuf) {
    println!("Processing file: {:?}", file_path);
    let file = std::fs::File::open(&file_path).expect("Error opening input file");
    let mut parser = Parser::new();
    let file_name = file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    let mut translator = CodeGenerator::new(file_name.to_string());

    let output_file_path = file_path.to_str().unwrap().replace(".vm", ".asm");
    let mut output_file =
        std::fs::File::create(output_file_path).expect("Error opening output file");
    for line in std::io::BufReader::new(file).lines() {
        let parsed_line = parser.parse_line(&line.expect("IO error reading line."));
        dbg!(&parsed_line);
        for instruction in translator.translate(&parsed_line) {
            dbg!(&instruction);
            let string_instruction = generate_instruction(&instruction);
            dbg!(&string_instruction);
            writeln!(output_file, "{}", &string_instruction).unwrap();
        }
    }
}

enum ValidFiles<'a> {
    File(&'a Path),
    Directory(Option<ReadDir>),
    Done,
}

impl<'a> ValidFiles<'a> {
    pub fn new(dir: Option<&'a String>) -> Self {
        let path = match dir {
            Some(value) => Path::new(value),
            None => Path::new("."),
        };
        match path.is_file() {
            true => Self::File(path),
            false => Self::Directory(Some(fs::read_dir(path).unwrap())),
        }
    }
}

impl Iterator for ValidFiles<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::File(path) => {
                let current = path.to_path_buf();
                *self = Self::Done;
                Some(current)
            }
            Self::Done => None,
            Self::Directory(read_dir) => {
                while read_dir.is_some() {
                    match read_dir {
                        Some(entry) => match entry.next() {
                            Some(next_entry) => {
                                let path = next_entry.unwrap().path();
                                let valid_name = path
                                    .file_name()
                                    .and_then(|value| value.to_string_lossy().chars().next())
                                    .map_or(false, char::is_uppercase);
                                let valid_extension = path.extension().map_or(false, |extension| {
                                    extension.to_string_lossy().eq("vm")
                                });
                                if path.is_dir() || !valid_name || !valid_extension {
                                    continue;
                                }
                                return Some(path);
                            }
                            None => break,
                        },
                        None => break,
                    }
                }
                *self = Self::Done;
                None
            }
        }
    }
}

