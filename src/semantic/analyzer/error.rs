use crate::ast::TypeSpecifier;
use std::fmt;

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

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::UndefinedSymbol { name } => {
                write!(f, "error: use of undeclared identifier '{}'", name)
            }
            SemanticError::DuplicateDeclaration { name } => {
                write!(f, "error: redefinition of '{}'", name)
            }
            SemanticError::InvalidReturnType { expected, found } => {
                write!(
                    f,
                    "error: return type mismatch: expected '{}', found '{}'",
                    expected, found
                )
            }
            SemanticError::InvalidContinue => {
                write!(f, "error: 'continue' statement not in loop")
            }
            SemanticError::InvalidBreak => {
                write!(f, "error: 'break' statement not in loop")
            }
            SemanticError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "error: incompatible types: expected '{:?}', found '{:?}'",
                    expected, found
                )
            }
            SemanticError::NotAFunction { name } => {
                write!(f, "error: '{}' is not a function", name)
            }
            SemanticError::ArgumentCountMismatch { expected, found } => {
                write!(
                    f,
                    "error: too {} arguments to function call, expected {}, have {}",
                    if found > expected { "many" } else { "few" },
                    expected,
                    found
                )
            }
            SemanticError::ExpectedPointer { found } => {
                write!(
                    f,
                    "error: indirection requires pointer operand ('{}' invalid)",
                    format!("{:?}", found)
                )
            }
        }
    }
}

impl std::error::Error for SemanticError {}
