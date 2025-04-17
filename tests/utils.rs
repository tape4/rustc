use rustc_tape4::lexer::{Lexer, Token};

/// 입력 전체를 순환하며 토큰을 수집
pub fn collect_tokens(input: &str) -> Vec<Token> {
    let mut l = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let tok = l.next_token();
        tokens.push(tok.clone());
        if tok == Token::EOF {
            break;
        }
    }
    tokens
}
