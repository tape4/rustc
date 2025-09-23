use crate::lexer::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken {
        expected: Token,
        found: Token,
        line: usize,
        col: usize,
    },
    UnexpectedEOF {
        expected: String,
        line: usize,
        col: usize,
    },
    UnsupportedToken {
        found: Token,
        line: usize,
        col: usize,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken {
                expected,
                found,
                line,
                col,
            } => {
                write!(
                    f,
                    "{}:{}: syntax error: expected '{:?}', found '{:?}'",
                    line, col, expected, found
                )
            }
            ParserError::UnexpectedEOF {
                expected,
                line,
                col,
            } => {
                write!(
                    f,
                    "{}:{}: unexpected end of file, expected {}",
                    line, col, expected
                )
            }
            ParserError::UnsupportedToken { found, line, col } => {
                write!(f, "{}:{}: unsupported token '{:?}'", line, col, found)
            }
        }
    }
}

impl std::error::Error for ParserError {}
