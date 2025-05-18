use crate::utils::parse_program;
use rustc_tape4::ast::Expr;
use rustc_tape4::ast::Stmt;
use rustc_tape4::ast::expr::{BinaryOp, PrefixOp};
use rustc_tape4::ast::{Function, Program, TypeSpecifier};
use std::fs;

/// fixture 파일의 내용을 읽어오는 함수
fn read_fixture(filename: &str) -> String {
    fs::read_to_string(format!("tests/fixtures/{}", filename))
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", filename))
}

#[test]
fn test_parse_sample_c() {
    // 기존 sample.c 파일 파싱 테스트
    let source = read_fixture("sample.c");
    let program = parse_program(&source).unwrap();

    // 최소한의 기본 검증
    assert!(!program.functions.is_empty());

    // main 함수 찾기
    let main_func = program
        .functions
        .iter()
        .find(|f| f.name == "main")
        .expect("main function not found");

    assert_eq!(main_func.return_ty, TypeSpecifier::Int);

    // add, factorial, to_uppercase 함수 존재 확인
    assert!(program.functions.iter().any(|f| f.name == "add"));
    assert!(program.functions.iter().any(|f| f.name == "factorial"));
    assert!(program.functions.iter().any(|f| f.name == "to_uppercase"));
}

#[test]
fn test_expressions_fixture() {
    // 다양한 표현식을 포함하는 테스트
    let source = r#"
    int test_expressions() {
        // 산술 표현식
        int a = 10;
        int b = 20;
        int c = a + b * (30 - 5) / 5;

        // 비교 및 논리 표현식
        if (a > 0 && b <= 30) {
            c = 1;
        } else if (a == 0 || b != 20) {
            c = 2;
        } else {
            c = 3;
        }

        // 비트 연산 표현식
        int d = a & b;
        int e = a | b;
        int f = a ^ b;

        // 복합 할당 표현식
        c += 5;
        c -= 3;
        c *= 2;
        c /= 4;
        c %= 3;

        // 증감 연산자
        c++;
        ++c;
        c--;
        --c;

        // 포인터 연산
        int *p = &c;
        *p = 100;

        // 함수 호출
        int result = add(10, 20);

        // 배열 접근
        int arr[5] = {1, 2, 3, 4, 5};
        arr[2] = arr[0] + arr[1];

        return c;
    }
    "#;

    let program = parse_program(source).unwrap();
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "test_expressions");

    // 함수 본문 내 문장 개수 확인
    assert!(program.functions[0].body.statements.len() > 10);
}

#[test]
fn test_statements_fixture() {
    // 다양한 문장을 포함하는 테스트
    let source = r#"
    int test_statements() {
        // 변수 선언 문장
        int x;
        int y = 10;
        int arr[5];
        char c = 'A';

        // if-else 문장
        if (x > 0) {
            y = 1;
        } else if (x < 0) {
            y = -1;
        } else {
            y = 0;
        }

        // while 루프
        int i = 0;
        while (i < 5) {
            arr[i] = i * i;
            i++;
        }

        // for 루프
        int sum = 0;
        for (int j = 0; j < 5; j++) {
            sum += arr[j];

            if (sum > 10) {
                break;
            }

            if (j % 2 == 0) {
                continue;
            }
        }

        // 중첩 블록
        {
            int temp = x;
            x = y;
            y = temp;

            {
                int z = x + y;
            }
        }

        // return 문장
        return sum;
    }
    "#;

    let program = parse_program(source).unwrap();
    assert_eq!(program.functions.len(), 1);
    assert_eq!(program.functions[0].name, "test_statements");

    // 전체 AST 구조 탐색으로 특정 문장 유형 확인
    let has_if = contains_stmt_type(&program, |s| matches!(s, Stmt::If { .. }));
    let has_while = contains_stmt_type(&program, |s| matches!(s, Stmt::While { .. }));
    let has_for = contains_stmt_type(&program, |s| matches!(s, Stmt::For { .. }));
    let has_break = contains_stmt_type(&program, |s| matches!(s, Stmt::Break));
    let has_continue = contains_stmt_type(&program, |s| matches!(s, Stmt::Continue));
    let has_return = contains_stmt_type(&program, |s| matches!(s, Stmt::Return(..)));

    assert!(has_if, "프로그램에 if 문이 없습니다");
    assert!(has_while, "프로그램에 while 문이 없습니다");
    assert!(has_for, "프로그램에 for 문이 없습니다");
    assert!(has_break, "프로그램에 break 문이 없습니다");
    assert!(has_continue, "프로그램에 continue 문이 없습니다");
    assert!(has_return, "프로그램에 return 문이 없습니다");
}

#[test]
fn test_pointers_and_arrays_fixture() {
    // 포인터와 배열 관련 테스트
    let source = r#"
    void swap(int *a, int *b) {
        int temp = *a;
        *a = *b;
        *b = temp;
    }

    int* createArray(int size) {
        // 실제로는 메모리 할당을 해야 하지만 파서 테스트이므로 0을 리턴
        return 0;
    }

    void arrayOperations() {
        int arr[10];
        int *ptr = arr;

        // 배열 초기화
        for (int i = 0; i < 10; i++) {
            arr[i] = i * 10;
        }

        // 포인터 산술
        ptr = ptr + 5;    // arr[5]를 가리킴
        *ptr = 100;       // arr[5] = 100

        ptr -= 2;         // arr[3]를 가리킴
        *(ptr + 1) = 50;  // arr[4] = 50

        // 1차원 배열
        int matrix[9];
        for (int k = 0; k < 9; k++) {
            matrix[k] = k;  // 0,1,2,…,8
        }

        // 포인터 배열
        int *ptrArray[5];
        for (int i = 0; i < 5; i++) {
            ptrArray[i] = &arr[i * 2];
        }
    }
    "#;

    let program = parse_program(source).unwrap();
    assert_eq!(program.functions.len(), 3);

    // swap 함수 검증
    let swap_func = program
        .functions
        .iter()
        .find(|f| f.name == "swap")
        .expect("swap 함수를 찾을 수 없음");

    // swap 함수의 매개변수가 포인터 타입인지 확인
    assert_eq!(swap_func.params.len(), 2);
    for param in &swap_func.params {
        match &param.ty {
            TypeSpecifier::Pointer(inner) => {
                assert_eq!(**inner, TypeSpecifier::Int);
            }
            _ => panic!("swap 함수의 매개변수가 포인터 타입이 아님"),
        }
    }

    // createArray 함수의 반환 타입이 int* 인지 확인
    let create_array_func = program
        .functions
        .iter()
        .find(|f| f.name == "createArray")
        .expect("createArray 함수를 찾을 수 없음");

    match &create_array_func.return_ty {
        TypeSpecifier::Pointer(inner) => {
            assert_eq!(**inner, TypeSpecifier::Int);
        }
        _ => panic!("createArray 함수의 반환 타입이 int* 가 아님"),
    }

    // 포인터 연산이 AST에 존재하는지 확인
    let array_ops_func = program
        .functions
        .iter()
        .find(|f| f.name == "arrayOperations")
        .expect("arrayOperations 함수를 찾을 수 없음");

    let has_pointer_arith = contains_expr_type(array_ops_func, |e| {
        matches!(
            e,
            Expr::BinaryOp {
                op: BinaryOp::Add,
                ..
            } | Expr::BinaryOp {
                op: BinaryOp::Sub,
                ..
            }
        )
    });

    let has_dereference = contains_expr_type(array_ops_func, |e| {
        matches!(
            e,
            Expr::UnaryPrefixOp {
                op: PrefixOp::Deref,
                ..
            }
        )
    });

    assert!(has_pointer_arith, "포인터 산술 연산을 찾을 수 없음");
    assert!(has_dereference, "포인터 역참조 연산을 찾을 수 없음");
}

#[test]
fn test_complex_program_fixture() {
    // 복잡한 프로그램 예제
    let source = r#"
    // 간단한 버블 정렬 구현
    void bubbleSort(int arr[], int n) {
        for (int i = 0; i < n-1; i++) {
            for (int j = 0; j < n-i-1; j++) {
                if (arr[j] > arr[j+1]) {
                    // 두 원소 교환
                    int temp = arr[j];
                    arr[j] = arr[j+1];
                    arr[j+1] = temp;
                }
            }
        }
    }

    // 이진 검색 구현
    int binarySearch(int arr[], int left, int right, int x) {
        if (right >= left) {
            int mid = left + (right - left) / 2;

            // 중간 원소인 경우
            if (arr[mid] == x)
                return mid;

            // 중간 원소보다 작은 경우: 왼쪽 하위 배열 검색
            if (arr[mid] > x)
                return binarySearch(arr, left, mid-1, x);

            // 중간 원소보다 큰 경우: 오른쪽 하위 배열 검색
            return binarySearch(arr, mid+1, right, x);
        }

        // 원소가 배열에 없음
        return -1;
    }

    // 팩토리얼 계산 (재귀)
    int factorial(int n) {
        if (n <= 1)
            return 1;
        return n * factorial(n-1);
    }

    // 피보나치 수열 계산 (반복)
    int fibonacci(int n) {
        int a = 0, b = 1, c;
        if (n == 0) return a;

        for (int i = 2; i <= n; i++) {
            c = a + b;
            a = b;
            b = c;
        }
        return b;
    }

    int main() {
        int arr[10] = {64, 34, 25, 12, 22, 11, 90, 87, 56, 45};
        int n = 10;

        bubbleSort(arr, n);

        // 정렬된 배열에서 22 검색
        int result = binarySearch(arr, 0, n-1, 22);

        int fact5 = factorial(5);  // 5! = 120
        int fib10 = fibonacci(10); // fib(10) = 55

        return 0;
    }
    "#;

    let program = parse_program(source).unwrap();
    assert_eq!(program.functions.len(), 5);

    // 모든 함수가 존재하는지 확인
    let function_names = vec![
        "bubbleSort",
        "binarySearch",
        "factorial",
        "fibonacci",
        "main",
    ];
    for name in function_names {
        assert!(
            program.functions.iter().any(|f| f.name == name),
            "함수 {}를 찾을 수 없음",
            name
        );
    }

    // 재귀 호출이 있는지 확인
    let has_recursion = contains_recursive_call(&program);
    assert!(has_recursion, "재귀 함수 호출을 찾을 수 없음");

    // 중첩된 반복문이 있는지 확인
    let has_nested_loops = contains_nested_loops(&program);
    assert!(has_nested_loops, "중첩된 반복문을 찾을 수 없음");
}

/// 프로그램에서 주어진 조건을 만족하는 문장이 있는지 검사
fn contains_stmt_type<F>(program: &Program, predicate: F) -> bool
where
    F: Fn(&Stmt) -> bool,
{
    for function in &program.functions {
        if contains_stmt_in_block(&function.body.statements, &predicate) {
            return true;
        }
    }
    false
}

/// 블록 내 문장 중 조건을 만족하는 문장이 있는지 재귀적으로 검사
fn contains_stmt_in_block<F>(statements: &[Stmt], predicate: &F) -> bool
where
    F: Fn(&Stmt) -> bool,
{
    for stmt in statements {
        if predicate(stmt) {
            return true;
        }

        // 블록, if, while, for 등 내부에 문장을 포함하는 경우 재귀 검사
        match stmt {
            Stmt::Block(block) => {
                if contains_stmt_in_block(&block.statements, predicate) {
                    return true;
                }
            }
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                if let Stmt::Block(block) = &**then_branch {
                    if contains_stmt_in_block(&block.statements, predicate) {
                        return true;
                    }
                } else if predicate(then_branch) {
                    return true;
                }

                if let Some(else_stmt) = else_branch {
                    if let Stmt::Block(block) = &**else_stmt {
                        if contains_stmt_in_block(&block.statements, predicate) {
                            return true;
                        }
                    } else if predicate(else_stmt) {
                        return true;
                    }
                }
            }
            Stmt::While { body, .. } => {
                if let Stmt::Block(block) = &**body {
                    if contains_stmt_in_block(&block.statements, predicate) {
                        return true;
                    }
                } else if predicate(body) {
                    return true;
                }
            }
            Stmt::For { body, .. } => {
                if let Stmt::Block(block) = &**body {
                    if contains_stmt_in_block(&block.statements, predicate) {
                        return true;
                    }
                } else if predicate(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// 함수에서 주어진 조건을 만족하는 표현식이 있는지 검사
fn contains_expr_type<F>(function: &Function, predicate: F) -> bool
where
    F: Fn(&Expr) -> bool,
{
    contains_expr_in_block(&function.body.statements, &predicate)
}

/// 블록 내 문장 중 조건을 만족하는 표현식이 있는지 재귀적으로 검사
fn contains_expr_in_block<F>(statements: &[Stmt], predicate: &F) -> bool
where
    F: Fn(&Expr) -> bool,
{
    for stmt in statements {
        match stmt {
            Stmt::ExprStmt(Some(expr)) => {
                if contains_expr_in_expr(expr, predicate) {
                    return true;
                }
            }
            Stmt::Declaration { declarators, .. } => {
                for decl in declarators {
                    if let Some(expr) = &decl.init {
                        if contains_expr_in_expr(expr, predicate) {
                            return true;
                        }
                    }
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                if contains_expr_in_expr(cond, predicate) {
                    return true;
                }

                if let Stmt::Block(block) = &**then_branch {
                    if contains_expr_in_block(&block.statements, predicate) {
                        return true;
                    }
                }

                if let Some(else_stmt) = else_branch {
                    if let Stmt::Block(block) = &**else_stmt {
                        if contains_expr_in_block(&block.statements, predicate) {
                            return true;
                        }
                    }
                }
            }
            Stmt::While { cond, body } => {
                if contains_expr_in_expr(cond, predicate) {
                    return true;
                }

                if let Stmt::Block(block) = &**body {
                    if contains_expr_in_block(&block.statements, predicate) {
                        return true;
                    }
                }
            }
            Stmt::For {
                init,
                cond,
                step,
                body,
            } => {
                if let Some(expr) = init {
                    match &**expr {
                        // 선언문일 수도 있지만, ExprStmt(Some(e)) 일 때만 살펴보고
                        Stmt::ExprStmt(Some(expr)) => {
                            if contains_expr_in_expr(expr, predicate) {
                                return true;
                            }
                        }
                        _ => {}
                    }
                }

                if let Some(expr) = cond {
                    if contains_expr_in_expr(expr, predicate) {
                        return true;
                    }
                }

                if let Some(expr) = step {
                    if contains_expr_in_expr(expr, predicate) {
                        return true;
                    }
                }

                if let Stmt::Block(block) = &**body {
                    if contains_expr_in_block(&block.statements, predicate) {
                        return true;
                    }
                }
            }
            Stmt::Return(Some(expr)) => {
                if contains_expr_in_expr(expr, predicate) {
                    return true;
                }
            }
            Stmt::Block(block) => {
                if contains_expr_in_block(&block.statements, predicate) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// 표현식 내에서 조건을 만족하는 표현식이 있는지 재귀적으로 검사
fn contains_expr_in_expr<F>(expr: &Expr, predicate: &F) -> bool
where
    F: Fn(&Expr) -> bool,
{
    if predicate(expr) {
        return true;
    }

    match expr {
        Expr::BinaryOp { lhs, rhs, .. } => {
            if contains_expr_in_expr(lhs, predicate) || contains_expr_in_expr(rhs, predicate) {
                return true;
            }
        }
        Expr::UnaryPrefixOp { rhs, .. } => {
            if contains_expr_in_expr(rhs, predicate) {
                return true;
            }
        }
        Expr::UnaryPostfixOp { lhs, .. } => {
            if contains_expr_in_expr(lhs, predicate) {
                return true;
            }
        }
        Expr::Call { func, args } => {
            if contains_expr_in_expr(func, predicate) {
                return true;
            }

            for arg in args {
                if contains_expr_in_expr(arg, predicate) {
                    return true;
                }
            }
        }
        Expr::ArrayIndex { array, index } => {
            if contains_expr_in_expr(array, predicate) || contains_expr_in_expr(index, predicate) {
                return true;
            }
        }
        Expr::Assignment { left, right, .. } => {
            if contains_expr_in_expr(left, predicate) || contains_expr_in_expr(right, predicate) {
                return true;
            }
        }
        Expr::InitializerList(items) => {
            for item in items {
                if contains_expr_in_expr(item, predicate) {
                    return true;
                }
            }
        }
        _ => {}
    }

    false
}

/// 프로그램에 재귀 함수 호출이 있는지 검사
fn contains_recursive_call(program: &Program) -> bool {
    for function in &program.functions {
        let func_name = &function.name;

        let has_recursive = contains_expr_in_block(&function.body.statements, &|expr| {
            if let Expr::Call { func, .. } = expr {
                if let Expr::Ident(name) = &**func {
                    return name == func_name;
                }
            }
            false
        });

        if has_recursive {
            return true;
        }
    }
    false
}

/// 프로그램에 중첩 반복문이 있는지 검사
fn contains_nested_loops(program: &Program) -> bool {
    for function in &program.functions {
        // 반복문 내에 반복문이 있는지 검사
        let has_nested = contains_stmt_in_block(&function.body.statements, &|stmt| {
            match stmt {
                Stmt::While { body, .. } | Stmt::For { body, .. } => {
                    // 반복문 본문에 또 다른 반복문이 있는지 확인
                    if let Stmt::Block(block) = &**body {
                        contains_stmt_in_block(&block.statements, &|inner_stmt| {
                            matches!(inner_stmt, Stmt::While { .. } | Stmt::For { .. })
                        })
                    } else {
                        matches!(**body, Stmt::While { .. } | Stmt::For { .. })
                    }
                }
                _ => false,
            }
        });

        if has_nested {
            return true;
        }
    }
    false
}
