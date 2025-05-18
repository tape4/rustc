use crate::utils::parse_program;
use rustc_tape4::ast::expr::{AssignOp, BinaryOp};
use rustc_tape4::ast::{Expr, Stmt, TypeSpecifier};

#[test]
fn test_empty_program() {
    // 함수 정의가 하나도 없으면 빈 Program 반환
    let input = "";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 0);
}

#[test]
fn test_empty_body() {
    // 문장 없는 함수
    let program = parse_program("int f() {}").unwrap();
    assert!(program.functions[0].body.statements.is_empty());
}

#[test]
fn test_void_return_type() {
    // void 리턴 타입 함수 정의 테스트 (매개변수 없음, 빈 본문)
    let input = "void do_nothing() { }";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];
    assert_eq!(func.name, "do_nothing");
    assert_eq!(func.return_ty, TypeSpecifier::Void);
    assert_eq!(func.params.len(), 0);
    assert_eq!(func.body.statements.len(), 0);
}

#[test]
fn test_simple_function() {
    // 기본 함수 정의 테스트 (매개변수 없음)
    let input = "int main() { return 0; }";
    let program = parse_program(input).unwrap();

    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];

    // 함수 이름과 반환 타입 검증
    assert_eq!(func.name, "main");
    assert_eq!(func.return_ty, TypeSpecifier::Int);

    // 매개변수 없음 검증
    assert_eq!(func.params.len(), 0);

    // 함수 본문이 한 개의 문장을 가지고 있는지 검증
    assert_eq!(func.body.statements.len(), 1);
}

#[test]
fn test_function_with_parameters() {
    // 매개변수가 있는 함수 정의 테스트
    let input = "int add(int a, int b) { return a + b; }";
    let program = parse_program(input).unwrap();

    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];

    // 함수 이름과 반환 타입 검증
    assert_eq!(func.name, "add");
    assert_eq!(func.return_ty, TypeSpecifier::Int);

    // 매개변수 검증
    assert_eq!(func.params.len(), 2);

    assert_eq!(func.params[0].name, "a");
    assert_eq!(func.params[0].ty, TypeSpecifier::Int);

    assert_eq!(func.params[1].name, "b");
    assert_eq!(func.params[1].ty, TypeSpecifier::Int);

    // 함수 본문이 한 개의 문장을 가지고 있는지 검증
    assert_eq!(func.body.statements.len(), 1);
}

#[test]
fn test_function_with_pointer_type() {
    // 포인터 타입을 사용하는 함수 정의 테스트
    let input = "int* get_array(int size) { return 0; }";
    let program = parse_program(input).unwrap();

    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];

    // 함수 이름 검증
    assert_eq!(func.name, "get_array");

    // 포인터 반환 타입 검증
    match &func.return_ty {
        TypeSpecifier::Pointer(inner_ty) => {
            match &**inner_ty {
                TypeSpecifier::Int => {} // 정상
                _ => panic!("Expected int pointer return type"),
            }
        }
        _ => panic!("Expected pointer return type"),
    }

    // 매개변수 검증
    assert_eq!(func.params.len(), 1);
    assert_eq!(func.params[0].name, "size");
    assert_eq!(func.params[0].ty, TypeSpecifier::Int);
}

#[test]
fn test_function_with_pointer_parameter() {
    // 포인터 매개변수 함수 테스트
    let input = "void update(int* ptr) { *ptr = 42; }";
    let program = parse_program(input).unwrap();

    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];

    // 함수 반환 타입과 이름 검증
    assert_eq!(func.name, "update");

    // 포인터 매개변수 검증
    assert_eq!(func.params.len(), 1);
    assert_eq!(func.params[0].name, "ptr");

    match &func.params[0].ty {
        TypeSpecifier::Pointer(inner_ty) => {
            match &**inner_ty {
                TypeSpecifier::Int => {} // 정상
                _ => panic!("Expected int pointer parameter type"),
            }
        }
        _ => panic!("Expected pointer parameter type"),
    }
}

#[test]
fn test_multiple_functions() {
    // 여러 함수 정의 테스트
    let input = "
        int foo() { return 1; }
        int bar() { return 2; }
    ";
    let program = parse_program(input).unwrap();

    assert_eq!(program.functions.len(), 2);

    assert_eq!(program.functions[0].name, "foo");
    assert_eq!(program.functions[0].return_ty, TypeSpecifier::Int);
    assert_eq!(program.functions[0].params.len(), 0);

    assert_eq!(program.functions[1].name, "bar");
    assert_eq!(program.functions[1].return_ty, TypeSpecifier::Int);
    assert_eq!(program.functions[1].params.len(), 0);
}

#[test]
fn test_char_return_type() {
    // char 리턴 타입 함수 정의 테스트
    let input = "char get_char() { return 'a'; }";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];

    // 함수 이름과 리턴 타입 검증
    assert_eq!(func.name, "get_char");
    assert_eq!(func.return_ty, TypeSpecifier::Char);
    assert_eq!(func.params.len(), 0);

    // 본문에 하나의 return 'a'; 문장이 있는지
    let stmts = &func.body.statements;
    assert_eq!(stmts.len(), 1);
    match &stmts[0] {
        Stmt::Return(Some(Expr::CharLiteral(c))) if *c == 'a' => {}
        other => panic!("expected `return 'a';`, got {:?}", other),
    }
}
#[test]
fn test_multiple_pointer_return() {
    // 여러 단계 포인터 리턴 타입 함수 정의 테스트
    let input = "int*** foo() {}";
    let program = parse_program(input).unwrap();
    let func = &program.functions[0];

    // Return 타입이 Pointer<Pointer<Pointer<Int>>> 인지 검증
    let mut ty = &func.return_ty;
    for _ in 0..3 {
        match ty {
            TypeSpecifier::Pointer(inner) => ty = inner,
            _ => panic!("expected an extra pointer level, got {:?}", ty),
        }
    }
    assert_eq!(*ty, TypeSpecifier::Int);
}

#[test]
fn test_name_with_underscores_and_digits() {
    // 함수 이름에 밑줄과 숫자가 포함된 경우 테스트
    let input = "int _init42_() {}";
    let program = parse_program(input).unwrap();
    let func = &program.functions[0];

    // 함수 이름 검증
    assert_eq!(func.name, "_init42_");
}

#[test]
fn test_pointer_parameters_with_spaces() {
    // 매개변수 타입에 포인터와 공백이 섞인 경우 테스트
    let input = "int f( char  * a  ,int* b) {}";
    let program = parse_program(input).unwrap();
    let func = &program.functions[0];
    assert_eq!(func.params.len(), 2);

    // 첫 번째 파라미터는 char*
    match &func.params[0].ty {
        TypeSpecifier::Pointer(inner) => assert_eq!(**inner, TypeSpecifier::Char),
        other => panic!("expected pointer-to-char, got {:?}", other),
    }
    // 두 번째 파라미터는 int*
    match &func.params[1].ty {
        TypeSpecifier::Pointer(inner) => assert_eq!(**inner, TypeSpecifier::Int),
        other => panic!("expected pointer-to-int, got {:?}", other),
    }
}

#[test]
fn test_recursive_function() {
    let input = "int factorial(int n) { if (n <= 1) return 1; else return n * factorial(n-1); }";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "factorial");
    // 함수 내부에서 자기 자신을 호출하는지 확인
    if let Stmt::If {
        cond: _,
        then_branch,
        else_branch,
    } = &program.functions[0].body.statements[0]
    {
        if let Some(else_branch) = else_branch {
            if let Stmt::Return(Some(Expr::BinaryOp {
                op: BinaryOp::Mul,
                lhs,
                rhs,
            })) = &**else_branch
            {
                if let Expr::Call { func, args } = &**rhs {
                    if let Expr::Ident(name) = &**func {
                        assert_eq!(name, "factorial");
                    } else {
                        panic!("Expected recursive call to factorial");
                    }
                } else {
                    panic!("Expected recursive call in multiplication");
                }
            } else {
                panic!("Expected return n * factorial(n-1)");
            }
        } else {
            panic!("Expected else branch");
        }
    } else {
        panic!("Expected if statement");
    }
}

#[test]
fn test_function_prototype() {
    let input = "int prototype(int a, char b);";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 1);
    assert!(program.functions[0].body.statements.is_empty());
    assert_eq!(program.functions[0].name, "prototype");
    assert_eq!(program.functions[0].params.len(), 2);
}

#[test]
fn test_function_with_void_parameter() {
    let input = "int f(void) { return 1; }";
    let program = parse_program(input).unwrap();
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].params.len(), 0);
}
