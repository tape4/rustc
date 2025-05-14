use crate::ast::Expr;
use crate::ast::expr::{AssignOp, BinaryOp, PostfixOp, PrefixOp};
use crate::lexer::Token;
use crate::parser::Parser;
use crate::parser::parser::ParseResult;

impl Parser {
    pub fn parse_expr(&mut self) -> ParseResult<Expr> {
        self.parse_assignment()
    }

    // assignment ::= logical_or ( ( "="  | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" ) assignment )?
    fn parse_assignment(&mut self) -> ParseResult<Expr> {
        let mut lhs = self.parse_logical_or()?;
        let op = match self.current_token() {
            Token::Assign => AssignOp::Assign,
            Token::PlusAssign => AssignOp::PlusAssign,
            Token::MinusAssign => AssignOp::MinusAssign,
            Token::AsteriskAssign => AssignOp::MulAssign,
            Token::SlashAssign => AssignOp::DivAssign,
            Token::ModuloAssign => AssignOp::RemAssign,
            Token::BitAndAssign => AssignOp::BitAndAssign,
            Token::BitOrAssign => AssignOp::BitOrAssign,
            Token::BitXorAssign => AssignOp::BitXorAssign,
            _ => {
                // 할당 연산자가 아니면 그대로 lhs 리턴
                return Ok(lhs);
            }
        };

        // 할당 연산자 소비
        self.next_token();
        let rhs = self.parse_assignment()?;

        lhs = Expr::Assignment {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        };
        Ok(lhs)
    }

    // unary ::= ( "!" | "-" | "&" | "*" | "++" | "--" ) unary | postfix
    fn parse_unary(&mut self) -> ParseResult<Expr> {
        let op = match self.current_token() {
            Token::Not => PrefixOp::Not,
            Token::Minus => PrefixOp::Neg,
            Token::Ampersand => PrefixOp::Address,
            Token::Asterisk => PrefixOp::Deref,
            Token::Increment => PrefixOp::PreInc,
            Token::Decrement => PrefixOp::PreDec,
            _ => return self.parse_postfix(),
        };
        self.next_token();
        let rhs = self.parse_unary()?;
        Ok(Expr::UnaryPrefixOp {
            op,
            rhs: Box::new(rhs),
        })
    }

    // multiplicative ::= unary ( ( "*" | "/" | "%" ) unary )*
    fn parse_multiplicative(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_unary()?;

        loop {
            let op = match self.current_token() {
                Token::Asterisk => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Rem,
                _ => break,
            };
            self.next_token(); // 연산자소비
            let rhs = self.parse_unary()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    // additive ::= multiplicative ( ( "+" | "-" ) multiplicative )*
    fn parse_additive(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.next_token(); // 연산자소비
            let rhs = self.parse_multiplicative()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    // relational ::= additive ( ( "<" | "<=" | ">" | ">=" ) additive )*
    fn parse_relational(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_additive()?;

        loop {
            let op = match self.current_token() {
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                _ => break,
            };
            self.next_token(); // 연산자소비
            let rhs = self.parse_additive()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    // equality ::= relational ( ( "==" | "!=" ) relational )*
    fn parse_equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_relational()?;
        loop {
            let op = match self.current_token() {
                Token::Equal => BinaryOp::Eq,
                Token::NotEqual => BinaryOp::Ne,
                _ => break,
            };
            self.next_token(); // 연산자 소비
            let rhs = self.parse_relational()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    // logical_and ::= equality ( "&&" equality )*
    fn parse_logical_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_equality()?;

        while self.current_token() == &Token::And {
            self.next_token(); // '&&' 소비
            let rhs = self.parse_equality()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op: BinaryOp::And,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    // logical_or ::= logical_and ( "||" logical_and )*
    fn parse_logical_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_logical_and()?;

        while self.current_token() == &Token::Or {
            self.next_token(); // '||' 소비
            let rhs = self.parse_logical_and()?;
            expr = Expr::BinaryOp {
                lhs: Box::new(expr),
                op: BinaryOp::Or,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// primary + (postfix_op)*
    /// postfix_op ::= "(" argument_list? ")"   (함수 호출)
    ///              | "[" expression "]"        (배열 인덱싱)
    ///              | "++"                       (후위 증가)
    ///              | "--"                       (후위 감소)
    fn parse_postfix(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary()?;

        loop {
            expr = match self.current_token() {
                Token::LParen => {
                    self.next_token(); // '('
                    let args = if self.current_token() != &Token::RParen {
                        let mut v = Vec::new();
                        loop {
                            v.push(self.parse_expr()?);
                            if self.current_token() == &Token::Comma {
                                self.next_token();
                                continue;
                            }
                            break;
                        }
                        v
                    } else {
                        Vec::new()
                    };
                    self.expect(Token::RParen)?;
                    Expr::Call {
                        func: Box::new(expr),
                        args,
                    }
                }

                Token::LBracket => {
                    self.next_token(); // '['
                    let idx = self.parse_expr()?;
                    self.expect(Token::RBracket)?; // ']'
                    Expr::ArrayIndex {
                        array: Box::new(expr),
                        index: Box::new(idx),
                    }
                }

                Token::Increment => {
                    self.next_token(); // 후위 ++
                    Expr::UnaryPostfixOp {
                        lhs: Box::new(expr),
                        op: PostfixOp::PostInc,
                    }
                }

                Token::Decrement => {
                    self.next_token(); // 후위 --
                    Expr::UnaryPostfixOp {
                        lhs: Box::new(expr),
                        op: PostfixOp::PostDec,
                    }
                }
                _ => break,
            };
        }

        Ok(expr)
    }

    // primary ::= identifier | int_literal | char_literal | "(" expression ")"
    fn parse_primary(&mut self) -> ParseResult<Expr> {
        let expr = match self.current_token() {
            Token::Ident(_) => Expr::Ident(self.expect_ident()?),
            Token::IntLiteral(_) => Expr::IntLiteral(self.expect_int_literal()?),
            Token::CharLiteral(_) => Expr::CharLiteral(self.expect_char_literal()?),
            Token::LParen => {
                self.next_token(); // '('
                let e = self.parse_expr()?;
                self.expect(Token::RParen)?;
                e
            }
            _ => return self.unsupported_token(),
        };
        Ok(expr)
    }

    // initializer ::= expression | "{" initializer_list? "}"
    fn parse_initializer(&mut self) -> ParseResult<Expr> {
        if self.current_token() == &Token::LBrace {
            self.parse_initializer_list()
        } else {
            self.parse_assignment()
        }
    }

    // initializer_list ::= initializer ( "," initializer )* ","?
    fn parse_initializer_list(&mut self) -> ParseResult<Expr> {
        self.expect(Token::LBrace)?; // '{' 소비
        let mut exprs = Vec::new();

        if self.current_token() != &Token::RBrace {
            loop {
                exprs.push(self.parse_initializer()?);

                // ',' 가 있다면 소비
                if self.current_token() == &Token::Comma {
                    self.next_token();

                    // ',' 다음에 '}' 이면 종료
                    if self.current_token() == &Token::RBrace {
                        break;
                    }
                    continue;
                }
                // ',' 가 없다면 종료
                break;
            }
        }

        self.expect(Token::RBrace)?; // '}' 소비
        Ok(Expr::InitializerList(exprs))
    }
    fn parse_char_literal(&mut self) -> ParseResult<Expr> {
        let value = self.expect_char_literal()?;
        Ok(Expr::CharLiteral(value))
    }

    fn parse_int_literal(&mut self) -> ParseResult<Expr> {
        let value = self.expect_int_literal()?;
        Ok(Expr::IntLiteral(value))
    }

    fn parse_identifier(&mut self) -> ParseResult<Expr> {
        let string = self.expect_ident()?;
        Ok(Expr::Ident(string))
    }
}
