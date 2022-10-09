use std::fs;
use std::fs::ReadDir;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use assembler::assembler::{ComputeFields, ComputeOp, DestOp, Instruction, JumpOp};

pub enum Segment {
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

pub enum Arithmetic {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

pub enum VMInstruction {
    Comment(String),
    Push(Segment, u16),
    Pop(Segment, u16),
    Arithmetic,
    Label(String),
    Goto, //?
    Branch,
    Function,
    Return,
    Call,
}

pub enum Address {
    SP,
    LCL,
    ARG,
    THIS,
    THAT,
    TEMP0,
    TEMP1,
    TEMP2,
    TEMP3,
    TEMP4,
    TEMP5,
    TEMP6,
    TEMP7,
    R13,
    R14,
    R15,
    Symbol(u8),
}

fn parse_push(line: String) -> VMInstruction {
    let words: Vec<&str> = line.split(" ").collect();
    if words.len() != 3 {
        panic!("Error parsing push. Expected: push <segment> <value>")
    }
    VMInstruction::Push(
        parse_segment(words[1]),
        u16::from_str_radix(words[2], 10).expect("Error parsing push index"),
    )
}

fn parse_segment(segment: &str) -> Segment {
    match segment {
        "local" => Segment::Local,
        "static" => Segment::Static,
        "this" => Segment::This,
        "that" => Segment::That,
        "temp" => Segment::Temp, // double check
        "constant" => Segment::Constant,
        unexpected => panic!("Unexpected segment: {}", unexpected),
    }
}

fn gen_load(addr: u16) -> Vec<Instruction> {
    let mut result = vec![];
    result.push(Instruction::Address(addr));
    result.push(Instruction::Compute(ComputeFields {
        compute_op: ComputeOp::A(true),
        jump_op: JumpOp::Nothing,
        destination_op: DestOp::D,
    }));
    result
}

// move this to assembler crate?
fn generate_instruction(ins: Instruction) -> String {
    match ins {
        Instruction::Address(value) => format!("@{}", value),
        Instruction::LabeledAddress(value) => format!("@{}", value),
        Instruction::Comment(content) => format!("//{}", content),
        Instruction::Compute(fields) => generate_compute_instruction(fields),
        Instruction::Label(label) => format!("{}:", label),
        Instruction::Nothing => String::new(),
    }
}

fn generate_compute_instruction(fields: ComputeFields) -> String {
    let mut result = String::new();
    result.push_str(match fields.compute_op {
        ComputeOp::Zero => "0",
        ComputeOp::One => "1",
        ComputeOp::MinusOne => "-1",
        ComputeOp::D => "D",
        ComputeOp::A(true) => "M",
        ComputeOp::A(false) => "A",
        ComputeOp::NotD => "!D",
        ComputeOp::NotA(true) => "!M",
        ComputeOp::NotA(false) => "!A",
        ComputeOp::MinusD => "-D",
        ComputeOp::MinusA(true) => "-M",
        ComputeOp::MinusA(false) => "-A",
        ComputeOp::IncD => "D+1",
        ComputeOp::IncA(true) => "M+1",
        ComputeOp::IncA(false) => "A+1",
        ComputeOp::DecD => "D-1",
        ComputeOp::DecA(true) => "M-1",
        ComputeOp::DecA(false) => "A-1",
        ComputeOp::DPlusA(true) => "D+M",
        ComputeOp::DPlusA(false) => "D+A",
        ComputeOp::DMinusA(true) => "D-M",
        ComputeOp::DMinusA(false) => "D-A",
        ComputeOp::AMinusD(true) => "M-D",
        ComputeOp::AMinusD(false) => "A-D",
        ComputeOp::DAndA(true) => "D&M",
        ComputeOp::DAndA(false) => "D&A",
        ComputeOp::DOrA(true) => "D|M",
        ComputeOp::DOrA(false) => "D|A",
    });
    result.push_str(match fields.destination_op {
        DestOp::M => "M",
        DestOp::D => "D",
        DestOp::A => "A",
        DestOp::DM => "DM",
        DestOp::AM => "AM",
        DestOp::AD => "AD",
        DestOp::ADM => "ADM",
        DestOp::Nothing => "",
    });
    if fields.jump_op == JumpOp::Nothing {
        return result;
    } else if result.len() > 0 {
        result.push(';');
    }
    result.push_str(match fields.jump_op {
        JumpOp::Greater => "JGT",
        JumpOp::Equal => "JEQ",
        JumpOp::GreaterEqual => "JGE",
        JumpOp::Lower => "JLT",
        JumpOp::NotEqual => "JNE",
        JumpOp::LessEqual => "JLE",
        JumpOp::Unconditional => "JMP",
        JumpOp::Nothing => "",
    });
    result
}

struct Parser {
    line_number: u16,
}

impl Parser {
    pub fn new() -> Self {
        Self { line_number: 0 }
    }
    pub fn parse_line(&mut self, line: &String) -> VMInstruction {
        self.line_number += 1;
        let lower_line = line.to_lowercase();

        if line.contains("//") {
            VMInstruction::Comment(line.to_string()) // todo: handle comments after instructions
        } else if lower_line.starts_with("push") {
            parse_push(line.to_lowercase())
        } else {
            panic!("Unexpected instruction: {}", line)
        }
    }
}

fn main() {
    if std::env::args().len() > 1 {
        for argument in std::env::args().skip(1) {
            for file_path in ValidFiles::new(Some(&argument)) {
                process_file(file_path);
            }
        }
    } else {
        for file_path in ValidFiles::new(None) {
            process_file(file_path);
        }
    }
}

fn process_file(file_path: PathBuf) {
    println!("Processing file: {:?}", file_path);
    let file = std::fs::File::open(file_path).expect("Error opening file");
    let mut parser = Parser::new();
    for line in std::io::BufReader::new(file).lines() {
        let parsed_line = parser.parse_line(&line.expect("IO error reading line."));
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
            true => Self::File(&path),
            false => Self::Directory(Some(fs::read_dir(path).unwrap())),
        }
    }
}

impl Iterator for ValidFiles<'_> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::File(path) => {
                let current = path.clone().to_path_buf();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_load_instructions() {
        // load 16 to A register
        assert_eq!(
            vec!(
                Instruction::Address(16),
                Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(true),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::D
                })
            ),
            gen_load(16)
        );
    }
}
