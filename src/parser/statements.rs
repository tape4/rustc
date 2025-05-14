use crate::ast::Stmt;
use crate::ast::Stmt::{Break, Return};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn parse_statement(&mut self) -> ParseResult<Stmt> {
        let stmt = match self.current_token() {
            Token::Break => self.parse_break_statement()?,
            _ => unreachable!(), // Token::LBrace => self.parse_block_statement(),
                                 // …
        };

        Ok(stmt)
    }

    fn parse_continue_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Continue)?; // continue 소비
        self.expect(Token::Semicolon)?; // ';' 소비
        Ok(Stmt::Continue)
    }
    fn parse_break_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Break)?; // break 소비
        self.expect(Token::Semicolon)?; // ';' 소비
        Ok(Break)
    }
    // pub fn parse_block_statement(&mut self) -> ParseResult<Block> {
    //     let mut statements: Vec<Stmt> = Vec::new();
    //     //todo
    //     Ok(Block { statements })
    // }
    // fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
    //     Ok(Stmt::If { cond: /* value */, then_branch: /* value */, else_branch: /* value */ }
    // }

    // fn parse_if_statement(&mut self) -> ParseResult<Stmt> {  }
    // while, for, return, decl, expr_stmt, break, continue
}
