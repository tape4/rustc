use rustc_tape4::lexer::Lexer;
use rustc_tape4::parser::Parser;
use rustc_tape4::semantic;
use semantic::analyzer::analyzer::Analyzer;

mod lexer;

fn main() {
    let source = std::fs::read_to_string("tests/fixtures/sample.c").expect("could not read file");

    // lexing
    let tokens = Lexer::new(&source).collect_spanned_tokens();

    // parsing
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing Failed");

    // println!("{:#?}", program);
    // semantic analysis
    let mut analyzer = Analyzer::new(&program);
    let result = analyzer.analyze().expect("Analyzing Failed");
    // println!("Hello, world!");
}
