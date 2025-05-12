use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn expect_ident(&mut self) -> ParseResult<String> {
        if let Token::Ident(name) = self.current_token().clone() {
            self.next_token();
            Ok(name.clone())
        } else {
            self.unexpected_token(Token::Ident(String::from("Identifier")))
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
        } else {
            self.unexpected_token(expected)
        }
    }
}
