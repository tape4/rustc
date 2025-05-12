use crate::ast::TypeSpecifier;
use crate::ast::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Block),
    If {
        cond: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    For {
        init: Option<Expr>,
        cond: Option<Expr>,
        step: Option<Expr>,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Declaration {
        ty: TypeSpecifier,
        name: String,
        init: Option<Expr>,
    },
    ExprStmt(Expr),
}

// 문장: 구문들
// statement             ::= block
// | if_statement
// | while_statement
// | for_statement
// | return_statement
// | break_statement
// | continue_statement
// | declaration_statement
// | expression_statement
