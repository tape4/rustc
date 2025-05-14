#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String), // variable or function name
    IntLiteral(i64),
    CharLiteral(char),

    // 단항연산자
    UnaryPrefixOp {
        op: PrefixOp,
        rhs: Box<Expr>,
    },
    UnaryPostfixOp {
        lhs: Box<Expr>,
        op: PostfixOp,
    },

    BinaryOp {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    }, // 이항연산자
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    }, // 함수 호출
    ArrayIndex {
        array: Box<Expr>,
        index: Box<Expr>,
    }, // 인덱싱
    InitializerList(Vec<Expr>), // 배열 초기화 ex) {1, 2, 3}

    Assignment {
        left: Box<Expr>,
        op: AssignOp,
        right: Box<Expr>,
    }, // 할당 x = y, x += 1.
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrefixOp {
    Address, // &
    Deref,   // *
    Neg,     // -
    Not,     // !
    PreInc,  // ++x
    PreDec,  // --x
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostfixOp {
    PostInc, // x++
    PostDec, // x--
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Rem,    // %
    Eq,     // ==
    Ne,     // !=
    Lt,     // <
    Le,     // <=
    Gt,     // >
    Ge,     // >=
    And,    // &&
    Or,     // ||
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
}

#[derive(Clone, Debug, PartialEq)]
pub enum AssignOp {
    Assign,       // =
    PlusAssign,   // +=
    MinusAssign,  // -=
    MulAssign,    // *=
    DivAssign,    // /=
    RemAssign,    // %=
    BitAndAssign, // &=
    BitOrAssign,  // |=
    BitXorAssign, // ^=
}
