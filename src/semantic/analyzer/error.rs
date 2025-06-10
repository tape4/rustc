#[derive(Debug)]
pub enum SemanticError {
    UndefinedSymbol { name: String },
    DuplicateDeclaration { name: String },
    InvalidReturnType { expected: String, found: String },
    InvalidContinue,
    InvalidBreak,
}
