use rustc_tape4::{Lexer, Parser, SemanticAnalyzer};

fn main() {
    let files = [
        "tests/fixtures/error_missing_semicolon.c",
        "tests/fixtures/error_undefined_var.c",
        "tests/fixtures/error_type_mismatch.c",
        "tests/fixtures/error_invalid_token.c",
    ];

    for file_path in &files {
        println!("\n=== Testing {} ===", file_path);
        let source = std::fs::read_to_string(file_path).expect("could not read file");
        
        // lexing
        let tokens = Lexer::new(&source).collect_spanned_tokens();
        
        // parsing
        let mut parser = Parser::new(tokens);
        match parser.parse_program() {
            Ok(program) => {
                println!("Parsing successful");
                
                // semantic analysis
                let mut analyzer = SemanticAnalyzer::new(&program);
                match analyzer.analyze() {
                    Ok(_) => println!("Semantic analysis successful"),
                    Err(e) => println!("Semantic error: {:?}", e),
                }
            }
            Err(e) => println!("Parser error: {:?}", e),
        }
    }
}