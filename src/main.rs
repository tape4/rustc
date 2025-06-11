use rustc_tape4::{Lexer, Parser, SemanticAnalyzer};

fn main() {
    let source = std::fs::read_to_string("tests/fixtures/sample.c").expect("could not read file");

    // lexing
    let tokens = Lexer::new(&source).collect_spanned_tokens();

    // parsing
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing Failed");

    // semantic analysis
    let mut analyzer = SemanticAnalyzer::new(&program);
    let _result = analyzer.analyze().expect("Analyzing Failed");
}
