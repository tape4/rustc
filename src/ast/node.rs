use crate::ast::Expr;
use crate::ast::program::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<T> {
    pub span: Span,
    pub node: T,
}

pub type ExprNode = Node<Expr>;