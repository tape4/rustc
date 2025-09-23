use std::fmt;

#[derive(Debug)]
pub enum CompilerError {
    Lexer(String, usize, usize),
    Parser(ParserError),
    Semantic(SemanticError),
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnexpectedToken { expected: String, found: String },
    UnexpectedEOF { expected: String },
    UnsupportedToken { token: String },
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMismatch { expected: String, found: String },
    DuplicateDeclaration(String),
    InvalidOperation { op: String, types: String },
    BreakOutsideLoop,
    ContinueOutsideLoop,
    ReturnTypeMismatch { expected: String, found: String },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lexer(msg, line, col) => {
                write!(f, "Lexical error at {}:{}: {}", line, col, msg)
            }
            CompilerError::Parser(e) => write!(f, "{}", e),
            CompilerError::Semantic(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Syntax error at {}:{}: ", self.line, self.column)?;
        match &self.kind {
            ParserErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "expected '{}', found '{}'", expected, found)
            }
            ParserErrorKind::UnexpectedEOF { expected } => {
                write!(f, "unexpected end of file, expected {}", expected)
            }
            ParserErrorKind::UnsupportedToken { token } => {
                write!(f, "unsupported token '{}'", token)
            }
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let (Some(line), Some(col)) = (self.line, self.column) {
            write!(f, "Semantic error at {}:{}: ", line, col)?;
        } else {
            write!(f, "Semantic error: ")?;
        }
        
        match &self.kind {
            SemanticErrorKind::UndefinedVariable(name) => {
                write!(f, "undefined variable '{}'", name)
            }
            SemanticErrorKind::UndefinedFunction(name) => {
                write!(f, "undefined function '{}'", name)
            }
            SemanticErrorKind::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected '{}', found '{}'", expected, found)
            }
            SemanticErrorKind::DuplicateDeclaration(name) => {
                write!(f, "duplicate declaration of '{}'", name)
            }
            SemanticErrorKind::InvalidOperation { op, types } => {
                write!(f, "invalid operation '{}' for types {}", op, types)
            }
            SemanticErrorKind::BreakOutsideLoop => {
                write!(f, "'break' statement not in loop")
            }
            SemanticErrorKind::ContinueOutsideLoop => {
                write!(f, "'continue' statement not in loop")
            }
            SemanticErrorKind::ReturnTypeMismatch { expected, found } => {
                write!(f, "return type mismatch: expected '{}', found '{}'", expected, found)
            }
        }
    }
}

pub fn print_error_with_source(error: &CompilerError, source: &str) {
    eprintln!("{}", error);
    
    let (line, column) = match error {
        CompilerError::Lexer(_, l, c) => (*l, *c),
        CompilerError::Parser(e) => (e.line, e.column),
        CompilerError::Semantic(e) => {
            if let (Some(l), Some(c)) = (e.line, e.column) {
                (l, c)
            } else {
                return;
            }
        }
    };
    
    let lines: Vec<&str> = source.lines().collect();
    if line > 0 && line <= lines.len() {
        let line_str = lines[line - 1];
        let line_num_str = format!("{}", line);
        let padding = " ".repeat(line_num_str.len());
        
        eprintln!("{}  |", padding);
        eprintln!("{} | {}", line_num_str, line_str);
        eprintln!("{}  |{}^", padding, " ".repeat(column.saturating_sub(1)));
    }
}