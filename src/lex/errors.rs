use crate::lex::types::LexItem;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Location {
    pub character: usize,
}

#[derive(Clone, Debug)]
pub struct LexError {
    pub error_type: LexErrorType,
    pub location: Location,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LexErrorType {
    Unfinished(String),
    UnclosedStringLiteral(String),
    InvalidEscape(String),
    InvalidLiteral(String),
    InvalidSize(usize),
    InvalidCharacter(char),
    EmptyNumericLiteral,
    LargeNumericLiteral,
    UnfinishedEscape,
}

impl Display for LexErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            LexErrorType::Unfinished(comment) => write!(f, "Unfinished statement: {}", comment),
            LexErrorType::UnclosedStringLiteral(comment) => {
                write!(f, "String literal \"{}\" never ends", comment)
            }
            LexErrorType::InvalidEscape(comment) => {
                write!(f, "Invalid escape sequence: {}", comment)
            }
            LexErrorType::InvalidLiteral(comment) => write!(f, "Invalid literal: {}", comment),
            LexErrorType::InvalidSize(comment) => write!(f, "Invalid size: {} bits long", comment),
            LexErrorType::InvalidCharacter(chr) => {
                write!(f, "'\\{}' is not a valid escape sequence", chr)
            }
            LexErrorType::EmptyNumericLiteral => write!(f, "Empty numeric literal"),
            LexErrorType::LargeNumericLiteral => {
                write!(f, "Numeric literal too large for any data type")
            }
            LexErrorType::UnfinishedEscape => write!(f, "Escape sequence unfinished"),
        }
    }
}

pub type LexResult = Result<LexSuccess, LexError>;

#[derive(Clone, Debug, PartialEq)]
pub struct LexSuccess {
    pub item: LexItem,
    pub location: Location,
}
