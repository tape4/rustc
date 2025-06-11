use crate::ast::TypeSpecifier;

#[derive(Debug)]
pub enum SemanticError {
    UndefinedSymbol {
        name: String,
    },
    DuplicateDeclaration {
        name: String,
    },
    InvalidReturnType {
        expected: String,
        found: String,
    },
    InvalidContinue,
    InvalidBreak,
    TypeMismatch {
        expected: TypeSpecifier,
        found: TypeSpecifier,
    },
    NotAFunction {
        name: String,
    },
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
    },
    ExpectedPointer {
        found: TypeSpecifier,
    },
}
