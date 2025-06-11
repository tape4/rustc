use crate::ast::Program;
use crate::lexer::{SpannedToken, Token};
use crate::parser::error::ParserError;

pub type ParseResult<T> = Result<T, ParserError>;

pub struct Parser {
    pub tokens: Vec<SpannedToken>,
    pub pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// program ::= function_definition*
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut functions = Vec::new();
        while self.current_token() != &Token::EOF {
            // 토큰이 'int'|'char'|'void' 시작이면 함수 정의
            let func = self.parse_function()?;
            functions.push(func);
        }
        Ok(Program { functions })
    }
}
