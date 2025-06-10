use crate::ast::Function;

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

impl Span {
    pub fn new(start: Pos, end: Pos) -> Span {
        Span { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

impl Pos {
    pub fn new(line: usize, column: usize) -> Pos {
        Pos { line, column }
    }
}

// 프로그램 전체: 0개 이상의 함수 정의
// program               ::= function_definition*
