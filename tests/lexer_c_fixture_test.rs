mod utils;
use rustc_tape4::lexer::{Lexer, Token};
use utils::collect_tokens;

#[test]
fn test_sample_c_tokens() {
    let src = include_str!("fixtures/sample.c");
    let mut l = Lexer::new(src);
    let mut next_token = || l.next_token();

    // int add(int a, int b) {
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("add".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("a".into()));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("b".into()));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //     return a + b;
    assert_eq!(next_token(), Token::Return);
    assert_eq!(next_token(), Token::Ident("a".into()));
    assert_eq!(next_token(), Token::Plus);
    assert_eq!(next_token(), Token::Ident("b".into()));
    assert_eq!(next_token(), Token::Semicolon);

    // }
    assert_eq!(next_token(), Token::RBrace);

    // int factorial(int n) {
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("factorial".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("n".into()));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    // int result = 1;
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("result".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(1));
    assert_eq!(next_token(), Token::Semicolon);

    //    for (int i = 1; i <= n; ++i) {
    assert_eq!(next_token(), Token::For);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(1));
    assert_eq!(next_token(), Token::Semicolon);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Le);
    assert_eq!(next_token(), Token::Ident("n".into()));
    assert_eq!(next_token(), Token::Semicolon);
    assert_eq!(next_token(), Token::Increment);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //        result *= i;
    assert_eq!(next_token(), Token::Ident("result".into()));
    assert_eq!(next_token(), Token::AsteriskAssign);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Semicolon);

    // }
    assert_eq!(next_token(), Token::RBrace);

    // return result;
    assert_eq!(next_token(), Token::Return);
    assert_eq!(next_token(), Token::Ident("result".into()));
    assert_eq!(next_token(), Token::Semicolon);

    // }
    assert_eq!(next_token(), Token::RBrace);

    // char to_uppercase(char c) {
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("to_uppercase".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("c".into()));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //     if (c >= 'a' && c <= 'z') {
    assert_eq!(next_token(), Token::If);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("c".into()));
    assert_eq!(next_token(), Token::Ge);
    assert_eq!(next_token(), Token::CharLiteral('a'));
    assert_eq!(next_token(), Token::And);
    assert_eq!(next_token(), Token::Ident("c".into()));
    assert_eq!(next_token(), Token::Le);
    assert_eq!(next_token(), Token::CharLiteral('z'));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //         return c - ('a' - 'A');
    assert_eq!(next_token(), Token::Return);
    assert_eq!(next_token(), Token::Ident("c".into()));
    assert_eq!(next_token(), Token::Minus);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::CharLiteral('a'));
    assert_eq!(next_token(), Token::Minus);
    assert_eq!(next_token(), Token::CharLiteral('A'));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::Semicolon);

    //     }
    assert_eq!(next_token(), Token::RBrace);

    //     return c;
    assert_eq!(next_token(), Token::Return);
    assert_eq!(next_token(), Token::Ident("c".into()));
    assert_eq!(next_token(), Token::Semicolon);

    // }
    assert_eq!(next_token(), Token::RBrace);

    // int main() {
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("main".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //     int x = 42;
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(42));
    assert_eq!(next_token(), Token::Semicolon);

    //     int oct = 007;       // 8진수 리터럴
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("oct".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(7));
    assert_eq!(next_token(), Token::Semicolon);

    //     int *p = &x;         // 주소 연산자
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Asterisk);
    assert_eq!(next_token(), Token::Ident("p".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ampersand);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Semicolon);

    //     char ch1 = 'A';
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("ch1".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::CharLiteral('A'));
    assert_eq!(next_token(), Token::Semicolon);

    //     char ch2 = '\n';     // 이스케이프 시퀀스
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("ch2".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::CharLiteral('\n'));
    assert_eq!(next_token(), Token::Semicolon);

    //     char ch3 = '\0';
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("ch3".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::CharLiteral('\0'));
    assert_eq!(next_token(), Token::Semicolon);

    //     char buf[5] = {'h','e','l','l','o'};  // 고정 크기 배열
    assert_eq!(next_token(), Token::Char);
    assert_eq!(next_token(), Token::Ident("buf".into()));
    assert_eq!(next_token(), Token::LBracket);
    assert_eq!(next_token(), Token::IntLiteral(5));
    assert_eq!(next_token(), Token::RBracket);
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::LBrace);
    assert_eq!(next_token(), Token::CharLiteral('h'));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::CharLiteral('e'));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::CharLiteral('l'));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::CharLiteral('l'));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::CharLiteral('o'));
    assert_eq!(next_token(), Token::RBrace);
    assert_eq!(next_token(), Token::Semicolon);

    //     int arr2[3];
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("arr2".into()));
    assert_eq!(next_token(), Token::LBracket);
    assert_eq!(next_token(), Token::IntLiteral(3));
    assert_eq!(next_token(), Token::RBracket);
    assert_eq!(next_token(), Token::Semicolon);

    //     for (int i = 0; i < 3; i++) {
    assert_eq!(next_token(), Token::For);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(0));
    assert_eq!(next_token(), Token::Semicolon);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Lt);
    assert_eq!(next_token(), Token::IntLiteral(3));
    assert_eq!(next_token(), Token::Semicolon);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Increment);
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //         arr2[i] = i * 2;
    assert_eq!(next_token(), Token::Ident("arr2".into()));
    assert_eq!(next_token(), Token::LBracket);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::RBracket);
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ident("i".into()));
    assert_eq!(next_token(), Token::Asterisk);
    assert_eq!(next_token(), Token::IntLiteral(2));
    assert_eq!(next_token(), Token::Semicolon);

    //     }
    assert_eq!(next_token(), Token::RBrace);

    //    *p += 10;
    assert_eq!(next_token(), Token::Asterisk);
    assert_eq!(next_token(), Token::Ident("p".into()));
    assert_eq!(next_token(), Token::PlusAssign);
    assert_eq!(next_token(), Token::IntLiteral(10));
    assert_eq!(next_token(), Token::Semicolon);

    //    x -= 5;
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::MinusAssign);
    assert_eq!(next_token(), Token::IntLiteral(5));
    assert_eq!(next_token(), Token::Semicolon);

    //    x++;
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Increment);
    assert_eq!(next_token(), Token::Semicolon);
    //    ++x;
    assert_eq!(next_token(), Token::Increment);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Semicolon);
    //    --x;
    assert_eq!(next_token(), Token::Decrement);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Semicolon);
    //    x--
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Decrement);
    assert_eq!(next_token(), Token::Semicolon);

    //     if (x > 10 && x < 100) {
    assert_eq!(next_token(), Token::If);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Gt);
    assert_eq!(next_token(), Token::IntLiteral(10));
    assert_eq!(next_token(), Token::And);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Lt);
    assert_eq!(next_token(), Token::IntLiteral(100));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //         x = add(x, oct);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ident("add".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Comma);
    assert_eq!(next_token(), Token::Ident("oct".into()));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::Semicolon);

    //     } else if (x == 0 || x == -1) {
    assert_eq!(next_token(), Token::RBrace);
    assert_eq!(next_token(), Token::Else);
    assert_eq!(next_token(), Token::If);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Equal);
    assert_eq!(next_token(), Token::IntLiteral(0));
    assert_eq!(next_token(), Token::Or);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Equal);
    assert_eq!(next_token(), Token::Minus);
    assert_eq!(next_token(), Token::IntLiteral(1));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //         x = factorial(5);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ident("factorial".into()));
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::IntLiteral(5));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::Semicolon);

    //     }
    assert_eq!(next_token(), Token::RBrace);

    //     int b_and = x & oct;
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("b_and".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::Ampersand);
    assert_eq!(next_token(), Token::Ident("oct".into()));
    assert_eq!(next_token(), Token::Semicolon);

    //     int b_or  = x | oct;
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("b_or".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::Ident("x".into()));
    assert_eq!(next_token(), Token::BitOr);
    assert_eq!(next_token(), Token::Ident("oct".into()));
    assert_eq!(next_token(), Token::Semicolon);

    //     int count = 0;
    assert_eq!(next_token(), Token::Int);
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::Assign);
    assert_eq!(next_token(), Token::IntLiteral(0));
    assert_eq!(next_token(), Token::Semicolon);

    //     while (count < 5) {
    assert_eq!(next_token(), Token::While);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::Lt);
    assert_eq!(next_token(), Token::IntLiteral(5));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //         if (count == 2) {
    assert_eq!(next_token(), Token::If);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::Equal);
    assert_eq!(next_token(), Token::IntLiteral(2));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    // count++;
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::Increment);
    assert_eq!(next_token(), Token::Semicolon);

    //             continue;
    assert_eq!(next_token(), Token::Continue);
    assert_eq!(next_token(), Token::Semicolon);
    // }
    assert_eq!(next_token(), Token::RBrace);

    //         if (count == 4) {
    assert_eq!(next_token(), Token::If);
    assert_eq!(next_token(), Token::LParen);
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::Equal);
    assert_eq!(next_token(), Token::IntLiteral(4));
    assert_eq!(next_token(), Token::RParen);
    assert_eq!(next_token(), Token::LBrace);

    //             break;
    assert_eq!(next_token(), Token::Break);
    assert_eq!(next_token(), Token::Semicolon);

    assert_eq!(next_token(), Token::RBrace);

    //        count += 1;
    assert_eq!(next_token(), Token::Ident("count".into()));
    assert_eq!(next_token(), Token::PlusAssign);
    assert_eq!(next_token(), Token::IntLiteral(1));
    assert_eq!(next_token(), Token::Semicolon);

    //         }
    assert_eq!(next_token(), Token::RBrace);

    //     return 0;
    assert_eq!(next_token(), Token::Return);
    assert_eq!(next_token(), Token::IntLiteral(0));
    assert_eq!(next_token(), Token::Semicolon);

    // }
    assert_eq!(next_token(), Token::RBrace);

    // End with EOF
    assert_eq!(next_token(), Token::EOF);
}
