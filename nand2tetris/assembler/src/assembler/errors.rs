use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct ParsingError {
    pub kind: ParsingErrorKind
}
#[derive(Eq, PartialEq, Debug)]
pub enum ParsingErrorKind {
    Generic(std::num::ParseIntError),
    ValueTooLarge,
    InvalidDestination,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ParsingErrorKind::Generic(e) => write!(f, "Error parsing address: {}", e),
            ParsingErrorKind::ValueTooLarge => write!(f, "Value should be at most 15 bits."),
            ParsingErrorKind::InvalidDestination => write!(f, "Invalid destination register.")
        }
    }
}