use std::fmt::format;

use assembler::assembler::{Instruction, ComputeFields, ComputeOp, DestOp, JumpOp};

pub enum Segment {
    Local,
    Static,
    Constant(i16),
    This,
    That,
    Pointer,
    Temp
}

pub enum Command {
    Push(Segment),
    Pop(Segment),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not
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
    Symbol(u8)
}

fn gen_load(addr: u16) -> Vec<Instruction> {
    let mut result = vec!();
    result.push(Instruction::Address(addr));
    result.push(Instruction::Compute(ComputeFields {compute_op: ComputeOp::A(true), jump_op: JumpOp::Nothing, destination_op: DestOp::D}));
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
        Instruction::Nothing => String::new()
    }
}

fn generate_compute_instruction(fields: ComputeFields) -> String {
    let mut result = String::new();
    result.push_str(match fields.compute_op {
        ComputeOp::Zero =>  "0",
        ComputeOp::One =>  "1",
        ComputeOp::MinusOne =>  "-1",
        ComputeOp::D =>  "D",
        ComputeOp::A(true) =>  "M",
        ComputeOp::A(false) =>  "A",
        ComputeOp::NotD =>  "!D",
        ComputeOp::NotA(true) =>  "!M",
        ComputeOp::NotA(false) =>  "!A",
        ComputeOp::MinusD =>  "-D",
        ComputeOp::MinusA(true) =>  "-M",
        ComputeOp::MinusA(false) =>  "-A",
        ComputeOp::IncD =>  "D+1",
        ComputeOp::IncA(true) =>  "M+1",
        ComputeOp::IncA(false) =>  "A+1",
        ComputeOp::DecD =>  "D-1",
        ComputeOp::DecA(true) =>  "M-1",
        ComputeOp::DecA(false) =>  "A-1",
        ComputeOp::DPlusA(true) =>  "D+M",
        ComputeOp::DPlusA(false) =>  "D+A",
        ComputeOp::DMinusA(true) =>  "D-M",
        ComputeOp::DMinusA(false) =>  "D-A",
        ComputeOp::AMinusD(true) =>  "M-D",
        ComputeOp::AMinusD(false) =>  "A-D",
        ComputeOp::DAndA(true) =>  "D&M",
        ComputeOp::DAndA(false) =>  "D&A",
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
        DestOp::Nothing => ""
    });
    if fields.jump_op == JumpOp::Nothing {
        return result
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
        JumpOp::Nothing => ""
    });
    result
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
   fn generate_load_instructions() {
       assert_eq!(vec!(Instruction::Address(16)), gen_load(16));
   }
}