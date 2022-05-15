use std::num::IntErrorKind::PosOverflow;
use std::num::ParseIntError;
mod errors;
use errors::{ParsingError, ParsingErrorKind};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Address(u16),
    Compute(ComputeFields),
    Comment(String),
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

enum ParserState {
    Start,
    Error,
    GotAt,
    ParsingAddress(u8, u16),
    GotDest(DestOp),
    ParsingComp(DestOp, String),
    ParsingJump(DestOp, ComputeOp, String),
}

fn parse_address(value: String) -> Result<Instruction, ParsingError> {
    match value[1..].parse::<u16>() {
        Ok(parsed) if parsed <= 0x7FFF => Ok(Instruction::Address(parsed)),
        Ok(_) => Err(ParsingError {kind: ParsingErrorKind::ValueTooLarge}),
        Err(e) => Err(ParsingError {kind: ParsingErrorKind::Generic(e)}),
    }
}

fn parse_dest(line: String) -> Result<DestOp, ParsingError> {
    if !line.contains("=") {
        Ok(DestOp::Nothing)
    } else {
        match line.split("=").nth(0) {
            Some("M") => Ok(DestOp::M),
            Some("D") => Ok(DestOp::D),
            Some("A") => Ok(DestOp::A),
            Some("DM") => Ok(DestOp::DM),
            Some("AM") => Ok(DestOp::AM),
            Some("AD") => Ok(DestOp::AD),
            Some("ADM") => Ok(DestOp::ADM),
            _ => Err(ParsingError {kind: ParsingErrorKind::InvalidDestination})
        }
    }
}

fn parse_computation(line: String) -> Result<ComputeOp, ParsingError> {
    let line = match line.split(";").next() {
        Some(value) => value.to_string(),
        None => line
    };
    match line.as_str() {
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
        "M+!" => Ok(ComputeOp::IncA(true)),
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
            _ => Err(ParsingError {kind: ParsingErrorKind::InvalidDestination})
    }
}

fn parse(line: String) -> Result<Instruction, ParsingError> {
    let line = line.replace(" ", "");
    if line.starts_with("@") {
        return parse_address(line);
    }
    let dest = parse_dest(line.clone());
    Ok(Instruction::Comment(line))
}

#[cfg(test)]
mod tests {
    use crate::assembler::Instruction::Address;
    use crate::assembler::*;

    #[test]
    fn it_parses_address_simple_case() {
        let case = " @ 123 ".to_string();
        let parsed: Instruction = parse(case).expect("fail");
        assert_eq!(parsed, Address(123));
    }

    #[test]
    fn it_works_for_largest_15_bit_value() {
        let case = "@32767".to_string();
        let parsed: Instruction = parse(case).expect("fail");
        assert_eq!(parsed, Address(0x7FFF));
    }

    #[test]
    fn it_fails_for_more_than_15_bits() {
        let case = "@32768".to_string();
        let parsed = parse(case);
        assert_eq!(parsed, Err(ParsingError {kind: ParsingErrorKind::ValueTooLarge}));
    }
}
