use crate::ast::{Expr, Program, Stmt};
use crate::semantic::analyzer::AnalyzeResult;
use crate::semantic::analyzer::error::SemanticError;
use crate::semantic::symbol::symbol::SymbolTable;

pub struct Resolver {
    pub table: SymbolTable,
    pub loop_depth: usize,
}

impl Resolver {
    pub fn resolve_program(&mut self, prog: &Program) -> AnalyzeResult<()> {
        // 함수 선언 등록
        for func in &prog.functions {
            self.declare_function(func)
                .map_err(|_| SemanticError::DuplicateDeclaration {
                    name: func.name.clone(),
                })?;
        }
        // 함수별 스코프·본문 검사
        for func in &prog.functions {
            self.push_scope();
            for param in &func.params {
                self.declare_variable(&param.name, &param.ty).map_err(|_| {
                    SemanticError::DuplicateDeclaration {
                        name: param.name.clone(),
                    }
                })?;
            }
            self.resolve_block(&Stmt::Block(func.body.clone()))?;
            self.pop_scope();
        }
        Ok(())
    }

    fn resolve_block(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        if let Stmt::Block(block) = stmt {
            for s in &block.statements {
                self.resolve_stmt(s)?;
            }
        } else {
            // println!("{:?}", stmt);
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), SemanticError> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                if let Some(expr_opt) = expr {
                    self.resolve_expr(expr_opt)?
                }
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(cond)?;
                self.resolve_block(then_branch)?;
                if let Some(else_branch_opt) = else_branch {
                    self.resolve_block(else_branch_opt)?;
                }
            }
            Stmt::While { cond, body } => {
                self.loop_depth += 1;
                self.resolve_expr(cond)?;
                self.resolve_stmt(body)?;
                self.loop_depth -= 1;
            }
            Stmt::Return(expr) => {
                if let Some(expr_opt) = expr {
                    self.resolve_expr(expr_opt)?;
                }
            }
            Stmt::Block(inner_stmts) => {
                self.resolve_block(&Stmt::Block(inner_stmts.clone()))?;
            }
            Stmt::For {
                init,
                cond,
                step,
                body,
            } => {
                self.loop_depth += 1;
                self.push_scope();
                if let Some(init_stmt) = init {
                    self.resolve_stmt(init_stmt)?;
                }
                if let Some(cond_expr) = cond {
                    self.resolve_expr(cond_expr)?;
                }
                self.resolve_stmt(body)?;
                if let Some(step_expr) = step {
                    self.resolve_expr(step_expr)?;
                }
                self.pop_scope();
                self.loop_depth -= 1;
            }
            Stmt::Declaration { ty, declarators } => {
                for declarator in declarators {
                    self.declare_variable(&declarator.name, ty).map_err(|_| {
                        SemanticError::DuplicateDeclaration {
                            name: declarator.name.clone(),
                        }
                    })?;
                    if let Some(init_expr) = &declarator.init {
                        self.resolve_expr(&init_expr)?;
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

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), SemanticError> {
        match expr {
            Expr::Ident(name) => {
                self.resolve_identifier(name)
                    .map_err(|_| SemanticError::UndefinedSymbol { name: name.clone() })?;
            }
            Expr::BinaryOp { lhs, rhs, .. } => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
            }
            Expr::UnaryPostfixOp { lhs, .. } => {
                self.resolve_expr(lhs)?;
            }
            Expr::UnaryPrefixOp { rhs, .. } => {
                self.resolve_expr(rhs)?;
            }
            Expr::Assignment { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::ArrayIndex { array, index } => {
                self.resolve_expr(array)?;
                self.resolve_expr(index)?;
            }
            Expr::Call { func, args, .. } => match func.as_ref() {
                Expr::Ident(func_name) => {
                    self.resolve_identifier(func_name).map_err(|_| {
                        SemanticError::UndefinedSymbol {
                            name: func_name.clone(),
                        }
                    })?;
                    for arg in args {
                        self.resolve_expr(arg)?;
                    }
                }
                _ => unreachable!(),
            },
            Expr::InitializerList(expr_list) => {
                for e in expr_list {
                    self.resolve_expr(e)?;
                }
            }
            Expr::CharLiteral(_) | Expr::IntLiteral(_) => {}
        }
        Ok(())
    }
}
