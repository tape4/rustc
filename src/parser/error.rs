use crate::lexer::Token;

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
