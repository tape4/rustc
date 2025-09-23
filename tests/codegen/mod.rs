mod test_ir_gen;

use rustc_tape4::{IrGenerator, Lexer, Parser, SemanticAnalyzer};

fn compile_to_ir(source: &str) -> String {
    let tokens = Lexer::new(source).collect_spanned_tokens();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("Parsing failed");

    let mut analyzer = SemanticAnalyzer::new(&program);
    analyzer.analyze().expect("Semantic analysis failed");

    let mut ir_gen = IrGenerator::new();
    let ir_program = ir_gen.generate(&program).expect("IR generation failed");

    format!("{}", ir_program)
}
