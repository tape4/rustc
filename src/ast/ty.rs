#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpecifier {
    Int,
    Char,
    Void,
    Pointer(Box<TypeSpecifier>),
}

// 타입 지정자: 기본 타입(int|char|void) + 0개 이상 포인터
// type_specifier        ::= ( "int" | "char" | "void" ) "*"*
