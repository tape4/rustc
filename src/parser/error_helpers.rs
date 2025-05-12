use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::error::ParserError;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn unexpected_token<T>(&self, expected: Token) -> ParseResult<T> {
        let (line, col) = self.current_span();
        return Err(ParserError::UnexpectedToken {
            expected,
            found: self.current_token().clone(),
            line,
            col,
        });
    }

    pub fn unsupported_token<T>(&self) -> ParseResult<T> {
        let (line, col) = self.current_span();
        return Err(ParserError::UnsupportedToken {
            found: self.current_token().clone(),
            line,
            col,
        });
    }
}
