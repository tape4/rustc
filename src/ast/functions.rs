use crate::ast::stmt::Block;
use crate::ast::ty::TypeSpecifier;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub ty: TypeSpecifier,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub return_ty: TypeSpecifier,
    pub params: Vec<Parameter>,
    pub body: Block,
}

// 함수 정의: 반환 타입, 함수 이름, 매개변수 목록, 함수 본문
// function_definition   ::= type_specifier identifier "(" parameter_list? ")" block

// 매개변수 목록: 첫 매개변수 + 쉼표로 구분된 추가 매개변수 0개 이상
// parameter_list        ::= parameter ( "," parameter )*
// 단일 매개변수: 타입 + 이름
// parameter             ::= type_specifier identifier
