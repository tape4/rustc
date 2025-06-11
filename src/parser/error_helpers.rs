use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::error::ParserError;
use crate::parser::parser::ParseResult;
// #[macro_export]
// macro_rules! expect {
//     ($p:expr, $pat:pat_param) => {{
//         let tok = $p.current_token().clone();
//         match tok {
//             Token::EOF => {
//                 return $p.unexpected_eof(concat!("expected ", stringify!($pat)));
//             }
//             $pat => {
//                 $p.next_token();
//                 Ok(())
//             }
//             other => {
//                 return $p.unexpected_token(other);
//             }
//         }
//     }};
// }

impl Parser {
    /// 더 읽을 토큰이 없어서 EOF 만난 경우
    pub fn unexpected_eof<T>(&self, expected: impl Into<String>) -> ParseResult<T> {
        let (line, col) = self.current_span();
        Err(ParserError::UnexpectedEOF {
            expected: expected.into(),
            line,
            col,
        })
    }

    /// 기대한 토큰이 오지 않았을 때
    pub fn unexpected_token<T>(&self, expected: Token) -> ParseResult<T> {
        let (line, col) = self.current_span();
        Err(ParserError::UnexpectedToken {
            expected,
            found: self.current_token().clone(),
            line,
            col,
        })
    }

    /// 아예 지원하지 않는 토큰을 만난 경우
    pub fn unsupported_token<T>(&self) -> ParseResult<T> {
        let (line, col) = self.current_span();
        Err(ParserError::UnsupportedToken {
            found: self.current_token().clone(),
            line,
            col,
        })
    }
}
