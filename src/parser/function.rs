use crate::ast::functions::Parameter;
use crate::ast::{Function, Stmt, TypeSpecifier};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn parse_function_definition(&mut self) -> ParseResult<Function> {
        let return_ty = self.parse_type_specifier()?;
        let name = self.expect_ident()?;

        // "(" parameter_list? ")" 파싱
        self.expect(Token::LParen)?;
        let params: Vec<Parameter> = self.parse_parameter_list()?;
        self.expect(Token::RParen)?;

        // block 파싱 (Stmt::Block)
        let body = self.parse_block_statement()?;
        Ok(Function {
            name,
            return_ty,
            params,
            body,
        })
    }

    fn parse_type_specifier(&mut self) -> ParseResult<TypeSpecifier> {
        // 기본 타입(int|char) 확인
        let base_ty = match self.current_token() {
            Token::Int => TypeSpecifier::Int,
            Token::Char => TypeSpecifier::Char,
            _ => return self.unsupported_token(),
        };
        self.next_token();

        // 뒤따르는 "*" 만큼 포인터 레벨 올리기
        let mut ty = base_ty;
        while self.current_token() == &Token::Asterisk {
            self.next_token(); // '*' 소비
            ty = TypeSpecifier::Pointer(Box::new(ty));
        }

        Ok(ty)
    }

    fn parse_parameter_list(&mut self) -> ParseResult<Vec<Parameter>> {
        let mut params = Vec::new();

        // ')'
        if self.current_token() == &Token::RParen {
            return Ok(params);
        }

        loop {
            // 변수명, 타입
            let ty = self.parse_type_specifier()?;
            let name = self.expect_ident()?;
            params.push(Parameter { name, ty });

            // ',' 혹은 ')'
            match self.current_token() {
                Token::Comma => {
                    self.next_token();
                    continue;
                }
                Token::RParen => break,
                _ => return self.unsupported_token(),
            }
        }

        Ok(params)
    }
}
