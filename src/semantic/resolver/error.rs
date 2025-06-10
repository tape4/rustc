#[derive(Debug)]
pub enum ResolveError {
    UndefinedSymbol { name: String },
}
