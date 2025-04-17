use super::token::{LexError, Token, lookup_ident};

/// 렉서 구조
pub struct Lexer {
    input: Vec<char>, // full codes
    pos: usize,       // current position
    read_pos: usize,  // next reading position
    ch: Option<char>, // current char
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            pos: 0,
            read_pos: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    /// 다음 문자로 넘어가기
    fn read_char(&mut self) {
        self.ch = if self.read_pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_pos])
        };
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    /// 다음 문자 보기 (Consume) 없이
    fn peek_char(&self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_pos])
        }
    }

    /// 주석, 빈칸 건너뛰기
    fn skip_trivia(&mut self) {
        loop {
            match self.ch {
                // Skip plain whitespace
                Some(c) if c.is_whitespace() => {
                    self.read_char();
                }

                // 단일 줄 주석: // ...
                Some('/') if self.peek_char() == Some('/') => {
                    self.read_char();
                    self.read_char();

                    while let Some(ch) = self.ch {
                        if ch == '\n' {
                            break;
                        }
                        self.read_char();
                    }
                }

                // 여러 줄 주석: /* ... */
                Some('/') if self.peek_char() == Some('*') => {
                    self.read_char();
                    self.read_char();

                    while let Some(ch) = self.ch {
                        if ch == '*' && self.peek_char() == Some('/') {
                            // consume '*/'
                            self.read_char();
                            self.read_char();
                            break;
                        }
                        self.read_char();
                    }
                }

                _ => break,
            }
        }
    }

    /// 다음 토큰 얻기
    pub fn next_token(&mut self) -> Token {
        self.skip_trivia();
        let tok = match self.ch {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('%') => Token::Percent,

            Some('=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            Some('<') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token::Ge
                } else {
                    Token::Gt
                }
            }

            Some('&') => {
                if self.peek_char() == Some('&') {
                    self.read_char();
                    Token::And
                } else {
                    Token::Ampersand
                }
            }
            Some('|') => {
                if self.peek_char() == Some('|') {
                    self.read_char();
                    Token::Or
                } else {
                    Token::BitOr
                }
            }

            Some(';') => Token::Semicolon,
            Some(',') => Token::Comma,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,

            Some('\'') => match self.read_char_literal() {
                Ok(ch) => return Token::CharLiteral(ch),
                Err(e) => return Token::Error(e),
            },

            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier();
                return lookup_ident(&ident);
            }

            Some(c) if c.is_numeric() => match self.read_number() {
                Ok(num) => return Token::IntLiteral(num),
                Err(e) => return Token::Error(e),
            },

            None => Token::EOF,
            Some(c) => Token::Illegal(c),
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while let Some(c) = self.ch {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.read_char()
        }
        self.input[start..self.pos].iter().collect()
    }

    fn read_number(&mut self) -> Result<i64, LexError> {
        let start = self.pos;
        while let Some(c) = self.ch {
            if !c.is_numeric() {
                break;
            }
            self.read_char()
        }

        let lit: String = self.input[start..self.pos].iter().collect();
        let value = lit
            .parse::<i64>()
            .map_err(|_| LexError::InvalidNumericLiteral(lit))?;
        Ok(value)
    }

    fn read_char_literal(&mut self) -> Result<char, LexError> {
        // current ch == Some(') 일때,
        self.read_char();

        let lit = match self.ch {
            Some('\\') => {
                self.read_char();
                match self.ch {
                    Some('0') => '\0',
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('\\') => '\\',
                    Some('\'') => '\'',
                    _ => self.ch.unwrap_or('\0'),
                }
            }
            Some(c) => c,
            None => '\0',
        };
        // 리터럴 문자 소비
        self.read_char();

        if self.ch != Some('\'') {
            return Err(LexError::UnterminatedCharLiteral);
        }
        // 닫는 따옴표 소비
        self.read_char();
        Ok(lit)
    }
}
