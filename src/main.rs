use rustc_tape4::{IrGenerator, Lexer, Parser, SemanticAnalyzer};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/sample.c"
    };

    let source = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    // lexing
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.collect_spanned_tokens();

    // Check for lexer errors
    for token in &tokens {
        match &token.kind {
            rustc_tape4::lexer::Token::Error(e) => {
                eprintln!("{}:{}: lexical error: {}", token.line, token.column, e);
                print_error_at_position(&source, token.line, token.column);
                process::exit(1);
            }
            rustc_tape4::lexer::Token::Illegal(ch) => {
                eprintln!(
                    "{}:{}: illegal character '{}'",
                    token.line, token.column, ch
                );
                print_error_at_position(&source, token.line, token.column);
                process::exit(1);
            }
            _ => {}
        }
    }

    // parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("{}", e);
            print_error_location(&source, &format!("{}", e));
            process::exit(1);
        }
    };

    // semantic analysis
    let mut analyzer = SemanticAnalyzer::new(&program);
    if let Err(e) = analyzer.analyze() {
        eprintln!("{}", e);
        process::exit(1);
    }

    // IR generation
    let mut ir_gen = IrGenerator::new();
    let ir_program = match ir_gen.generate(&program) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    // output IR code
    println!("{}", ir_program);
}

fn print_error_location(source: &str, error_msg: &str) {
    // Try to extract line:col from error message
    if let Some(pos) = error_msg.find(':') {
        if let Some(pos2) = error_msg[pos + 1..].find(':') {
            let line_str = &error_msg[..pos];
            let col_str = &error_msg[pos + 1..pos + 1 + pos2];

            if let (Ok(line), Ok(col)) = (line_str.parse::<usize>(), col_str.parse::<usize>()) {
                print_error_at_position(source, line, col);
            }
        }
    }
}

fn print_error_at_position(source: &str, line: usize, col: usize) {
    let lines: Vec<&str> = source.lines().collect();
    if line > 0 && line <= lines.len() {
        let line_content = lines[line - 1];
        eprintln!("    |");
        eprintln!("{:3} | {}", line, line_content);
        eprintln!("    |{}^", " ".repeat(col));
    }
}
