use crate::ast::Program;
use crate::semantic::analyzer::SemanticError;
use crate::semantic::resolver::resolver::Resolver;
use crate::semantic::type_checker::type_checker::TypeChecker;

pub type AnalyzeResult<T> = Result<T, SemanticError>;

pub struct Analyzer<'a> {
    pub program: &'a Program,
}

impl<'a> Analyzer<'a> {
    pub fn new(program: &'a Program) -> Self {
        Analyzer { program }
    }

    pub fn analyze(&mut self) -> AnalyzeResult<()> {
        // 이름 해석
        let mut resolver = Resolver::new();
        resolver.resolve_program(self.program)?;

        // 타입 검사
        let mut tc = TypeChecker::new(&mut resolver);
        tc.check_program(self.program)?;

        Ok(())
    }
}
