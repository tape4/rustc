use crate::ast::Function;

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

// 프로그램 전체: 0개 이상의 함수 정의
// program               ::= function_definition*
