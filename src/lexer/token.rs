/// Lex Errors
#[derive(Debug, PartialEq, Clone)]
pub enum LexError {
    // 닫는 따옴표 없이 끝난 문자 리터럴
    UnterminatedCharLiteral,
    // 잘못된 정수 리터럴 (파싱 실패)
    InvalidNumericLiteral(String),
}

pub struct SpannedToken {
    pub kind: Token,
    pub line: usize,
    pub column: usize,
}

/// Token types
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Special
    EOF,
    Error(LexError), // 에러용
    Illegal(char),

    // Identifiers
    Ident(String),

    // literals
    IntLiteral(i64),   // 123
    CharLiteral(char), // 'a', '\n', '\0'

    // Operators
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Percent,  // %

    Equal,    // ==
    NotEqual, // !=
    Lt,       // <
    Gt,       // >
    Le,       // <=
    Ge,       // >=

    Assign,    // =
    And,       // &&
    Or,        // ||
    Not,       // !
    Ampersand, // &
    BitOr,     // |
    BitXor,    // ^

    Semicolon, // ;
    Comma,     // ,

    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // Keywords
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,

    // Types
    Int,
    Char,

    Increment,      // ++
    Decrement,      // --
    PlusAssign,     // +=
    MinusAssign,    // -=
    AsteriskAssign, // *=
    SlashAssign,    // /=
    ModuloAssign,   // %=
    BitOrAssign,    // |=
    BitAndAssign,   // &=
    BitXorAssign,   // ^=
}

/// Lookup identifier keyword
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "int" => Token::Int,
        "char" => Token::Char,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "for" => Token::For,
        "return" => Token::Return,
        "break" => Token::Break,
        "continue" => Token::Continue,
        _ => Token::Ident(ident.to_string()),
    }
}
