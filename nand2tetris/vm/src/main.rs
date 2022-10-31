use std::fs;
use std::fs::ReadDir;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use assembler::assembler::{ComputeFields, ComputeOp, DestOp, Instruction, JumpOp};

#[derive(Debug)]
pub enum Segment {
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum VMInstruction {
    Comment(String),
    Push(Segment, u16),
    Pop(Segment, u16),
    Arithmetic(Arithmetic),
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

fn try_parse_arithmetic(line: &String) -> Option<VMInstruction> {
    match line.trim().to_lowercase().as_str() {
        "add" => Some(VMInstruction::Arithmetic(Arithmetic::Add)),
        "sub" => Some(VMInstruction::Arithmetic(Arithmetic::Sub)),
        "neg" => Some(VMInstruction::Arithmetic(Arithmetic::Neg)),
        "eq" => Some(VMInstruction::Arithmetic(Arithmetic::Eq)),
        "gt" => Some(VMInstruction::Arithmetic(Arithmetic::Gt)),
        "lt" => Some(VMInstruction::Arithmetic(Arithmetic::Lt)),
        "and" => Some(VMInstruction::Arithmetic(Arithmetic::And)),
        "or" => Some(VMInstruction::Arithmetic(Arithmetic::Or)),
        "not" => Some(VMInstruction::Arithmetic(Arithmetic::Not)),
        _ => None,
    }
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

fn parse_pop(line: String) -> VMInstruction {
    let words: Vec<&str> = line.split(" ").collect();
    if words.len() != 3 {
        panic!("Error parsing pop. Expected: pop <segment> <value>")
    }
    VMInstruction::Pop(
        parse_segment(words[1]),
        u16::from_str_radix(words[2], 10).expect("Error parsing pop index"),
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

// move this to assembler crate?
fn generate_instruction(ins: &Instruction) -> String {
    match ins {
        Instruction::Address(value) => format!("@{}", value),
        Instruction::LabeledAddress(value) => format!("@{}", value),
        Instruction::Comment(content) => format!("//{}", content),
        Instruction::Compute(fields) => generate_compute_instruction(fields),
        Instruction::Label(label) => format!("{}:", label),
        Instruction::Nothing => String::new(),
    }
}

fn generate_compute_instruction(fields: &ComputeFields) -> String {
    let mut result = String::new();
    result.push_str(match fields.destination_op {
        DestOp::M => "M=",
        DestOp::D => "D=",
        DestOp::A => "A=",
        DestOp::DM => "DM=",
        DestOp::AM => "AM=",
        DestOp::AD => "AD=",
        DestOp::ADM => "ADM=",
        DestOp::Nothing => "",
    });
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

struct CodeGenerator {
    static_variables: u16,
    program_name: String
}

impl CodeGenerator {

    pub fn new(program_name: String) -> CodeGenerator {
        CodeGenerator { static_variables: 0, program_name: program_name }
    }

    pub fn translate(&mut self, vm_instruction: &VMInstruction) -> Vec<Instruction> {
        let mut instructions = vec![];
        match &vm_instruction {
            VMInstruction::Push(segment, value) => {
                match segment {
                    Segment::Constant => {
                        // load constant
                        instructions.push(self.segment_to_address_instruction(segment, *value));
                        // D=A
                        instructions.push(Instruction::Compute(ComputeFields {compute_op: ComputeOp::A(false), jump_op: JumpOp::Nothing, destination_op: DestOp::D}));
                        instructions.extend(self.gen_push_from_d()) // push D
                    },
                    _ => {
                        // all other cases are loading from memory segments
                        instructions.push(self.segment_to_address_instruction(segment, *value));
                        // D=M
                        instructions.push(Instruction::Compute(ComputeFields {compute_op: ComputeOp::A(true), jump_op: JumpOp::Nothing, destination_op: DestOp::D}));
                        instructions.extend(self.gen_push_from_d()) // push D
                    }
                }

            },
            _ => ()
        }
        instructions
    }

    fn segment_to_address_instruction(&mut self, segment: &Segment, offset: u16) -> Instruction {
        match segment {
            Segment::Constant => Instruction::Address(offset),
            Segment::Temp => Instruction::Address(5 + offset),
            Segment::Static => {
                self.static_variables += 1;
                Instruction::LabeledAddress(self.program_name.clone() + self.static_variables.to_string().as_str())
            },
            Segment::Pointer => match offset {
                0 => Instruction::LabeledAddress("THIS".to_owned()),
                1 => Instruction::LabeledAddress("THAT".to_owned()),
                _ => panic!("pointer with a value that is not 1/0 is illegal")
            },
            Segment::Local => Instruction::Address(1 + offset),
            Segment::This => Instruction::Address(3 + offset),
            Segment::That => Instruction::Address(4 + offset)
        }
    }

    fn load(addr: u16) -> Vec<Instruction> {
        let mut result = vec![];
        result.push(Instruction::Address(addr));
        result.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::A(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        result
    }

    fn gen_push_from_d(&self) -> Vec<Instruction> {
        let mut instructions = vec![];
        // @SP
        instructions.push(Instruction::Address(0));
        // AM = M+1 (AM = SP++)
        instructions.push(Instruction::Compute(ComputeFields {compute_op: ComputeOp::IncA(true), jump_op: JumpOp::Nothing, destination_op: DestOp::AM}));
        // M=D
        instructions.push(Instruction::Compute(ComputeFields {compute_op: ComputeOp::D, jump_op: JumpOp::Nothing, destination_op: DestOp::M}));
        instructions
    }

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
        let lower_line = line.trim().to_lowercase();

        if line.contains("//") {
            VMInstruction::Comment(line.to_string()) // todo: handle comments after instructions
        } else if lower_line.starts_with("push") {
            parse_push(line.to_lowercase())
        } else if try_parse_arithmetic(line).is_some() {
            try_parse_arithmetic(line).unwrap()
        } else if lower_line.starts_with("pop") {
            parse_pop(line.to_lowercase())
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
        dbg!(parsed_line);
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
            CodeGenerator::load(16)
        );
    }

    #[test]
    fn generate_push_constant_42() {
        let expected = vec!(
            "@42",
            "D=A",
            "@0", // can we just @SP?
            "AM=M+1",
            "M=D"
        );
        let instructions = CodeGenerator::new("Test".to_string()).translate(&VMInstruction::Push(Segment::Constant, 42));
        assert_eq!(expected.len(), instructions.len());
        for index in 0..expected.len() {
            let str_instruction = generate_instruction(&instructions[index]);
            assert_eq!(expected[index], str_instruction);
        }
    }

    #[test]
    fn generate_push_local_13() {
        let expected = vec!(
            "@14",  // local (1) + 13 offset
            "D=M",
            "@0", // can we just @SP?
            "AM=M+1",
            "M=D"
        );
        let instructions = CodeGenerator::new("Test".to_string()).translate(&VMInstruction::Push(Segment::Local, 13));
        assert_eq!(expected.len(), instructions.len());
        for index in 0..expected.len() {
            let str_instruction = generate_instruction(&instructions[index]);
            assert_eq!(expected[index], str_instruction);
        }
    }
}
