mod errors;
use errors::{ParsingError, ParsingErrorKind};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Address(u16),
    LabeledAddress(String),
    Compute(ComputeFields),
    Comment(String),
    Label(String),
    Nothing,
}

#[derive(Debug, PartialEq)]
pub enum ComputeOp {
    Zero,
    One,
    MinusOne,
    D,
    A(bool),
    NotD,
    NotA(bool),
    MinusD,
    MinusA(bool),
    IncD,
    IncA(bool),
    DecD,
    DecA(bool),
    DPlusA(bool),
    DMinusA(bool),
    AMinusD(bool),
    DAndA(bool),
    DOrA(bool),
}

#[derive(Debug, PartialEq)]
pub enum JumpOp {
    Nothing,
    Greater,
    Equal,
    GreaterEqual,
    Lower,
    NotEqual,
    LessEqual,
    Unconditional,
}

#[derive(Debug, PartialEq)]
pub enum DestOp {
    Nothing,
    M,
    D,
    DM,
    A,
    AM,
    AD,
    ADM,
}

#[derive(Debug, PartialEq)]
pub struct ComputeFields {
    compute_op: ComputeOp,
    jump_op: JumpOp,
    destination_op: DestOp,
}

impl FromStr for ComputeOp {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = match s.split(";").next() {
            Some(value) => value,
            None => s
        };
        let line = match line.split("=").skip(1).next() {
            Some(value) => value,
            None => line
        };
        match line {
            "0" => Ok(ComputeOp::Zero),
            "1" => Ok(ComputeOp::One),
            "-1" => Ok(ComputeOp::MinusOne),
            "D" => Ok(ComputeOp::D),
            "M" => Ok(ComputeOp::A(true)),
            "A" => Ok(ComputeOp::A(false)),
            "!D" => Ok(ComputeOp::NotD),
            "!M" => Ok(ComputeOp::NotA(true)),
            "!A" => Ok(ComputeOp::NotA(false)),
            "-D" => Ok(ComputeOp::MinusD),
            "-M" => Ok(ComputeOp::MinusA(true)),
            "-A" => Ok(ComputeOp::MinusA(false)),
            "D+1" => Ok(ComputeOp::IncD),
            "M+1" => Ok(ComputeOp::IncA(true)),
            "A+1" => Ok(ComputeOp::IncA(false)),
            "D-1" => Ok(ComputeOp::DecD),
            "M-1" => Ok(ComputeOp::DecA(true)),
            "A-1" => Ok(ComputeOp::DecA(false)),
            "D+M" => Ok(ComputeOp::DPlusA(true)),
            "D+A" => Ok(ComputeOp::DPlusA(false)),
            "D-M" => Ok(ComputeOp::DMinusA(true)),
            "D-A" => Ok(ComputeOp::DMinusA(false)),
            "M-D" => Ok(ComputeOp::AMinusD(true)),
            "A-D" => Ok(ComputeOp::AMinusD(false)),
            "D&M" => Ok(ComputeOp::DAndA(true)),
            "D&A" => Ok(ComputeOp::DAndA(false)),
            "D|M" => Ok(ComputeOp::DOrA(true)),
            "D|A" => Ok(ComputeOp::DOrA(false)),
                _ => Err(ParsingError {kind: ParsingErrorKind::InvalidComputation})
        }

    }
}

impl FromStr for DestOp {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains("=") {
            Ok(DestOp::Nothing)
        } else {
            match s.split("=").nth(0) {
                Some("M") => Ok(DestOp::M),
                Some("D") => Ok(DestOp::D),
                Some("A") => Ok(DestOp::A),
                Some("DM") => Ok(DestOp::DM),
                Some("MD") => Ok(DestOp::DM),
                Some("AM") => Ok(DestOp::AM),
                Some("MA") => Ok(DestOp::AM),
                Some("AD") => Ok(DestOp::AD),
                Some("DA") => Ok(DestOp::AD),
                Some("ADM") => Ok(DestOp::ADM),
                _ => Err(ParsingError {kind: ParsingErrorKind::InvalidDestination})
            }
        }
    }
}

impl FromStr for JumpOp {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(";") {
            return Ok(JumpOp::Nothing);
        }

        let s = s.split(";").skip(1).next().unwrap_or(s);

        match s {
            "JGT" => Ok(JumpOp::Greater),
            "JEQ" => Ok(JumpOp::Equal),
            "JGE" => Ok(JumpOp::GreaterEqual),
            "JLT" => Ok(JumpOp::Lower),
            "JNE" => Ok(JumpOp::NotEqual),
            "JLE" => Ok(JumpOp::LessEqual),
            "JMP" => Ok(JumpOp::Unconditional),
            _ => Err(ParsingError {kind: ParsingErrorKind::InvalidJump})
        }
    }
}

fn parse_address(value: String) -> Result<Instruction, ParsingError> {
    let is_label = value.chars().skip(1).any(|c| {c > '9' || c < '0'});
    if !is_label {
        match value[1..].parse::<u16>() {
            Ok(parsed) if parsed <= 0x7FFF => Ok(Instruction::Address(parsed)),
            Ok(_) => Err(ParsingError {kind: ParsingErrorKind::ValueTooLarge}),
            Err(e) => Err(ParsingError {kind: ParsingErrorKind::Generic(e)}),
        }
    } else {
        match &value[1..] {
            "R0" => Ok(Instruction::Address(0)),
            "R1" => Ok(Instruction::Address(1)),
            "R2" => Ok(Instruction::Address(2)),
            "R3" => Ok(Instruction::Address(3)),
            "R4" => Ok(Instruction::Address(4)),
            "R5" => Ok(Instruction::Address(5)),
            "R6" => Ok(Instruction::Address(6)),
            "R7" => Ok(Instruction::Address(7)),
            "R8" => Ok(Instruction::Address(8)),
            "R9" => Ok(Instruction::Address(9)),
            "R10" => Ok(Instruction::Address(10)),
            "R11" => Ok(Instruction::Address(11)),
            "R12" => Ok(Instruction::Address(12)),
            "R13" => Ok(Instruction::Address(13)),
            "R14" => Ok(Instruction::Address(14)),
            "R15" => Ok(Instruction::Address(15)),
            "SP" => Ok(Instruction::Address(0)),
            "LCL" => Ok(Instruction::Address(1)),
            "ARG" => Ok(Instruction::Address(2)),
            "THIS" => Ok(Instruction::Address(3)),
            "THAT" => Ok(Instruction::Address(4)),
            "SCREEN" => Ok(Instruction::Address(16384)),
            "KBD" => Ok(Instruction::Address(24576)),
            _ => Ok(Instruction::LabeledAddress(value[1..].to_string()))
        }
    }
}

pub fn parse(line: &str) -> Result<Instruction, ParsingError> {
    let original_line = line.to_owned();
    let line = line.replace(" ", "");
    if line.starts_with("@") {
        parse_address(line)
    } else if line == "" {
        Ok(Instruction::Nothing)
    } else if line.starts_with("(") {
        Ok(Instruction::Label(line.chars().skip(1).take(line.len()-2).collect()))
    } else if line.starts_with("//") {
        Ok(Instruction::Comment(original_line))
    } else {
        let line = match line.find("//") {
            Some(idx) => &line[..idx],
            None => &line
        };
        let dest = DestOp::from_str(&line)?;
        let comp = ComputeOp::from_str(&line)?;
        let jump = JumpOp::from_str(&line)?;
        Ok(Instruction::Compute(ComputeFields {destination_op: dest, compute_op: comp, jump_op: jump}))
    }
}

pub fn assemble_address(from_value: &u16) -> String {
    format!("0{:015b}", from_value)
}

fn assemble_compute_op(op: &ComputeOp) -> &'static str {
    match op {
        ComputeOp::Zero =>           "0101010",
        ComputeOp::One =>            "0111111",
        ComputeOp::MinusOne =>       "0111010",
        ComputeOp::D =>              "0001100",
        ComputeOp::A(true) =>        "1110000",
        ComputeOp::A(false) =>       "0110000",
        ComputeOp::NotD =>           "0001101",
        ComputeOp::NotA(true) =>     "1110001",
        ComputeOp::NotA(false) =>    "0110001",
        ComputeOp::MinusD =>         "0001111",
        ComputeOp::MinusA(true) =>   "1110011",
        ComputeOp::MinusA(false) =>  "0110011",
        ComputeOp::IncD =>           "0011111",
        ComputeOp::IncA(true) =>     "1110111",
        ComputeOp::IncA(false) =>    "0110111",
        ComputeOp::DecD =>           "0001110",
        ComputeOp::DecA(true) =>     "1110010",
        ComputeOp::DecA(false) =>    "0110010",
        ComputeOp::DPlusA(true) =>   "1000010",
        ComputeOp::DPlusA(false) =>  "0000010",
        ComputeOp::DMinusA(true) =>  "1010011",
        ComputeOp::DMinusA(false) => "0010011",
        ComputeOp::AMinusD(true) =>  "1000111",
        ComputeOp::AMinusD(false) => "0000111",
        ComputeOp::DAndA(true) =>    "1000000",
        ComputeOp::DAndA(false) =>   "0000000",
        ComputeOp::DOrA(true) =>     "1010101",
        ComputeOp::DOrA(false) =>    "0010101",
    }
}

fn assemble_dest_op(op: &DestOp) -> &'static str {
    match op {
        DestOp::Nothing => "000",
        DestOp::M =>       "001",
        DestOp::D =>       "010",
        DestOp::DM =>      "011",
        DestOp::A =>       "100",
        DestOp::AM =>      "101",
        DestOp::AD =>      "110",
        DestOp::ADM =>     "111",
    }

}

fn assemble_jump_op(op: &JumpOp) -> &'static str {
    match op {
        JumpOp::Nothing =>       "000",
        JumpOp::Greater =>       "001",
        JumpOp::Equal =>         "010",
        JumpOp::GreaterEqual =>  "011",
        JumpOp::Lower =>         "100",
        JumpOp::NotEqual =>      "101",
        JumpOp::LessEqual =>     "110",
        JumpOp::Unconditional => "111",
    }

}

fn assemble_compute(fields: &ComputeFields) -> String {
    format!("111{}{}{}", assemble_compute_op(&fields.compute_op), assemble_dest_op(&fields.destination_op), assemble_jump_op(&fields.jump_op))
}

pub fn assemble(instruction: &Instruction) -> String {
    match instruction {
        Instruction::Address(addr) => assemble_address(addr),
        Instruction::Compute(fields) => assemble_compute(&fields),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::Instruction::Address;
    use crate::assembler::*;

    #[test]
    fn parse_and_assemble_with_label() {
        assert_eq!("0000000000000000", assemble(&parse("@R0").unwrap()));
    }

    #[test]
    fn it_assembles_addresses() {
        assert_eq!("0000000000000010", assemble(&Instruction::Address(2)));
        assert_eq!("0111111111111111", assemble(&Instruction::Address(0x7fff)));
    }

    #[test]
    fn it_assembles_computation_cases() {
        assert_eq!("1110111111001000", assemble(&Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::One,
                destination_op: DestOp::M,
                jump_op: JumpOp::Nothing
            }
        )));
    }

    #[test]
    fn it_parses_address_simple_case() {
        let case = " @ 123 ".to_string();
        let parsed: Instruction = parse(&case).expect("fail");
        assert_eq!(parsed, Address(123));
    }

    #[test]
    fn it_works_for_largest_15_bit_value() {
        let case = "@32767".to_string();
        let parsed: Instruction = parse(&case).expect("fail");
        assert_eq!(parsed, Address(0x7FFF));
    }

    #[test]
    fn it_fails_for_more_than_15_bits() {
        let case = "@32768";
        let parsed = parse(case);
        assert_eq!(parsed, Err(ParsingError {kind: ParsingErrorKind::ValueTooLarge}));
    }

    #[test]
    fn it_parses_simple_cases() {
        let parsed = parse("0;JMP").expect("fail");
        assert_eq!(parsed, Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::Zero,
                destination_op: DestOp::Nothing,
                jump_op: JumpOp::Unconditional
            }
        ));
        let parsed = parse("M=D+1;JGE").expect("fail");
        assert_eq!(parsed, Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::IncD,
                destination_op: DestOp::M,
                jump_op: JumpOp::GreaterEqual
            }
        ));
        let parsed = parse("D-1;JLT").expect("fail");
        assert_eq!(parsed, Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::DecD,
                destination_op: DestOp::Nothing,
                jump_op: JumpOp::Lower
            }
        ));
        let parsed = parse("M=M-1 // what is that?").expect("fail");
        assert_eq!(parsed, Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::DecA(true),
                destination_op: DestOp::M,
                jump_op: JumpOp::Nothing
            }
        ));
        let parsed = parse("M=M-1").expect("fail");
        assert_eq!(parsed, Instruction::Compute(
            ComputeFields {
                compute_op: ComputeOp::DecA(true),
                destination_op: DestOp::M,
                jump_op: JumpOp::Nothing
            }
        ));
        let parsed = parse("(MYLABEL)").expect("fail");
        assert_eq!(parsed, Instruction::Label("MYLABEL".to_string()));
        let parsed = parse("@MYLABEL").expect("fail");
        assert_eq!(parsed, Instruction::LabeledAddress("MYLABEL".to_string()));
        let parsed = parse("").expect("fail");
        assert_eq!(parsed, Instruction::Nothing);
    }
}
