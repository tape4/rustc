use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn expect_ident(&mut self) -> ParseResult<String> {
        let tok = self.current_token().clone();
        match tok {
            Token::EOF => self.unexpected_eof("identifier"),
            Token::Ident(name) => {
                self.next_token();
                Ok(name)
            }
            other => self.unexpected_token(other),
        }
    }

    pub fn expect_int_literal(&mut self) -> ParseResult<i64> {
        let tok = self.current_token().clone();
        match tok {
            Token::EOF => self.unexpected_eof("integer literal"),
            Token::IntLiteral(value) => {
                self.next_token();
                Ok(value)
            }
            other => self.unexpected_token(other),
        }
    }

    pub fn expect_char_literal(&mut self) -> ParseResult<char> {
        let tok = self.current_token().clone();
        match tok {
            Token::EOF => self.unexpected_eof("char literal"),
            Token::CharLiteral(value) => {
                self.next_token();
                Ok(value)
            }
            other => self.unexpected_token(other),
        }
    }

    pub fn current_token(&self) -> &Token {
        &self
            .tokens
            .get(self.pos)
            .map(|st| &st.kind)
            .unwrap_or(&Token::EOF)
    }

    pub fn current_span(&self) -> (usize, usize) {
        if let Some(st) = self.tokens.get(self.pos) {
            (st.line, st.column)
        } else {
            (0, 0)
        }
    }

    pub fn peek_token(&self) -> &Token {
        &self
            .tokens
            .get(self.pos + 1)
            .map(|st| &st.kind)
            .unwrap_or(&Token::EOF)
    }

    pub fn next_token(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    pub fn expect(&mut self, expected: Token) -> ParseResult<()> {
        if self.current_token() == &expected {
            self.next_token();
            Ok(())
        } else if self.current_token() == &Token::EOF {
            self.unexpected_eof(format!("`{:?}`", expected))
        } else {
            self.unexpected_token(expected)
        }
    }
}
