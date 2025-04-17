mod utils;
use rustc_tape4::lexer::{LexError, Lexer, Token};
use utils::collect_tokens;

#[test]
fn simple_tokens() {
    let input = "+ - * / % = == != < <= > >= & && | || ; , ( ) { } [ ]";
    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::Slash,
        Token::Percent,
        Token::Assign,
        Token::Equal,
        Token::NotEqual,
        Token::Lt,
        Token::Le,
        Token::Gt,
        Token::Ge,
        Token::Ampersand,
        Token::And,
        Token::BitOr,
        Token::Or,
        Token::Semicolon,
        Token::Comma,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::LBracket,
        Token::RBracket,
        Token::EOF,
    ];
    assert_eq!(collect_tokens(input), expected);
}

#[test]
fn identifiers_and_numbers() {
    let input = "foo _bar Baz123 42 007";
    let expected = vec![
        Token::Ident("foo".into()),
        Token::Ident("_bar".into()),
        Token::Ident("Baz123".into()),
        Token::IntLiteral(42),
        Token::IntLiteral(7), // 앞 0은 무시
        Token::EOF,
    ];
    assert_eq!(collect_tokens(input), expected);
}

#[test]
fn char_literals_and_errors() {
    let input = r"'a' '\n' '\0' 'x";
    let expected = vec![
        Token::CharLiteral('a'),
        Token::CharLiteral('\n'),
        Token::CharLiteral('\0'),
        Token::Error(LexError::UnterminatedCharLiteral),
        Token::EOF,
    ];
    assert_eq!(collect_tokens(input), expected);
}

#[test]
fn comments_and_whitespace() {
    let input = "
        // single line comment
        foo /* multi
                 line */ 123
    ";
    let expected = vec![
        Token::Ident("foo".into()),
        Token::IntLiteral(123),
        Token::EOF,
    ];
    assert_eq!(collect_tokens(input), expected);
}
