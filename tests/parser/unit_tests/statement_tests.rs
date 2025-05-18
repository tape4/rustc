use crate::utils::parse_statement;
use rustc_tape4::ast::expr::BinaryOp;
use rustc_tape4::ast::{Expr, Stmt, TypeSpecifier};

#[test]
fn test_empty_statement() {
    // 세미콜론만 있는 빈 문장 테스트
    let stmt = parse_statement(";");
    assert!(matches!(stmt, Stmt::ExprStmt(None)));
}

#[test]
fn test_expression_statement() {
    // 표현식 문장 테스트
    let stmt = parse_statement("x = 42;");

    if let Stmt::ExprStmt(Some(expr)) = stmt {
        assert!(matches!(expr, Expr::Assignment { .. }));
    } else {
        panic!("Expected expression statement");
    }

    // 함수 호출 문장
    let stmt = parse_statement("foo(1, 2);");

    if let Stmt::ExprStmt(Some(Expr::Call { .. })) = stmt {
        // OK
    } else {
        panic!("Expected function call statement");
    }
}

#[test]
fn test_declaration_statement() {
    // 기본 변수 선언
    let stmt = parse_statement("int x = 10;");

    if let Stmt::Declaration { ty, declarators } = stmt {
        assert_eq!(ty, TypeSpecifier::Int);
        assert_eq!(declarators.len(), 1);
        assert_eq!(declarators[0].name, "x");
        assert!(matches!(&declarators[0].init, Some(Expr::IntLiteral(10))));
    } else {
        panic!("Expected declaration statement");
    }

    // 초기화 없는 선언
    let stmt = parse_statement("char c;");

    if let Stmt::Declaration { ty, declarators } = stmt {
        assert_eq!(ty, TypeSpecifier::Char);
        assert_eq!(declarators[0].name, "c");
        assert!(declarators[0].init.is_none());
    } else {
        panic!("Expected declaration statement");
    }

    // 포인터 변수 선언
    let stmt = parse_statement("int* ptr;");

    if let Stmt::Declaration { ty, .. } = stmt {
        if let TypeSpecifier::Pointer(inner) = ty {
            assert_eq!(*inner, TypeSpecifier::Int);
        } else {
            panic!("Expected pointer type");
        }
    } else {
        panic!("Expected declaration statement");
    }
}

#[test]
fn test_multiple_declarators_in_declaration() {
    // 한 선언문에 여러 변수 선언 테스트
    let stmt = parse_statement("int x = 1, y, z = 3;");

    if let Stmt::Declaration { ty, declarators } = stmt {
        assert_eq!(ty, TypeSpecifier::Int);
        assert_eq!(declarators.len(), 3);

        // x = 1
        assert_eq!(declarators[0].name, "x");
        assert!(matches!(&declarators[0].init, Some(Expr::IntLiteral(1))));

        // y (초기화 없음)
        assert_eq!(declarators[1].name, "y");
        assert!(declarators[1].init.is_none());

        // z = 3
        assert_eq!(declarators[2].name, "z");
        assert!(matches!(&declarators[2].init, Some(Expr::IntLiteral(3))));
    } else {
        panic!("Expected declaration with multiple declarators");
    }
}

#[test]
fn test_array_declaration() {
    // 배열 선언 테스트
    let stmt = parse_statement("int arr[5];");

    if let Stmt::Declaration { ty, declarators } = stmt {
        assert_eq!(ty, TypeSpecifier::Int);
        assert_eq!(declarators[0].name, "arr");
        assert_eq!(declarators[0].array_size, Some(5));
    } else {
        panic!("Expected array declaration");
    }

    // 배열 초기화 테스트
    let stmt = parse_statement("int nums[3] = {1, 2, 3};");

    if let Stmt::Declaration { ty, declarators } = stmt {
        assert_eq!(ty, TypeSpecifier::Int);
        assert_eq!(declarators[0].name, "nums");
        assert_eq!(declarators[0].array_size, Some(3));

        if let Some(Expr::InitializerList(items)) = &declarators[0].init {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Expr::IntLiteral(1));
            assert_eq!(items[1], Expr::IntLiteral(2));
            assert_eq!(items[2], Expr::IntLiteral(3));
        } else {
            panic!("Expected initializer list");
        }
    } else {
        panic!("Expected array declaration with initialization");
    }
}

#[test]
fn test_initializer_list_trailing_comma() {
    // 배열 초기화 리스트 후행 콤마 테스트
    let stmt = parse_statement("int a[3] = {1, 2, 3,};");

    if let Stmt::Declaration { declarators, .. } = stmt {
        let d = &declarators[0];
        assert_eq!(d.name, "a");
        assert_eq!(d.array_size, Some(3));

        if let Some(Expr::InitializerList(v)) = &d.init {
            assert_eq!(
                v,
                &vec![
                    Expr::IntLiteral(1),
                    Expr::IntLiteral(2),
                    Expr::IntLiteral(3),
                ]
            );
        } else {
            panic!("Expected initializer list");
        }
    } else {
        panic!("Expected declaration statement");
    }
}

#[test]
fn test_if_statement() {
    // 기본 if 문 테스트
    let stmt = parse_statement("if (x > 0) { return x; }");

    if let Stmt::If {
        cond,
        then_branch,
        else_branch,
    } = stmt
    {
        assert!(matches!(
            cond,
            Expr::BinaryOp {
                op: BinaryOp::Gt,
                ..
            }
        ));
        assert!(else_branch.is_none());

        if let Stmt::Block(block) = *then_branch {
            assert_eq!(block.statements.len(), 1);
            assert!(matches!(block.statements[0], Stmt::Return(..)));
        } else {
            panic!("Expected block statement");
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_if_else_statement() {
    // if-else 문 테스트
    let stmt = parse_statement("if (x > 0) return x; else return -x;");

    if let Stmt::If {
        cond,
        then_branch,
        else_branch,
    } = stmt
    {
        assert!(matches!(
            cond,
            Expr::BinaryOp {
                op: BinaryOp::Gt,
                ..
            }
        ));

        assert!(matches!(*then_branch, Stmt::Return(..)));

        assert!(else_branch.is_some());
        if let Some(else_stmt) = else_branch {
            assert!(matches!(*else_stmt, Stmt::Return(..)));
        }
    } else {
        panic!("Expected if-else statement");
    }
}

#[test]
fn test_if_else_if_chain() {
    // if-else if-else 체인 테스트
    let stmt = parse_statement("if (x > 0) return 1; else if (x < 0) return -1; else return 0;");

    if let Stmt::If {
        cond: _,
        then_branch: _,
        else_branch: Some(else_stmt),
    } = &stmt
    {
        if let Stmt::If {
            cond: _,
            then_branch: _,
            else_branch: Some(inner_else),
        } = &**else_stmt
        {
            assert!(matches!(**inner_else, Stmt::Return(..)));
        } else {
            panic!("Expected else-if");
        }
    } else {
        panic!("Expected if-else if-else chain");
    }
}

#[test]
fn test_while_statement() {
    // while 문 테스트
    let stmt = parse_statement("while (i < 10) { i = i + 1; }");

    if let Stmt::While { cond, body } = stmt {
        assert!(matches!(
            cond,
            Expr::BinaryOp {
                op: BinaryOp::Lt,
                ..
            }
        ));

        if let Stmt::Block(block) = *body {
            assert_eq!(block.statements.len(), 1);
            assert!(matches!(block.statements[0], Stmt::ExprStmt(..)));
        } else {
            panic!("Expected block statement");
        }
    } else {
        panic!("Expected while statement");
    }
}

#[test]
fn test_for_statement() {
    // 기본 for 문 테스트
    let stmt = parse_statement("for (i = 0; i < 10; i++) sum += i;");

    if let Stmt::For {
        init,
        cond,
        step,
        body,
    } = stmt
    {
        assert!(init.is_some());
        assert!(cond.is_some());
        assert!(step.is_some());

        if let Some(init_expr) = init {
            assert!(matches!(
                *init_expr,
                Stmt::ExprStmt(Some(Expr::Assignment { .. }))
            ));
        }

        if let Some(cond_expr) = cond {
            assert!(matches!(
                cond_expr,
                Expr::BinaryOp {
                    op: BinaryOp::Lt,
                    ..
                }
            ));
        }

        if let Some(step_expr) = step {
            assert!(matches!(step_expr, Expr::UnaryPostfixOp { .. }));
        }

        assert!(matches!(*body, Stmt::ExprStmt(..)));
    } else {
        panic!("Expected for statement");
    }
}

#[test]
fn test_various_for_forms() {
    // 다양한 for 문 형태 테스트

    // 초기화 없는 for
    let stmt = parse_statement("for (; i < 10; i++) { }");
    if let Stmt::For { init, .. } = stmt {
        assert!(init.is_none());
    } else {
        panic!("Expected for without init");
    }

    // 조건 없는 for
    let stmt = parse_statement("for (i = 0;; i++) { }");
    if let Stmt::For { cond, .. } = stmt {
        assert!(cond.is_none());
    } else {
        panic!("Expected for without condition");
    }

    // 증감 없는 for
    let stmt = parse_statement("for (i = 0; i < 10;) { }");
    if let Stmt::For { step, .. } = stmt {
        assert!(step.is_none());
    } else {
        panic!("Expected for without step");
    }

    // 모두 없는 for (무한 루프)
    let stmt = parse_statement("for (;;) { }");
    if let Stmt::For {
        init, cond, step, ..
    } = stmt
    {
        assert!(init.is_none());
        assert!(cond.is_none());
        assert!(step.is_none());
    } else {
        panic!("Expected infinite for loop");
    }
}

#[test]
fn test_return_statement() {
    // 값 없는 return 문
    let stmt = parse_statement("return;");

    if let Stmt::Return(expr) = stmt {
        assert!(expr.is_none());
    } else {
        panic!("Expected return statement without value");
    }

    // 값 있는 return 문
    let stmt = parse_statement("return 42;");

    if let Stmt::Return(Some(expr)) = stmt {
        assert_eq!(expr, Expr::IntLiteral(42));
    } else {
        panic!("Expected return statement with value");
    }

    // 표현식 return 문
    let stmt = parse_statement("return a + b * c;");

    if let Stmt::Return(Some(expr)) = stmt {
        assert!(matches!(expr, Expr::BinaryOp { .. }));
    } else {
        panic!("Expected return statement with expression");
    }
}

#[test]
fn test_break_and_continue() {
    // break 문
    let stmt = parse_statement("break;");
    assert!(matches!(stmt, Stmt::Break));

    // continue 문
    let stmt = parse_statement("continue;");
    assert!(matches!(stmt, Stmt::Continue));
}

#[test]
fn test_block_statement() {
    // 단일 문장 블록
    let stmt = parse_statement("{ return 0; }");

    if let Stmt::Block(block) = stmt {
        assert_eq!(block.statements.len(), 1);
        assert!(matches!(block.statements[0], Stmt::Return(..)));
    } else {
        panic!("Expected block statement");
    }

    // 다중 문장 블록
    let stmt = parse_statement("{ int x = 10; x = x + 1; return x; }");

    if let Stmt::Block(block) = stmt {
        assert_eq!(block.statements.len(), 3);
        assert!(matches!(block.statements[0], Stmt::Declaration { .. }));
        assert!(matches!(block.statements[1], Stmt::ExprStmt(..)));
        assert!(matches!(block.statements[2], Stmt::Return(..)));
    } else {
        panic!("Expected multi-statement block");
    }

    // 빈 블록
    let stmt = parse_statement("{ }");

    if let Stmt::Block(block) = stmt {
        assert_eq!(block.statements.len(), 0);
    } else {
        panic!("Expected empty block statement");
    }
}

#[test]
fn test_nested_blocks() {
    // 중첩 블록 테스트
    let stmt = parse_statement("{ { int x = 1; } { int y = 2; } }");

    if let Stmt::Block(block) = stmt {
        assert_eq!(block.statements.len(), 2);
        assert!(matches!(block.statements[0], Stmt::Block(..)));
        assert!(matches!(block.statements[1], Stmt::Block(..)));
    } else {
        panic!("Expected nested blocks");
    }
}

#[test]
fn test_complex_nested_statements() {
    // 여러 문장 타입이 중첩된 복잡한 예제
    let stmt = parse_statement(
        r#"
    {
        int i = 0;
        int sum = 0;
        while (i < 10) {
            if (i % 2 == 0) {
                sum += i;
            } else {
                continue;
            }
            i++;
        }
        return sum;
    }
    "#,
    );

    if let Stmt::Block(block) = stmt {
        assert_eq!(block.statements.len(), 4); // 선언 2개, while 문, return 문

        // while 문 체크
        if let Stmt::While { body, .. } = &block.statements[2] {
            if let Stmt::Block(while_block) = &**body {
                assert_eq!(while_block.statements.len(), 2); // if 문, i++ 문

                // if 문 체크
                if let Stmt::If { else_branch, .. } = &while_block.statements[0] {
                    assert!(else_branch.is_some());
                } else {
                    panic!("Expected if statement in while block");
                }
            } else {
                panic!("Expected block as while body");
            }
        } else {
            panic!("Expected while statement");
        }

        // return 문 체크
        assert!(matches!(block.statements[3], Stmt::Return(..)));
    } else {
        panic!("Expected complex nested block");
    }
}
