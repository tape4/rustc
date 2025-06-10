#[derive(Debug)]
pub enum SymbolError {
    DuplicateDeclaration { name: String },
}
