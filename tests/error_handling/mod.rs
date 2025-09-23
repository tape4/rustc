use rustc_tape4::{Lexer, Parser, SemanticAnalyzer};

#[test]
fn test_lexer_error_unterminated_char() {
    let source = "char c = 'a";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.collect_spanned_tokens();

    let has_error = tokens
        .iter()
        .any(|t| matches!(&t.kind, rustc_tape4::lexer::Token::Error(_)));

    assert!(
        has_error,
        "Expected lexer error for unterminated char literal"
    );
}

#[test]
fn test_lexer_error_illegal_char() {
    let source = "int x = @;";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.collect_spanned_tokens();

    let has_illegal = tokens
        .iter()
        .any(|t| matches!(&t.kind, rustc_tape4::lexer::Token::Illegal(_)));

    assert!(has_illegal, "Expected illegal character error");
}

#[test]
fn test_parser_error_missing_semicolon() {
    let source = r#"
        int main() {
            int x = 5
            int y = 10;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    assert!(
        result.is_err(),
        "Expected parser error for missing semicolon"
    );

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("expected 'Semicolon'"));
    }
}

#[test]
fn test_parser_error_unexpected_token() {
    let source = "int main() { } }"; // Extra closing brace

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    assert!(
        result.is_err(),
        "Expected parser error for unexpected token"
    );
}

#[test]
fn test_semantic_error_undefined_variable() {
    let source = r#"
        int main() {
            return x; // x is not defined
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    let result = analyzer.analyze();

    assert!(
        result.is_err(),
        "Expected semantic error for undefined variable"
    );

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("undeclared identifier"));
    }
}

#[test]
fn test_semantic_error_type_mismatch() {
    let source = r#"
        int main() {
            int x = 5;
            char c = 'a';
            int *p = &c; // Type mismatch
            return 0;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    let result = analyzer.analyze();

    assert!(result.is_err(), "Expected semantic error for type mismatch");

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("incompatible types"));
    }
}

#[test]
fn test_semantic_error_duplicate_declaration() {
    let source = r#"
        int main() {
            int x = 5;
            int x = 10; // Duplicate declaration
            return x;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    let result = analyzer.analyze();

    assert!(
        result.is_err(),
        "Expected semantic error for duplicate declaration"
    );

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("redefinition"));
    }
}

#[test]
fn test_semantic_error_break_outside_loop() {
    let source = r#"
        int main() {
            if (1) {
                break; // Not in a loop
            }
            return 0;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    let result = analyzer.analyze();

    assert!(
        result.is_err(),
        "Expected semantic error for break outside loop"
    );

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("'break' statement not in loop"));
    }
}

#[test]
fn test_semantic_error_continue_outside_loop() {
    let source = r#"
        int main() {
            continue; // Not in a loop
            return 0;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    let result = analyzer.analyze();

    assert!(
        result.is_err(),
        "Expected semantic error for continue outside loop"
    );

    if let Err(e) = result {
        let error_str = format!("{}", e);
        assert!(error_str.contains("'continue' statement not in loop"));
    }
}

#[test]
fn test_parser_error_position_info() {
    let source = "int main() {\n    int x = 5\n    int y = 10;\n}";

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_program();

    assert!(result.is_err(), "Expected parser error");

    if let Err(e) = result {
        let error_str = format!("{}", e);
        // Should contain line and column information
        assert!(
            error_str.contains("3:5"),
            "Error should contain position 3:5"
        );
    }
}

#[test]
fn test_successful_compilation() {
    let source = r#"
        int add(int a, int b) {
            return a + b;
        }
        
        int main() {
            int x = 5;
            int y = 3;
            int result = add(x, y);
            return result;
        }
    "#;

    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing should succeed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    analyzer
        .analyze()
        .expect("Semantic analysis should succeed");

    // If we get here, compilation was successful
    assert!(true);
}
