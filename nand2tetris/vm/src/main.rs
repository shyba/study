use std::fs;
use std::fs::ReadDir;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use assembler::assembler::{ComputeFields, ComputeOp, DestOp, Instruction, JumpOp};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug)]
#[derive(PartialEq)]
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
#[derive(PartialEq)]
pub enum VMInstruction {
    Comment(String),
    Push(Segment, u16),
    Pop(Segment, u16),
    Arithmetic(Arithmetic),
    Label(String),
    GoTo(String),
    IfGoTo(String),
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
        "argument" => Segment::Argument,
        "pointer" => Segment::Pointer,
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
        Instruction::Label(label) => format!("({})", label),
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
    program_name: String,
    label_counter: usize,
}

impl CodeGenerator {
    pub fn new(program_name: String) -> CodeGenerator {
        CodeGenerator {
            program_name: program_name,
            label_counter: 0,
        }
    }

    pub fn translate(&mut self, vm_instruction: &VMInstruction) -> Vec<Instruction> {
        let mut instructions = vec![];
        match &vm_instruction {
            VMInstruction::Push(segment, value) => {
                match segment {
                    Segment::Constant => {
                        // load constant
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(false),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Temp => {
                        // load constant
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Static => {
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    Segment::Pointer => {
                        instructions.extend(vec![
                            match value {
                                0 => Instruction::Address(3), // this
                                1 => Instruction::Address(4), // that
                                _ => panic!("pointer can only be used with 1 and 0"),
                            },
                            Instruction::Compute(ComputeFields {
                                compute_op: ComputeOp::A(true),
                                jump_op: JumpOp::Nothing,
                                destination_op: DestOp::D,
                            }),
                        ]);
                        instructions.extend(self.gen_push_d()) // push D
                    }
                    _ => {
                        // all other cases are loading from memory segments
                        instructions.extend(self.segment_to_address_instruction(segment, *value));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::DPlusA(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::A,
                        }));
                        instructions.push(Instruction::Compute(ComputeFields {
                            compute_op: ComputeOp::A(true),
                            jump_op: JumpOp::Nothing,
                            destination_op: DestOp::D,
                        }));
                        instructions.extend(self.gen_push_d()) // push D
                    }
                }
            }
            VMInstruction::Pop(segment, value) => match segment {
                Segment::Static => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::Address(13));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                Segment::Pointer => {
                    match value {
                        0 => instructions.push(Instruction::Address(3)),
                        1 => instructions.push(Instruction::Address(4)),
                        _ => panic!("pointer can only be 0 or 1"),
                    }
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::Address(13));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                Segment::Temp => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::A(false),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::Address(13));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
                _ => {
                    instructions.extend(self.segment_to_address_instruction(segment, *value));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DPlusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::D,
                    }));
                    instructions.push(Instruction::Address(13));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::D,
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                    instructions.extend(self.pop_to_r13_pointer())
                }
            },
            VMInstruction::Arithmetic(operation) => match operation {
                Arithmetic::Add => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DPlusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Sub => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::AMinusD(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Neg => {
                    instructions.push(Instruction::Address(0));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DecA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::MinusA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Eq => instructions.extend(self.true_or_false(JumpOp::Equal)),
                Arithmetic::Gt => instructions.extend(self.true_or_false(JumpOp::Greater)),
                Arithmetic::Lt => instructions.extend(self.true_or_false(JumpOp::Lower)),
                Arithmetic::And => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DAndA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Or => {
                    instructions.extend(self.pop_to_d());
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DOrA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
                Arithmetic::Not => {
                    instructions.push(Instruction::Address(0));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::DecA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::A,
                    }));
                    instructions.push(Instruction::Compute(ComputeFields {
                        compute_op: ComputeOp::NotA(true),
                        jump_op: JumpOp::Nothing,
                        destination_op: DestOp::M,
                    }));
                }
            },
            _ => (),
        }
        instructions
    }

    fn true_or_false(&mut self, jump_op: JumpOp) -> Vec<Instruction> {
        let mut instructions = vec![];
        instructions.extend(self.pop_to_d());
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::AMinusD(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        self.label_counter += 1;
        let true_label = format!("true.{}", self.label_counter);
        instructions.push(Instruction::LabeledAddress(true_label.clone()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::D,
            jump_op: jump_op,
            destination_op: DestOp::Nothing,
        }));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::Zero,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        self.label_counter += 1;
        let end_label = format!("end.{}", self.label_counter);
        instructions.push(Instruction::LabeledAddress(end_label.clone()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::Zero,
            jump_op: JumpOp::Unconditional,
            destination_op: DestOp::Nothing,
        }));
        instructions.push(Instruction::Label(true_label.clone()));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::MinusOne,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::D,
        }));
        instructions.push(Instruction::Label(end_label.clone()));
	instructions.push(Instruction::Address(0));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::DecA(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::A,
        }));
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::D,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::M,
        }));
        instructions
    }

    fn pop_to_d(&self) -> Vec<Instruction> {
        vec![
            Instruction::Address(0),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::D,
            }),
            Instruction::Address(0),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
        ]
    }

    fn segment_to_address_instruction(
        &mut self,
        segment: &Segment,
        offset: u16,
    ) -> Vec<Instruction> {
        let mut instructions = vec![match &segment {
            Segment::Static => {
                Instruction::LabeledAddress(self.program_name.clone() + "." + &offset.to_string())
            }
            Segment::Temp => Instruction::Address(5 + offset),
            _ => Instruction::Address(offset),
        }];
        match segment {
            Segment::Constant => (),
            Segment::Temp => (),
            Segment::Static => (),
            _ => {
                instructions.push(Instruction::Compute(ComputeFields {
                    compute_op: ComputeOp::A(false),
                    jump_op: JumpOp::Nothing,
                    destination_op: DestOp::D,
                }));
            }
        }
        match segment {
            Segment::Local => instructions.push(Instruction::Address(1)),
            Segment::Argument => instructions.push(Instruction::Address(2)),
            Segment::This => instructions.push(Instruction::Address(3)),
            Segment::That => instructions.push(Instruction::Address(4)),
            _ => (),
        }
        instructions
    }

    fn gen_push_d(&self) -> Vec<Instruction> {
        let mut instructions = vec![];
        // @SP
        instructions.push(Instruction::Address(0));
        // A=M
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::A(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::A,
        }));
        // M=D
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::D,
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::M,
        }));
        // @SP
        instructions.push(Instruction::Address(0));
        // M=M+1
        instructions.push(Instruction::Compute(ComputeFields {
            compute_op: ComputeOp::IncA(true),
            jump_op: JumpOp::Nothing,
            destination_op: DestOp::M,
        }));
        instructions
    }

    fn pop_to_r13_pointer(&self) -> Vec<Instruction> {
        vec![
            Instruction::Address(0),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::DecA(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::D,
            }),
            Instruction::Address(13),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::A(true),
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::A,
            }),
            Instruction::Compute(ComputeFields {
                compute_op: ComputeOp::D,
                jump_op: JumpOp::Nothing,
                destination_op: DestOp::M,
            }),
        ]
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

        if line.len() == 0 || line.contains("//") {
            VMInstruction::Comment(line.to_string()) // todo: handle comments after instructions
        } else if lower_line.starts_with("push") {
            parse_push(line.to_lowercase())
        } else if try_parse_arithmetic(line).is_some() {
            try_parse_arithmetic(line).unwrap()
        } else if lower_line.starts_with("pop") {
            parse_pop(line.to_lowercase())
        } else if lower_line.starts_with("label") {
	    let label = line.split(" ").skip(1).next().unwrap();
            VMInstruction::Label(String::from(label))
        } else if lower_line.starts_with("if-goto") {
	    let label = line.split(" ").skip(1).next().unwrap();
            VMInstruction::IfGoTo(String::from(label))
        } else if lower_line.starts_with("goto") {
	    let label = line.split(" ").skip(1).next().unwrap();
            VMInstruction::GoTo(String::from(label))
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
    let file = std::fs::File::open(&file_path).expect("Error opening input file");
    let mut parser = Parser::new();
    let file_name = file_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .next()
        .unwrap();
    let mut translator = CodeGenerator::new(file_name.to_string());

    let output_file_path = file_path.to_str().unwrap().replace(".vm", ".asm");
    let mut output_file =
        std::fs::File::create(&output_file_path).expect("Error opening output file");
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

    fn assert_instructions(expected: &Vec<&str>, vm_instruction: VMInstruction) {
        let instructions = CodeGenerator::new("Test".to_string()).translate(&vm_instruction);
        let smallest = expected.len().min(instructions.len());
        for index in 0..smallest {
            let str_instruction = generate_instruction(&instructions[index]);
            assert_eq!(expected[index], str_instruction);
        }
        assert_eq!(expected.len(), instructions.len());
    }

    #[test]
    fn generate_push_constant_42() {
        assert_instructions(
            &vec![
                "@42", "D=A", // can we just @SP?
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Constant, 42),
        );
    }

    #[test]
    fn generate_push_local_13() {
        assert_instructions(
            &vec![
                "@13",   // 13 offset
                "D=A",   // store offset in D
                "@1",    // LOCAL base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[LOCAL + offset]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Local, 13),
        );
    }

    #[test]
    fn generate_push_static_20() {
        assert_instructions(
            &vec![
                "@Test.20", "D=M", // program name is Test
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Static, 20),
        );
    }

    #[test]
    fn generate_push_argument_13() {
        assert_instructions(
            &vec![
                "@13",   // load offset
                "D=A",   // store offset in D
                "@2",    // THIS base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THIS + offset]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Argument, 13),
        );
    }

    #[test]
    fn generate_push_this_19() {
        assert_instructions(
            &vec![
                "@19",   // load offset
                "D=A",   // store offset in D
                "@3",    // THIS base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THIS + offset]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::This, 19),
        );
    }

    #[test]
    fn generate_push_that_15() {
        assert_instructions(
            &vec![
                "@15",   // load offset
                "D=A",   // store offset in D
                "@4",    // THAT base addr
                "A=D+M", // sum offset
                "D=M",   // read D = RAM[THAT + offset]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::That, 15),
        );
    }

    #[test]
    fn generate_push_pointer() {
        assert_instructions(
            &vec![
                "@3",  // THIS base addr
                "D=M", // read D = RAM[THIS]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Pointer, 0),
        );
        assert_instructions(
            &vec![
                "@4",  // THIS base addr
                "D=M", // read D = RAM[THIS]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Pointer, 1),
        );
    }

    #[test]
    fn generate_push_temp_4() {
        assert_instructions(
            &vec![
                "@9",  // TEMP for 4 (5 + 4)
                "D=M", // read D = RAM[9]
                "@0", "A=M", "M=D", "@0", "M=M+1",
            ],
            VMInstruction::Push(Segment::Temp, 4),
        );
    }

    #[test]
    fn generate_pop_local_2() {
        assert_instructions(
            &vec![
                "@2",    // load offset
                "D=A",   // store offset in D
                "@1",    // LOCAL base addr
                "D=D+M", // sum offset, store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Local, 2),
        );
    }

    #[test]
    fn generate_pop_static_3() {
        assert_instructions(
            &vec![
                "@Test.3", // load offset
                "D=A",     // store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Static, 3),
        );
    }

    #[test]
    fn generate_pop_this_10() {
        assert_instructions(
            &vec![
                "@10",   // load offset
                "D=A",   // store offset in D
                "@3",    // THIS base addr
                "D=D+M", // sum offset, store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::This, 10),
        );
    }

    #[test]
    fn generate_pop_pointer() {
        assert_instructions(
            &vec![
                "@3",  // THIS base addr
                "D=A", // store address in D TODO: OPTIMIZE THAT
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Pointer, 0),
        );
        assert_instructions(
            &vec![
                "@4",  // THAT base addr
                "D=A", // store address in D TODO: OPTIMIZE THAT
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Pointer, 1),
        );
    }

    #[test]
    fn generate_pop_that_44() {
        assert_instructions(
            &vec![
                "@44",   // load offset
                "D=A",   // store offset in D
                "@4",    // THAT base addr
                "D=D+M", // sum offset, store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::That, 44),
        );
    }

    #[test]
    fn generate_pop_temp_7() {
        assert_instructions(
            &vec![
                "@12", // TEMP (5 + 7)
                "D=A", // store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Temp, 7),
        );
    }

    #[test]
    fn generate_pop_argument_9() {
        assert_instructions(
            &vec![
                "@9",    // load offset
                "D=A",   // store offset in D
                "@2",    // argument base addr
                "D=D+M", // sum offset, store address in D
                "@13", "M=D", // R13=D temporarly
                "@0", "M=M-1", "A=M", "D=M", // D = RAM[SP], SP-=1
                "@13", "A=M", "M=D", // (*R13) = D
            ],
            VMInstruction::Pop(Segment::Argument, 9),
        );
    }

    #[test]
    fn generate_add() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]+=D
                "@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "M=D+M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Add),
        );
    }

    #[test]
    fn generate_sub() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "M=M-D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Sub),
        );
    }

    #[test]
    fn generate_neg() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@0", "A=M-1", "M=-M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Neg),
        );
    }

    #[test]
    fn generate_eq() {
        assert_instructions(
            &vec![
                "@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "D=M-D", "@true.1", "D;JEQ", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@0", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Eq),
        );
    }

    #[test]
    fn generate_gt() {
        assert_instructions(
            &vec![
                "@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "D=M-D", "@true.1", "D;JGT", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@0", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Gt),
        );
    }

    #[test]
    fn generate_lt() {
        assert_instructions(
            &vec![
                "@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "D=M-D", "@true.1", "D;JLT", "D=0",
                "@end.2", "0;JMP", "(true.1)", "D=-1", "(end.2)", "@0", "A=M-1", "M=D",
            ],
            VMInstruction::Arithmetic(Arithmetic::Lt),
        );
    }

    #[test]
    fn generate_and() {
        assert_instructions(
            &vec!["@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "M=D&M"],
            VMInstruction::Arithmetic(Arithmetic::And),
        );
    }

    #[test]
    fn generate_or() {
        assert_instructions(
            &vec!["@0", "M=M-1", "A=M", "D=M", "@0", "A=M-1", "M=D|M"],
            VMInstruction::Arithmetic(Arithmetic::Or),
        );
    }

    #[test]
    fn generate_not() {
        assert_instructions(
            &vec![
                //SP--, D=RAM[SP], RAM[SP-1]-=D
                "@0", "A=M-1", "M=!M",
            ],
            VMInstruction::Arithmetic(Arithmetic::Not),
        );
    }

    #[test]
    fn parse_label() {
	let mut parser = Parser::new();
	let instruction = parser.parse_line(&String::from("label ELSE"));
	assert_eq!(instruction, VMInstruction::Label(String::from("ELSE")));
    }

    #[test]
    fn parse_goto() {
	let mut parser = Parser::new();
	let instruction = parser.parse_line(&String::from("goto FAIL"));
	assert_eq!(instruction, VMInstruction::GoTo(String::from("FAIL")));
    }

    #[test]
    fn parse_if_goto() {
	let mut parser = Parser::new();
	let instruction = parser.parse_line(&String::from("if-goto FAIL"));
	assert_eq!(instruction, VMInstruction::IfGoTo(String::from("FAIL")));
    }
}
