use crate::ast::Expr::*;
use crate::ast::Stmt::*;
use crate::ast::TypeSpecifier::{Char, Int, Pointer};
use crate::ast::expr::BinaryOp::*;
use crate::ast::expr::PrefixOp::*;
use crate::ast::{Expr, Program, Stmt, TypeSpecifier};
use crate::semantic::analyzer::AnalyzeResult;
use crate::semantic::analyzer::SemanticError::*;
use crate::semantic::resolver::{ResolveError, Resolver};

pub struct TypeChecker<'a> {
    pub resolver: &'a mut Resolver,
    pub current_ret_ty: Option<TypeSpecifier>,
}

impl<'a> TypeChecker<'a> {
    pub fn check_program(&mut self, prog: &Program) -> AnalyzeResult<()> {
        self.current_ret_ty = None;

        for func in &prog.functions {
            self.current_ret_ty = Some(func.return_ty.clone());
            // 새 스코프
            self.resolver.push_scope();
            // 파라미터를 로컬 변수로 선언
            for param in &func.params {
                self.resolver
                    .declare_variable(&param.name, &param.ty)
                    .map_err(|_| DuplicateDeclaration {
                        name: param.name.clone(),
                    })?;
            }
            // 본문 검사
            self.check_block(&func.body.statements)?;
            self.resolver.pop_scope();

            self.current_ret_ty = None;
        }
        Ok(())
    }

    fn check_block(&mut self, stmts: &[Stmt]) -> AnalyzeResult<()> {
        for stmt in stmts {
            self.check_stmt(stmt)?;
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> AnalyzeResult<()> {
        match stmt {
            Declaration { ty, declarators } => {
                for d in declarators {
                    // 초기화식 타입 검사
                    if let Some(init) = &d.init {
                        let found = self.check_expr(init)?;
                        if &found != ty {
                            return Err(TypeMismatch {
                                expected: ty.clone(),
                                found,
                            });
                        }
                    }

                    let var_ty = if d.array_size.is_some() {
                        Pointer(Box::new(ty.clone()))
                    } else {
                        ty.clone()
                    };

                    self.resolver
                        .declare_variable(&d.name, &var_ty)
                        .map_err(|_| DuplicateDeclaration {
                            name: d.name.clone(),
                        })?;
                }
            }
            ExprStmt(opt) => {
                if let Some(e) = opt {
                    let _ = self.check_expr(e)?;
                }
            }
            Return(opt) => {
                if let Some(e) = opt {
                    let found = self.check_expr(e)?;
                    let expected = self
                        .current_ret_ty
                        .clone()
                        .expect("현재 함수 반환 타입이 없음");
                    if found != expected && !(expected == Char && found == Int) {
                        return Err(TypeMismatch { expected, found });
                    }
                }
            }
            If {
                cond,
                then_branch,
                else_branch,
            } => {
                // 조건은 int
                let ct = self.check_expr(cond)?;
                if ct != TypeSpecifier::Int {
                    return Err(TypeMismatch {
                        expected: TypeSpecifier::Int,
                        found: ct,
                    });
                }
                self.check_stmt(then_branch)?;
                if let Some(eb) = else_branch {
                    self.check_stmt(eb)?;
                }
            }
            While { cond, body } => {
                // 조건은 int
                let ct = self.check_expr(cond)?;
                if ct != TypeSpecifier::Int {
                    return Err(TypeMismatch {
                        expected: TypeSpecifier::Int,
                        found: ct,
                    });
                }
                self.check_stmt(body)?;
            }
            For {
                init,
                cond,
                step,
                body,
            } => {
                if let Some(i) = init {
                    self.check_stmt(i)?;
                }
                if let Some(c) = cond {
                    // 조건문은 결과는 int
                    let ct = self.check_expr(c)?;
                    if ct != TypeSpecifier::Int {
                        return Err(TypeMismatch {
                            expected: TypeSpecifier::Int,
                            found: ct,
                        });
                    }
                }
                if let Some(s) = step {
                    let _ = self.check_expr(s)?;
                }
                self.check_stmt(body)?;
            }
            Block(stmts) => {
                self.resolver.push_scope();
                self.check_block(&stmts.statements)?;
                self.resolver.pop_scope();
            }
            Break | Continue => {
                // loop_depth 는 Resolver 에서 이미 검사
            }
        }
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr) -> AnalyzeResult<TypeSpecifier> {
        match expr {
            IntLiteral(_) => Ok(TypeSpecifier::Int),
            CharLiteral(_) => Ok(Char),
            Ident(name) => {
                let sym = self
                    .resolver
                    .resolve_identifier(name)
                    .map_err(|_| UndefinedSymbol { name: name.clone() })?;
                Ok(sym.ty.clone())
            }
            Assignment { left, right, op: _ } => {
                let lt = self.check_expr(left)?;
                let rt = self.check_expr(right)?;
                if lt == rt {
                    Ok(lt)
                } else {
                    Err(TypeMismatch {
                        expected: lt,
                        found: rt,
                    })
                }
            }

            UnaryPrefixOp { op, rhs } => {
                let ty = self.check_expr(rhs)?;
                match op {
                    // -x, !x 는 int만
                    Neg | Not => {
                        if ty == TypeSpecifier::Int {
                            Ok(TypeSpecifier::Int)
                        } else {
                            Err(TypeMismatch {
                                expected: TypeSpecifier::Int,
                                found: ty,
                            })
                        }
                    }
                    Address => Ok(TypeSpecifier::Pointer(Box::new(ty))),
                    Deref => {
                        if let TypeSpecifier::Pointer(inner) = ty {
                            Ok(*inner)
                        } else {
                            Err(TypeMismatch {
                                expected: TypeSpecifier::Pointer(Box::new(TypeSpecifier::Int)),
                                found: ty,
                            })
                        }
                    }
                    PreInc | PreDec => {
                        if ty == TypeSpecifier::Int {
                            Ok(TypeSpecifier::Int)
                        } else {
                            Err(TypeMismatch {
                                expected: TypeSpecifier::Int,
                                found: ty,
                            })
                        }
                    }
                }
            }

            UnaryPostfixOp { lhs, .. } => {
                let ty = self.check_expr(lhs)?;
                if ty != TypeSpecifier::Int {
                    return Err(TypeMismatch {
                        expected: TypeSpecifier::Int,
                        found: ty,
                    });
                }

                Ok(TypeSpecifier::Int)
            }

            Call { func, args } => {
                let function_name = if let Ident(name) = func.as_ref() {
                    name
                } else {
                    unreachable!()
                };

                let (param_types, ret_ty) =
                    self.resolver
                        .lookup_function(function_name)
                        .map_err(|e| match e {
                            ResolveError::UndefinedSymbol { name } => UndefinedSymbol { name },
                            ResolveError::NotAFunciton { name } => NotAFunction { name },
                        })?;

                // 인자 갯수 검사
                if args.len() != param_types.len() {
                    return Err(ArgumentCountMismatch {
                        expected: param_types.len(),
                        found: args.len(),
                    });
                }

                // 인자 타입 검사
                for (arg, expected_ty) in args.iter().zip(param_types) {
                    let actual_ty = self.check_expr(arg)?;
                    if actual_ty != expected_ty {
                        return Err(TypeMismatch {
                            expected: expected_ty.clone(),
                            found: actual_ty.clone(),
                        });
                    }
                }

                Ok(ret_ty)
            }
            ArrayIndex { array, index } => {
                let idx_ty = self.check_expr(index)?;
                if idx_ty != TypeSpecifier::Int {
                    return Err(TypeMismatch {
                        expected: TypeSpecifier::Int,
                        found: idx_ty,
                    });
                }

                let arr_ty = self.check_expr(array)?;
                match arr_ty {
                    TypeSpecifier::Pointer(inner) => Ok(*inner),
                    other => Err(ExpectedPointer { found: other }),
                }
            }
            InitializerList(es) => {
                if es.is_empty() {
                    return Ok(self.current_ret_ty.clone().unwrap());
                }

                let first_ty = self.check_expr(&es[0])?;
                for e in &es[1..] {
                    let ty = self.check_expr(e)?;
                    if ty != first_ty {
                        return Err(TypeMismatch {
                            expected: first_ty.clone(),
                            found: ty,
                        });
                    }
                }

                Ok(first_ty)
            }
            BinaryOp { lhs, op, rhs } => {
                let lt = self.check_expr(lhs)?;
                let rt = self.check_expr(rhs)?;

                match op {
                    Add | Sub | Mul | Div | Rem | BitAnd | BitOr | BitXor => {
                        if !matches!(lt, Int | Char) || !matches!(rt, Int | Char) {
                            return Err(TypeMismatch {
                                expected: Int,
                                found: if lt != Int { lt } else { rt },
                            });
                        }
                        Ok(Int)
                        // if lt != TypeSpecifier::Int || rt != TypeSpecifier::Int {
                        //     return Err(TypeMismatch {
                        //         expected: TypeSpecifier::Int,
                        //         found: if lt != TypeSpecifier::Int { lt } else { rt },
                        //     });
                        // }
                        // Ok(TypeSpecifier::Int)
                    }
                    And | Or => {
                        if lt != TypeSpecifier::Int || rt != TypeSpecifier::Int {
                            return Err(TypeMismatch {
                                expected: TypeSpecifier::Int,
                                found: if lt != TypeSpecifier::Int { lt } else { rt },
                            });
                        }
                        Ok(TypeSpecifier::Int)
                    }
                    Eq | Ne | Lt | Le | Gt | Ge => {
                        if lt != rt {
                            return Err(TypeMismatch {
                                expected: lt.clone(),
                                found: rt,
                            });
                        }
                        Ok(TypeSpecifier::Int)
                    }
                }
            }
        }
    }
}
