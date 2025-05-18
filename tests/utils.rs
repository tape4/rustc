use rustc_tape4::ast::{Expr, Program, Stmt};
use rustc_tape4::lexer::{Lexer, Token};
use rustc_tape4::parser::Parser;
use rustc_tape4::parser::parser::ParseResult;

/// 입력 전체를 순환하며 토큰을 수집
pub fn collect_tokens(input: &str) -> Vec<Token> {
    let mut l = Lexer::new(input);
    let tokens = l
        .collect_spanned_tokens()
        .iter()
        .map(|tok| tok.kind.clone())
        .collect();

    tokens
}

pub fn parse_program(input: &str) -> ParseResult<Program> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    if result.is_err() {
        println!("파싱 오류: {:?}", result.as_ref().err());
    }

    result
}

/// 표현식 문자열을 파싱하여 Expr AST 반환
pub fn parse_expression(input: &str) -> Expr {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.collect_spanned_tokens();

    // 디버깅용: 토큰 출력
    println!(
        "파싱할 토큰: {:?}",
        tokens.iter().map(|t| &t.kind).collect::<Vec<_>>()
    );

    let mut parser = Parser::new(tokens);
    match parser.parse_expr() {
        Ok(expr) => expr,
        Err(e) => {
            panic!("표현식 파싱 실패: {:?}\n입력: '{}'", e, input);
        }
    }
}

/// 문장 문자열을 파싱하여 Stmt AST 반환
pub fn parse_statement(input: &str) -> Stmt {
    // 문장은 세미콜론으로 끝나야 하므로 없으면 추가
    let input = if !input.trim().ends_with(';') {
        format!("{};", input)
    } else {
        input.to_string()
    };

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.collect_spanned_tokens();

    // 디버깅용: 토큰 출력
    println!(
        "파싱할 토큰: {:?}",
        tokens.iter().map(|t| &t.kind).collect::<Vec<_>>()
    );

    let mut parser = Parser::new(tokens);
    match parser.parse_statement() {
        Ok(stmt) => stmt,
        Err(e) => {
            panic!("문장 파싱 실패: {:?}\n입력: '{}'", e, input);
        }
    }
}
