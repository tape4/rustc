use crate::utils::parse_expression;
use rustc_tape4::ast::Expr;
use rustc_tape4::ast::expr::{AssignOp, BinaryOp, PostfixOp, PrefixOp};

#[test]
fn test_integer_literal() {
    // 정수 리터럴 테스트
    let expr = parse_expression("42");
    assert_eq!(expr, Expr::IntLiteral(42));
}

#[test]
fn test_char_literal() {
    // 문자 리터럴 테스트
    let expr = parse_expression("'a'");
    assert_eq!(expr, Expr::CharLiteral('a'));

    // 이스케이프 문자 테스트
    let expr = parse_expression("'\\n'");
    assert_eq!(expr, Expr::CharLiteral('\n'));

    let expr = parse_expression("'\\0'");
    assert_eq!(expr, Expr::CharLiteral('\0'));
}

#[test]
fn test_identifier() {
    // 식별자 테스트
    let expr = parse_expression("variable");
    assert_eq!(expr, Expr::Ident("variable".to_string()));
}

#[test]
fn test_binary_arithmetic_ops() {
    // 이항 산술 연산자 테스트
    let expr = parse_expression("1 + 2");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Add,
            ..
        }
    ));

    let expr = parse_expression("3 - 4");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Sub,
            ..
        }
    ));

    let expr = parse_expression("5 * 6");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Mul,
            ..
        }
    ));

    let expr = parse_expression("8 / 2");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Div,
            ..
        }
    ));

    let expr = parse_expression("10 % 3");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Rem,
            ..
        }
    ));
}

#[test]
fn test_comparison_ops() {
    // 비교 연산자 테스트
    let expr = parse_expression("a == b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Eq,
            ..
        }
    ));

    let expr = parse_expression("a != b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Ne,
            ..
        }
    ));

    let expr = parse_expression("a < b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Lt,
            ..
        }
    ));

    let expr = parse_expression("a <= b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Le,
            ..
        }
    ));

    let expr = parse_expression("a > b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Gt,
            ..
        }
    ));

    let expr = parse_expression("a >= b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Ge,
            ..
        }
    ));
}

#[test]
fn test_logical_ops() {
    // 논리 연산자 테스트
    let expr = parse_expression("a && b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::And,
            ..
        }
    ));

    let expr = parse_expression("a || b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Or,
            ..
        }
    ));

    let expr = parse_expression("!a");
    assert!(matches!(
        expr,
        Expr::UnaryPrefixOp {
            op: PrefixOp::Not,
            ..
        }
    ));
}

#[test]
fn test_bitwise_ops() {
    // 비트 연산자 테스트
    let expr = parse_expression("a & b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::BitAnd,
            ..
        }
    ));

    let expr = parse_expression("a | b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::BitOr,
            ..
        }
    ));

    let expr = parse_expression("a ^ b");
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::BitXor,
            ..
        }
    ));
}

#[test]
fn test_assignment_ops() {
    // 할당 연산자 테스트
    let expr = parse_expression("x = 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::Assign,
            ..
        }
    ));

    let expr = parse_expression("x += 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::PlusAssign,
            ..
        }
    ));

    let expr = parse_expression("x -= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::MinusAssign,
            ..
        }
    ));

    let expr = parse_expression("x *= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::MulAssign,
            ..
        }
    ));

    let expr = parse_expression("x /= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::DivAssign,
            ..
        }
    ));

    let expr = parse_expression("x %= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::RemAssign,
            ..
        }
    ));

    let expr = parse_expression("x &= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::BitAndAssign,
            ..
        }
    ));

    let expr = parse_expression("x |= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::BitOrAssign,
            ..
        }
    ));

    let expr = parse_expression("x ^= 5");
    assert!(matches!(
        expr,
        Expr::Assignment {
            op: AssignOp::BitXorAssign,
            ..
        }
    ));
}

#[test]
fn test_increment_decrement() {
    // 증감 연산자 테스트
    let expr = parse_expression("++x");
    assert!(matches!(
        expr,
        Expr::UnaryPrefixOp {
            op: PrefixOp::PreInc,
            ..
        }
    ));

    let expr = parse_expression("--x");
    assert!(matches!(
        expr,
        Expr::UnaryPrefixOp {
            op: PrefixOp::PreDec,
            ..
        }
    ));

    let expr = parse_expression("x++");
    assert!(matches!(
        expr,
        Expr::UnaryPostfixOp {
            op: PostfixOp::PostInc,
            ..
        }
    ));

    let expr = parse_expression("x--");
    assert!(matches!(
        expr,
        Expr::UnaryPostfixOp {
            op: PostfixOp::PostDec,
            ..
        }
    ));
}

#[test]
fn test_function_call() {
    // 함수 호출 테스트
    let expr = parse_expression("foo()");
    assert!(matches!(expr, Expr::Call { .. }));

    let expr = parse_expression("bar(1, 2, 3)");
    if let Expr::Call { args, .. } = expr {
        assert_eq!(args.len(), 3);
    } else {
        panic!("Expected function call");
    }
}

#[test]
fn test_array_indexing() {
    // 배열 인덱싱 테스트
    let expr = parse_expression("arr[0]");
    assert!(matches!(expr, Expr::ArrayIndex { .. }));

    let expr = parse_expression("matrix[i][j]");
    if let Expr::ArrayIndex { array, .. } = expr {
        assert!(matches!(*array, Expr::ArrayIndex { .. }));
    } else {
        panic!("Expected nested array index");
    }
}

#[test]
fn test_pointer_ops() {
    // 포인터 연산자 테스트
    let expr = parse_expression("&x");
    assert!(matches!(
        expr,
        Expr::UnaryPrefixOp {
            op: PrefixOp::Address,
            ..
        }
    ));

    let expr = parse_expression("*p");
    assert!(matches!(
        expr,
        Expr::UnaryPrefixOp {
            op: PrefixOp::Deref,
            ..
        }
    ));
}

#[test]
fn test_function_call_and_precedence() {
    // 함수 호출 및 연산자 우선순위 테스트
    let expr = parse_expression("foo(1, 2+3)");

    if let Expr::Call { func, args } = expr {
        match *func {
            Expr::Ident(name) => assert_eq!(name, "foo"),
            other => panic!("expected Ident, got {:?}", other),
        }
        assert_eq!(args.len(), 2);
        // 두 번째 인자는 2+3
        match &args[1] {
            Expr::BinaryOp {
                op: BinaryOp::Add,
                lhs,
                rhs,
            } => {
                assert_eq!(**lhs, Expr::IntLiteral(2));
                assert_eq!(**rhs, Expr::IntLiteral(3));
            }
            other => panic!("expected 2+3, got {:?}", other),
        }
    } else {
        panic!("expected call expr");
    }
}

#[test]
fn test_array_indexing_and_assignment() {
    // 배열 인덱싱과 대입 표현식 테스트
    let expr = parse_expression("arr[5] = b[2]");

    if let Expr::Assignment { left, op, right } = expr {
        assert_eq!(op, AssignOp::Assign);
        // arr[5]
        match *left {
            Expr::ArrayIndex {
                ref array,
                ref index,
            } => {
                match **array {
                    Expr::Ident(ref name) => assert_eq!(name, "arr"),
                    ref o => panic!("expected arr, got {:?}", o),
                }
                assert_eq!(**index, Expr::IntLiteral(5));
            }
            ref o => panic!("expected array index lhs, got {:?}", o),
        }
        // b[2]
        match *right {
            Expr::ArrayIndex {
                ref array,
                ref index,
            } => {
                match **array {
                    Expr::Ident(ref name) => assert_eq!(name, "b"),
                    ref o => panic!("expected b, got {:?}", o),
                }
                assert_eq!(**index, Expr::IntLiteral(2));
            }
            ref o => panic!("expected array index rhs, got {:?}", o),
        }
    } else {
        panic!("expected assignment expr");
    }
}

#[test]
fn test_complex_expression() {
    // 복합 표현식과 우선순위 테스트
    let expr = parse_expression("(a + b) * (c - d) / 2");

    if let Expr::BinaryOp {
        op: BinaryOp::Div,
        lhs,
        rhs,
    } = expr
    {
        assert_eq!(*rhs, Expr::IntLiteral(2));

        if let Expr::BinaryOp {
            op: BinaryOp::Mul,
            lhs: mul_lhs,
            rhs: mul_rhs,
        } = *lhs
        {
            // (a + b)
            if let Expr::BinaryOp {
                op: BinaryOp::Add,
                lhs: add_lhs,
                rhs: add_rhs,
            } = *mul_lhs
            {
                assert_eq!(*add_lhs, Expr::Ident("a".to_string()));
                assert_eq!(*add_rhs, Expr::Ident("b".to_string()));
            } else {
                panic!("Expected (a + b)");
            }

            // (c - d)
            if let Expr::BinaryOp {
                op: BinaryOp::Sub,
                lhs: sub_lhs,
                rhs: sub_rhs,
            } = *mul_rhs
            {
                assert_eq!(*sub_lhs, Expr::Ident("c".to_string()));
                assert_eq!(*sub_rhs, Expr::Ident("d".to_string()));
            } else {
                panic!("Expected (c - d)");
            }
        } else {
            panic!("Expected (a + b) * (c - d)");
        }
    } else {
        panic!("Expected ((a + b) * (c - d)) / 2");
    }
}

#[test]
fn test_initializer_list() {
    // 배열 초기화 리스트 테스트
    let expr = parse_expression("{1, 2, 3}");

    if let Expr::InitializerList(items) = expr {
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], Expr::IntLiteral(1));
        assert_eq!(items[1], Expr::IntLiteral(2));
        assert_eq!(items[2], Expr::IntLiteral(3));
    } else {
        panic!("Expected initializer list");
    }

    // 빈 초기화 리스트
    let expr = parse_expression("{}");
    assert!(matches!(expr, Expr::InitializerList(items) if items.is_empty()));

    // 중첩 초기화 리스트
    let expr = parse_expression("{{1, 2}, {3, 4}}");
    if let Expr::InitializerList(items) = expr {
        assert_eq!(items.len(), 2);
        assert!(matches!(&items[0], Expr::InitializerList(inner) if inner.len() == 2));
        assert!(matches!(&items[1], Expr::InitializerList(inner) if inner.len() == 2));
    } else {
        panic!("Expected nested initializer list");
    }
}

#[test]
fn test_multiple_assignment() {
    // 다중 할당 표현식 테스트 (a = b = c = 5)
    let expr = parse_expression("a = b = c = 5");

    if let Expr::Assignment {
        left: left1,
        op: op1,
        right: right1,
    } = expr
    {
        assert_eq!(*left1, Expr::Ident("a".to_string()));
        assert_eq!(op1, AssignOp::Assign);

        if let Expr::Assignment {
            left: left2,
            op: op2,
            right: right2,
        } = *right1
        {
            assert_eq!(*left2, Expr::Ident("b".to_string()));
            assert_eq!(op2, AssignOp::Assign);

            if let Expr::Assignment {
                left: left3,
                op: op3,
                right: right3,
            } = *right2
            {
                assert_eq!(*left3, Expr::Ident("c".to_string()));
                assert_eq!(op3, AssignOp::Assign);
                assert_eq!(*right3, Expr::IntLiteral(5));
            } else {
                panic!("Expected c = 5");
            }
        } else {
            panic!("Expected b = c = 5");
        }
    } else {
        panic!("Expected a = b = c = 5");
    }
}

#[test]
fn test_nested_pointer_ops() {
    // 중첩된 포인터 연산자 테스트
    let expr = parse_expression("**ptr");

    if let Expr::UnaryPrefixOp {
        op: PrefixOp::Deref,
        rhs,
    } = &expr
    {
        assert!(matches!(
            **rhs,
            Expr::UnaryPrefixOp {
                op: PrefixOp::Deref,
                ..
            }
        ));
    } else {
        panic!("Expected nested pointer dereference");
    }

    let expr = parse_expression("&*p");

    if let Expr::UnaryPrefixOp {
        op: PrefixOp::Address,
        rhs,
    } = &expr
    {
        assert!(matches!(
            **rhs,
            Expr::UnaryPrefixOp {
                op: PrefixOp::Deref,
                ..
            }
        ));
    } else {
        panic!("Expected address-of dereference");
    }
}

#[test]
fn test_complex_pointer_arithmetic() {
    // 포인터 산술 및 배열 인덱싱 조합
    let expr = parse_expression("*(arr + i)");

    if let Expr::UnaryPrefixOp {
        op: PrefixOp::Deref,
        rhs,
    } = &expr
    {
        assert!(matches!(
            **rhs,
            Expr::BinaryOp {
                op: BinaryOp::Add,
                ..
            }
        ));
    } else {
        panic!("Expected dereference of addition");
    }
}

#[test]
fn test_op_precedence_mix() {
    // 다양한 연산자 우선순위 혼합 테스트
    let expr = parse_expression("a + b * c || d && e == f");

    // 우선순위: || < && < == < + < *
    // 따라서 구조는 ((a + (b * c)) || (d && (e == f)))
    assert!(matches!(
        expr,
        Expr::BinaryOp {
            op: BinaryOp::Or,
            ..
        }
    ));
}

#[test]
fn test_array_with_expr_index() {
    // 복잡한 표현식을 배열 인덱스로 사용
    let expr = parse_expression("arr[i * 2 + 1]");

    if let Expr::ArrayIndex { array: _, index } = expr {
        assert!(matches!(
            *index,
            Expr::BinaryOp {
                op: BinaryOp::Add,
                ..
            }
        ));
    } else {
        panic!("Expected array index");
    }
}

#[test]
fn test_nested_function_call() {
    // 중첩된 함수 호출
    let expr = parse_expression("foo(bar(1, 2), 3)");

    if let Expr::Call { args, .. } = expr {
        assert_eq!(args.len(), 2);
        assert!(matches!(args[0], Expr::Call { .. }));
    } else {
        panic!("Expected function call");
    }
}
#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_unbalanced_parentheses() {
    // 괄호 짝이 맞지 않는 표현식
    parse_expression("(a + b");
}

#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_missing_operand() {
    // 피연산자가 없는 이항 연산자
    parse_expression("a + ");
}

#[test]
fn test_consecutive_operators() {
    // 연속된 연산자
    parse_expression("a ++ + b");
}

#[test]
#[should_panic]
fn test_invalid_assignment_target() {
    // 유효하지 않은 대입 대상 (좌변에는 lvalue가 와야 함)
    parse_expression("a + b = c");
}

#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_invalid_array_index() {
    // 배열 인덱스로 닫는 괄호가 없음
    parse_expression("arr[5");
}

#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_function_call_without_closing_paren() {
    // 함수 호출 닫는 괄호 없음
    parse_expression("foo(1, 2");
}

#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_invalid_initializer_list() {
    // 초기화 리스트에 닫는 괄호 없음
    parse_expression("{1, 2, 3");
}

#[test]
#[should_panic(expected = "UnexpectedToken")]
fn test_comma_without_args() {
    // 인자 없이 쉼표만 있는 함수 호출
    parse_expression("foo(,)");
}

#[test]
fn test_binary_op_without_lhs() {
    // 좌측 피연산자 없는 이항 연산자
    parse_expression("* b");
}

#[test]
#[should_panic(expected = "UnexpectedEOF")]
fn test_empty_expression() {
    // 빈 표현식
    parse_expression("");
}
