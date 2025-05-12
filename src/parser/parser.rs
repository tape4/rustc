use crate::ast::functions::Parameter;
use crate::ast::stmt::Block;
use crate::ast::{Function, Program, Stmt, TypeSpecifier};
use crate::lexer::Token;
use crate::lexer::token::SpannedToken;
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

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut functions = Vec::new();
        while self.current_token() != &Token::EOF {
            // 토큰이 'int'|'char' 시작이면 함수 정의
            let func = self.parse_function_definition()?;
            functions.push(func);
        }
        Ok(Program { functions })
    }
}
