use crate::ast::functions::Parameter;
use crate::ast::stmt::Block;
use crate::ast::{Function, TypeSpecifier};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::error::ParserError;
use crate::parser::parser::ParseResult;

impl Parser {
    /// 함수 헤더만 파싱
    fn parse_function_header(&mut self) -> ParseResult<(TypeSpecifier, String, Vec<Parameter>)> {
        let return_ty = self.parse_type_specifier()?;
        let name = self.expect_ident()?;
        self.expect(Token::LParen)?;
        let params = self.parse_parameters()?;
        self.expect(Token::RParen)?;
        Ok((return_ty, name, params))
    }

    /// function ::= function_declaration | function_definition
    pub fn parse_function(&mut self) -> ParseResult<Function> {
        let (return_ty, name, params) = self.parse_function_header()?;

        // 선언/정의 분기
        let body = match self.current_token() {
            Token::Semicolon => {
                // 선언: 세미콜론만 소비하고 빈 블록(body) 생성
                self.next_token();
                Block {
                    statements: Vec::new(),
                }
            }
            Token::LBrace => {
                // 정의: 실제 블록 파싱
                self.parse_block_statement()?
            }
            other => {
                return Err(ParserError::UnexpectedToken {
                    expected: Token::Semicolon, // 또는 Token::LBrace
                    found: other.clone(),
                    line: self.current_span().0,
                    col: self.current_span().1,
                });
            }
        };

        Ok(Function {
            name,
            return_ty,
            params,
            body,
        })
    }

    fn parse_parameters(&mut self) -> ParseResult<Vec<Parameter>> {
        // void만 있고 바로 ')' 이면 파라미터 없음
        if self.current_token() == &Token::Void && self.peek_token() == &Token::RParen {
            self.next_token(); // void 소비
            return Ok(Vec::new());
        }
        // 아무것도 없으면 빈 벡터
        if self.current_token() == &Token::RParen {
            return Ok(Vec::new());
        }
        // 진짜 파라미터 목록
        self.parse_parameter_list()
    }

    /// type_specifier ::= ( "int" | "char" | "void" ) "*"*
    pub fn parse_type_specifier(&mut self) -> ParseResult<TypeSpecifier> {
        // 기본 타입(int|char|void) 확인
        let base_ty = match self.current_token() {
            Token::Int => TypeSpecifier::Int,
            Token::Char => TypeSpecifier::Char,
            Token::Void => TypeSpecifier::Void,
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

    /// parameter_list ::= parameter ( "," parameter )*
    fn parse_parameter_list(&mut self) -> ParseResult<Vec<Parameter>> {
        let mut params = Vec::new();

        // ')'
        if self.current_token() == &Token::RParen {
            return Ok(params);
        }

        loop {
            // 변수명, 타입
            let base_ty = self.parse_type_specifier()?;
            let name = self.expect_ident()?;
            let ty = if self.current_token() == &Token::LBracket {
                // '['
                self.next_token();
                // 크기 명시(optional)
                if self.current_token() != &Token::RBracket {
                    // ex) int arr[10]
                    let _sz = self.expect_int_literal()?;
                }
                // ']' 소비
                self.expect(Token::RBracket)?;
                // 배열 파라미터를 포인터로
                TypeSpecifier::Pointer(Box::new(base_ty))
            } else {
                base_ty
            };

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
