use crate::ast::Stmt;
use crate::ast::stmt::Block;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn parse_block_statement(&mut self) -> ParseResult<Block> {
        let mut statements: Vec<Stmt> = Vec::new();
        //todo
        Ok(Block { statements })
    }
    // fn parse_if_statement(&mut self) -> ParseResult<Stmt> {  }
    // while, for, return, decl, expr_stmt, break, continue
}
