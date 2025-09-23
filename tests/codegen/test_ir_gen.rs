use super::compile_to_ir;

#[test]
fn test_simple_function() {
    let source = r#"
        int main() {
            return 42;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("define i32 @main()"));
    assert!(ir.contains("ret 42"));
}

#[test]
fn test_variable_declaration() {
    let source = r#"
        int main() {
            int x = 10;
            return x;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("alloca i32"));
    assert!(ir.contains("store"));
    assert!(ir.contains("load"));
}

#[test]
fn test_arithmetic_operations() {
    let source = r#"
        int add(int a, int b) {
            return a + b;
        }
        
        int main() {
            int x = 5;
            int y = 3;
            int z = x * y - 2;
            return z;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("add"));
    assert!(ir.contains("mul"));
    assert!(ir.contains("sub"));
}

#[test]
fn test_if_statement() {
    let source = r#"
        int max(int a, int b) {
            if (a > b) {
                return a;
            } else {
                return b;
            }
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("icmp sgt"));
    assert!(ir.contains("br"));
    assert!(ir.contains("then"));
    assert!(ir.contains("else"));
}

#[test]
fn test_while_loop() {
    let source = r#"
        int sum(int n) {
            int total = 0;
            int i = 1;
            while (i <= n) {
                total = total + i;
                i = i + 1;
            }
            return total;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("while."));
    assert!(ir.contains("while.body"));
    assert!(ir.contains("while.end"));
}

#[test]
fn test_for_loop() {
    let source = r#"
        int factorial(int n) {
            int result = 1;
            for (int i = 2; i <= n; i = i + 1) {
                result = result * i;
            }
            return result;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("for"));
    assert!(ir.contains("for.body"));
    assert!(ir.contains("for.update"));
    assert!(ir.contains("for.end"));
}

#[test]
fn test_function_call() {
    let source = r#"
        int square(int x) {
            return x * x;
        }
        
        int main() {
            int result = square(5);
            return result;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("call @square"));
}

#[test]
fn test_logical_operators() {
    let source = r#"
        int test_logic(int a, int b) {
            if (a > 0 && b > 0) {
                return 1;
            }
            if (a < 0 || b < 0) {
                return -1;
            }
            return 0;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("and."));
    assert!(ir.contains("or."));
}

#[test]
fn test_array_access() {
    let source = r#"
        int sum_array() {
            int arr[5];
            arr[0] = 1;
            arr[1] = 2;
            arr[2] = 3;
            arr[3] = 4;
            arr[4] = 5;
            
            int sum = 0;
            for (int i = 0; i < 5; i = i + 1) {
                sum = sum + arr[i];
            }
            return sum;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("getelementptr"));
    assert!(ir.contains("[5 x i32]"));
}

#[test]
fn test_pointer_operations() {
    let source = r#"
        int test_pointer() {
            int x = 42;
            int *p = &x;
            *p = 100;
            return x;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("alloca"));
    assert!(ir.contains("store"));
    assert!(ir.contains("load"));
}

#[test]
fn test_break_continue() {
    let source = r#"
        int test_control() {
            int sum = 0;
            for (int i = 0; i < 10; i = i + 1) {
                if (i == 5) {
                    continue;
                }
                if (i == 8) {
                    break;
                }
                sum = sum + i;
            }
            return sum;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("br label"));
}

#[test]
fn test_nested_loops() {
    let source = r#"
        int multiply_table() {
            int result = 0;
            for (int i = 1; i <= 3; i = i + 1) {
                for (int j = 1; j <= 3; j = j + 1) {
                    result = result + i * j;
                }
            }
            return result;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("for"));
    let for_count = ir.matches("for.").count();
    assert!(for_count >= 8);
}

#[test]
fn test_char_operations() {
    let source = r#"
        char get_char() {
            char c = 'A';
            return c;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("i8"));
}

#[test]
fn test_compound_assignment() {
    let source = r#"
        int test_compound() {
            int x = 10;
            x += 5;
            x -= 3;
            x *= 2;
            x /= 4;
            x %= 3;
            return x;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("add"));
    assert!(ir.contains("sub"));
    assert!(ir.contains("mul"));
    assert!(ir.contains("sdiv"));
    assert!(ir.contains("srem"));
}

#[test]
fn test_bitwise_operations() {
    let source = r#"
        int test_bitwise() {
            int a = 5;
            int b = 3;
            int c = a & b;
            int d = a | b;
            int e = a ^ b;
            return c + d + e;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("and"));
    assert!(ir.contains("or"));
    assert!(ir.contains("xor"));
}

#[test]
fn test_unary_operations() {
    let source = r#"
        int test_unary() {
            int x = 5;
            int y = -x;
            int z = !x;
            return y + z;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("sub 0,"));
    assert!(ir.contains("icmp eq 0,"));
}

#[test]
fn test_pre_post_increment() {
    let source = r#"
        int test_increment() {
            int x = 5;
            int a = ++x;
            int b = x++;
            int c = --x;
            int d = x--;
            return a + b + c + d;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("add"));
    assert!(ir.contains("sub"));
    assert!(ir.contains("store"));
    assert!(ir.contains("load"));
}

#[test]
fn test_empty_function() {
    let source = r#"
        void empty() {
        }
        
        int main() {
            empty();
            return 0;
        }
    "#;

    let ir = compile_to_ir(source);
    assert!(ir.contains("define void @empty()"));
    assert!(ir.contains("call @empty()"));
}

#[test]
fn test_multiple_returns() {
    let source = r#"
        int sign(int x) {
            if (x > 0) {
                return 1;
            }
            if (x < 0) {
                return -1;
            }
            return 0;
        }
    "#;

    let ir = compile_to_ir(source);
    let ret_count = ir.matches("ret ").count();
    assert!(ret_count >= 3);
}
