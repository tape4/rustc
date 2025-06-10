use crate::ast::{Expr, Program, Stmt};
use crate::semantic::analyzer::error::SemanticError;
use crate::semantic::resolver::resolver::Resolver;
pub type AnalzeResult<T> = Result<T, SemanticError>;
pub struct Analyzer<'a> {
    pub name_resolver: Resolver,
    pub program: &'a Program,
    pub loop_depth: usize,
}
impl<'a> Analyzer<'a> {
    pub fn new(program: &'a Program) -> Self {
        Analyzer {
            name_resolver: Resolver::new(),
            program,
            loop_depth: 0,
        }
    }
    pub fn analyze(&mut self) -> AnalzeResult<()> {
        for func in self.program.functions.iter() {
            self.name_resolver.declare_function(func).map_err(|_| {
                SemanticError::DuplicateDeclaration {
                    name: func.name.clone(),
                }
            })?;
        }

        for func in self.program.functions.iter() {
            self.name_resolver.push_scope();
            for param in &func.params {
                self.name_resolver
                    .declare_variable(&param.name, &param.ty)
                    .map_err(|_| SemanticError::DuplicateDeclaration {
                        name: param.name.clone(),
                    })?;
            }
            self.analyze_block(&Stmt::Block(func.body.clone()))?;
            self.name_resolver.pop_scope();
        }
        Ok(())
    }
    fn analyze_block(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        if let Stmt::Block(block) = stmt {
            for s in &block.statements {
                self.analyze_stmt(s)?;
            }
        } else {
            // println!("{:?}", stmt);
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }
    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                if let Some(expr_opt) = expr {
                    self.analyze_expr(expr_opt)?
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.analyze_expr(cond)?;
                self.analyze_block(then_branch)?;
                if let Some(else_branch_opt) = else_branch {
                    self.analyze_block(else_branch_opt)?;
                }
            }
            Stmt::While { cond, body } => {
                self.loop_depth += 1;
                self.analyze_expr(cond)?;
                self.analyze_stmt(body)?;
                self.loop_depth -= 1;
            }
            Stmt::Return(expr) => {
                if let Some(expr_opt) = expr {
                    self.analyze_expr(expr_opt)?;
                }
            }
            Stmt::Block(inner_stmts) => {
                self.analyze_block(&Stmt::Block(inner_stmts.clone()))?;
            }
            Stmt::For {
                init,
                cond,
                step,
                body,
            } => {
                self.loop_depth += 1;
                self.name_resolver.push_scope();
                if let Some(init_stmt) = init {
                    self.analyze_stmt(init_stmt)?;
                }
                if let Some(cond_expr) = cond {
                    self.analyze_expr(cond_expr)?;
                }
                self.analyze_stmt(body)?;
                if let Some(step_expr) = step {
                    self.analyze_expr(step_expr)?;
                }
                self.name_resolver.pop_scope();
                self.loop_depth -= 1;
            }
            Stmt::Declaration { ty, declarators } => {
                for declarator in declarators {
                    self.name_resolver
                        .declare_variable(&declarator.name, ty)
                        .map_err(|_| SemanticError::DuplicateDeclaration {
                            name: declarator.name.clone(),
                        })?;
                    if let Some(init_expr) = &declarator.init {
                        self.analyze_expr(&init_expr)?;
                    }
                }
            }
            Stmt::Continue => {
                if self.loop_depth == 0 {
                    return Err(SemanticError::InvalidContinue);
                }
            }
            Stmt::Break => {
                if self.loop_depth == 0 {
                    return Err(SemanticError::InvalidBreak);
                }
            }
        }
        Ok(())
    }
    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), SemanticError> {
        match expr {
            Expr::Ident(name) => {
                self.name_resolver
                    .resolve_identifier(name)
                    .map_err(|_| SemanticError::UndefinedSymbol { name: name.clone() })?;
            }
            Expr::BinaryOp { lhs, rhs, .. } => {
                self.analyze_expr(lhs)?;
                self.analyze_expr(rhs)?;
            }
            Expr::UnaryPostfixOp { lhs, .. } => {
                self.analyze_expr(lhs)?;
            }
            Expr::UnaryPrefixOp { rhs, .. } => {
                self.analyze_expr(rhs)?;
            }
            Expr::Assignment { left, right, .. } => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
            }
            Expr::ArrayIndex { array, index } => {
                self.analyze_expr(array)?;
                self.analyze_expr(index)?;
            }
            Expr::Call { func, args, .. } => match func.as_ref() {
                Expr::Ident(func_name) => {
                    self.name_resolver
                        .resolve_identifier(func_name)
                        .map_err(|_| SemanticError::UndefinedSymbol {
                            name: func_name.clone(),
                        })?;
                    for arg in args {
                        self.analyze_expr(arg)?;
                    }
                }
                _ => unreachable!(),
            },
            Expr::InitializerList(exprs) => {
                for e in exprs {
                    self.analyze_expr(e)?;
                }
            }
            Expr::CharLiteral(_) | Expr::IntLiteral(_) => {}
        }
        Ok(())
    }
}
