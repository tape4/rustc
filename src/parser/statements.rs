use crate::ast::Stmt;
use crate::ast::Stmt::{Break, For, Return, While};
use crate::ast::stmt::{Block, Declarator};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    /// statement             ::= block
    /// | if_statement
    /// | while_statement
    /// | for_statement
    /// | return_statement
    /// | break_statement
    /// | continue_statement
    /// | declaration_statement
    /// | expression_statement
    pub fn parse_statement(&mut self) -> ParseResult<Stmt> {
        let stmt = match self.current_token() {
            Token::LBrace => {
                let block = self.parse_block_statement()?;
                Stmt::Block(block)
            }
            Token::If => self.parse_if_statement()?,
            Token::While => self.parse_while_statement()?,
            Token::For => self.parse_for_statement()?,
            Token::Return => self.parse_return_statement()?,
            Token::Break => self.parse_break_statement()?,
            Token::Continue => self.parse_continue_statement()?,
            Token::Int | Token::Char => self.parse_declaration_statement()?,
            _ => self.parse_expression_statement()?,
        };

        Ok(stmt)
    }

    /// expression_statement ::= expression? ";"
    fn parse_expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = match self.current_token() {
            &Token::Semicolon => None,
            _ => Some(self.parse_expr()?),
        };
        self.expect(Token::Semicolon)?;
        Ok(Stmt::ExprStmt(expr))
    }

    /// declaration_statement ::= type_specifier init_declarator_list ";"
    fn parse_declaration_statement(&mut self) -> ParseResult<Stmt> {
        let ty = self.parse_type_specifier()?;
        let declarators = self.parse_init_declarator_list()?;
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Declaration { ty, declarators })
    }

    /// init_declarator_list ::= init_declarator ( "," init_declarator )*
    fn parse_init_declarator_list(&mut self) -> ParseResult<Vec<Declarator>> {
        let mut list = Vec::new();
        // 첫 번째 선언자는 반드시 있어야 함
        list.push(self.parse_init_declarator()?);

        // 콤마로 이어지는 추가 선언자들
        while self.current_token() == &Token::Comma {
            self.next_token(); // ','
            list.push(self.parse_init_declarator()?);
        }
        Ok(list)
    }

    /// init_declarator ::= declarator ( "=" initializer )?
    fn parse_init_declarator(&mut self) -> ParseResult<Declarator> {
        // 1) 선언자 파싱
        let mut decl = self.parse_declarator()?;
        // 2) 선택적 초기화
        if self.current_token() == &Token::Assign {
            self.next_token(); // '='
            let init_expr = self.parse_initializer()?;
            decl.init = Some(init_expr);
        }
        Ok(decl)
    }

    /// declarator ::= identifier ( "[" int_literal "]" )?
    fn parse_declarator(&mut self) -> ParseResult<Declarator> {
        // 1) 이름
        let name = self.expect_ident()?;
        // 2) 선택적 배열 첨자
        let array_size = if self.current_token() == &Token::LBracket {
            self.next_token(); // '['
            let sz = self.expect_int_literal()?;
            self.expect(Token::RBracket)?; // ']'
            Some(sz)
        } else {
            None
        };
        Ok(Declarator {
            name,
            array_size,
            init: None,
        })
    }

    /// if_statement ::= "if" "(" expression ")" statement ( "else" statement )?
    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::If)?; // 'if' 소비
        self.expect(Token::LParen)?; // '(' 소비
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?; // ')' 소비

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = match self.current_token() {
            Token::Else => {
                self.expect(Token::Else)?; // 'else' 소비
                Some(Box::new(self.parse_statement()?))
            }
            _ => None,
        };

        Ok(Stmt::If {
            cond,
            then_branch,
            else_branch,
        })
    }

    /// continue_statement ::= "continue" ";"
    fn parse_continue_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Continue)?; // continue 소비
        self.expect(Token::Semicolon)?; // ';' 소비
        Ok(Stmt::Continue)
    }

    /// break_statement ::= "break" ";"
    fn parse_break_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Break)?; // break 소비
        self.expect(Token::Semicolon)?; // ';' 소비
        Ok(Break)
    }

    /// block ::= "{" statement* "}"
    pub fn parse_block_statement(&mut self) -> ParseResult<Block> {
        self.expect(Token::LBrace)?; // '{' 소비

        let mut statements = Vec::new();
        while self.current_token() != &Token::RBrace {
            if self.current_token() == &Token::EOF {
                return self.unexpected_eof(concat!("`", "}", "`"));
            }
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        self.expect(Token::RBrace)?; // '}' 소비
        Ok(Block { statements })
    }

    /// while_statement ::= "while" "(" expression ")" statement
    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(While { cond, body })
    }

    /// for_statement ::= "for" "(" expression? ";" expression? ";" expression? ")" statement
    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::For)?; // 'for' 소비
        self.expect(Token::LParen)?; // '(' 소비

        // init
        let init = match self.current_token() {
            Token::Semicolon => {
                self.expect(Token::Semicolon)?; // ';' 소비
                None
            }
            Token::Void | Token::Int | Token::Char => {
                Some(Box::new(self.parse_declaration_statement()?))
            }
            _ => {
                let e = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Some(Box::new(Stmt::ExprStmt(Some(e))))
            }
        };

        // condition
        let cond = match self.current_token() {
            Token::Semicolon => None,
            _ => Some(self.parse_expr()?),
        };
        self.expect(Token::Semicolon)?; // ';' 소비

        let step = match self.current_token() {
            Token::RParen => None,
            _ => Some(self.parse_expr()?),
        };
        self.expect(Token::RParen)?; // ')' 소비
        let body = Box::new(self.parse_statement()?);

        Ok(For {
            init,
            cond,
            step,
            body,
        })
    }

    /// return_statement ::= "return" expression? ";"
    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {
        self.expect(Token::Return)?; // 'return' 소비
        let expr = match self.current_token() {
            Token::Semicolon => None,
            _ => Some(self.parse_expr()?),
        };
        self.expect(Token::Semicolon)?; // ';' 소비
        Ok(Return(expr))
    }
}
