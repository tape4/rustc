use crate::ast::expr::{AssignOp, BinaryOp as AstBinaryOp, Expr, PostfixOp, PrefixOp};
use crate::ast::functions::Function;
use crate::ast::stmt::{Block, Stmt};
use crate::ast::{Program, TypeSpecifier};
use crate::codegen::ir::{
    BinaryOp, IrBlock, IrFunction, IrInstruction, IrOperand, IrProgram, IrType, UnaryOp,
};
use std::collections::HashMap;

pub struct IrGenerator {
    current_function: Option<String>,
    functions: Vec<IrFunction>,
    current_blocks: Vec<IrBlock>,
    current_block_index: usize,
    temp_counter: usize,
    label_counter: usize,
    local_vars: HashMap<String, String>,
    loop_labels: Vec<(String, String)>,
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            current_function: None,
            functions: Vec::new(),
            current_blocks: Vec::new(),
            current_block_index: 0,
            temp_counter: 0,
            label_counter: 0,
            local_vars: HashMap::new(),
            loop_labels: Vec::new(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> IrProgram {
        for func in &program.functions {
            self.generate_function(func);
        }
        IrProgram {
            functions: self.functions.clone(),
        }
    }

    fn generate_function(&mut self, func: &Function) {
        self.current_function = Some(func.name.clone());
        self.local_vars.clear();
        self.temp_counter = 0;
        self.label_counter = 0;
        self.current_blocks = vec![IrBlock {
            label: "entry".to_string(),
            instructions: Vec::new(),
        }];
        self.current_block_index = 0;

        let params: Vec<String> = func
            .params
            .iter()
            .map(|p| {
                let param_name = format!("arg.{}", p.name);
                self.local_vars.insert(p.name.clone(), param_name.clone());
                param_name
            })
            .collect();

        let return_type = self.convert_type(&func.return_ty);

        let mut locals = Vec::new();
        for param in &func.params {
            let local_name = format!("{}.addr", param.name);
            let param_type = self.convert_type(&param.ty);
            locals.push((local_name.clone(), param_type.clone()));

            let param_reg = self.local_vars.get(&param.name).unwrap().clone();
            self.emit(IrInstruction::Alloca {
                dest: local_name.clone(),
                ty: param_type,
            });
            self.emit(IrInstruction::Store {
                value: IrOperand::Register(param_reg),
                ptr: local_name.clone(),
            });
            self.local_vars.insert(param.name.clone(), local_name);
        }

        self.generate_block(&func.body);

        if !self.has_terminator() {
            self.emit(IrInstruction::Return(None));
        }

        let ir_func = IrFunction {
            name: func.name.clone(),
            params,
            return_type,
            locals,
            blocks: self.current_blocks.clone(),
        };
        self.functions.push(ir_func);
    }

    fn generate_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.generate_stmt(stmt);
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt) -> Option<IrOperand> {
        match stmt {
            Stmt::ExprStmt(expr) => {
                if let Some(e) = expr {
                    self.generate_expr(e)
                } else {
                    None
                }
            }
            Stmt::Return(expr) => {
                let val = expr.as_ref().and_then(|e| self.generate_expr(e));
                self.emit(IrInstruction::Return(val));
                None
            }
            Stmt::Block(block) => {
                self.generate_block(block);
                None
            }
            Stmt::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let condition = self.generate_expr(cond).unwrap();
                let then_label = self.new_label("then");
                let else_label = self.new_label("else");
                let end_label = self.new_label("endif");

                self.emit(IrInstruction::CondJump {
                    cond: condition,
                    true_label: then_label.clone(),
                    false_label: else_label.clone(),
                });

                self.new_block(then_label);
                self.generate_stmt(then_branch);
                if !self.has_terminator() {
                    self.emit(IrInstruction::Jump(end_label.clone()));
                }

                self.new_block(else_label);
                if let Some(else_s) = else_branch {
                    self.generate_stmt(else_s);
                }
                if !self.has_terminator() {
                    self.emit(IrInstruction::Jump(end_label.clone()));
                }

                self.new_block(end_label);
                None
            }
            Stmt::While { cond, body } => {
                let loop_label = self.new_label("while");
                let body_label = self.new_label("while.body");
                let end_label = self.new_label("while.end");

                self.emit(IrInstruction::Jump(loop_label.clone()));
                self.new_block(loop_label.clone());

                let condition = self.generate_expr(cond).unwrap();
                self.emit(IrInstruction::CondJump {
                    cond: condition,
                    true_label: body_label.clone(),
                    false_label: end_label.clone(),
                });

                self.new_block(body_label);
                self.loop_labels
                    .push((loop_label.clone(), end_label.clone()));
                self.generate_stmt(body);
                self.loop_labels.pop();
                if !self.has_terminator() {
                    self.emit(IrInstruction::Jump(loop_label));
                }

                self.new_block(end_label);
                None
            }
            Stmt::For {
                init,
                cond,
                step,
                body,
            } => {
                if let Some(init) = init {
                    self.generate_stmt(init);
                }

                let loop_label = self.new_label("for");
                let body_label = self.new_label("for.body");
                let update_label = self.new_label("for.update");
                let end_label = self.new_label("for.end");

                self.emit(IrInstruction::Jump(loop_label.clone()));
                self.new_block(loop_label.clone());

                let condition = cond
                    .as_ref()
                    .and_then(|c| self.generate_expr(c))
                    .unwrap_or(IrOperand::Constant(1));
                self.emit(IrInstruction::CondJump {
                    cond: condition,
                    true_label: body_label.clone(),
                    false_label: end_label.clone(),
                });

                self.new_block(body_label);
                self.loop_labels
                    .push((update_label.clone(), end_label.clone()));
                self.generate_stmt(body);
                self.loop_labels.pop();
                if !self.has_terminator() {
                    self.emit(IrInstruction::Jump(update_label.clone()));
                }

                self.new_block(update_label.clone());
                if let Some(update) = step {
                    self.generate_expr(update);
                }
                self.emit(IrInstruction::Jump(loop_label));

                self.new_block(end_label);
                None
            }
            Stmt::Break => {
                if let Some((_, end_label)) = self.loop_labels.last() {
                    self.emit(IrInstruction::Jump(end_label.clone()));
                }
                None
            }
            Stmt::Continue => {
                if let Some((continue_label, _)) = self.loop_labels.last() {
                    self.emit(IrInstruction::Jump(continue_label.clone()));
                }
                None
            }
            Stmt::Declaration { ty, declarators } => {
                let var_ty = self.convert_type(ty);

                for decl in declarators {
                    let addr = format!("{}.addr", decl.name);

                    let actual_ty = if decl.array_size.is_some() {
                        IrType::Array(Box::new(var_ty.clone()), decl.array_size.unwrap() as usize)
                    } else {
                        var_ty.clone()
                    };

                    let insert_point = self.find_alloca_insert_point();
                    self.current_blocks[0].instructions.insert(
                        insert_point,
                        IrInstruction::Alloca {
                            dest: addr.clone(),
                            ty: actual_ty.clone(),
                        },
                    );

                    if let Some(init) = &decl.init {
                        if let Expr::InitializerList(elements) = init {
                            // Handle array initialization
                            for (i, elem) in elements.iter().enumerate() {
                                if let Some(val) = self.generate_expr(elem) {
                                    let elem_addr = self.new_temp();
                                    self.emit(IrInstruction::GetElementPtr {
                                        dest: elem_addr.clone(),
                                        ptr: addr.clone(),
                                        indices: vec![
                                            IrOperand::Constant(0),
                                            IrOperand::Constant(i as i32),
                                        ],
                                        ty: actual_ty.clone(),
                                    });
                                    self.emit(IrInstruction::Store {
                                        value: val,
                                        ptr: elem_addr,
                                    });
                                }
                            }
                        } else if let Some(val) = self.generate_expr(init) {
                            self.emit(IrInstruction::Store {
                                value: val,
                                ptr: addr.clone(),
                            });
                        }
                    }

                    self.local_vars.insert(decl.name.clone(), addr);
                }
                None
            }
        }
    }

    fn generate_expr(&mut self, expr: &Expr) -> Option<IrOperand> {
        match expr {
            Expr::IntLiteral(val) => Some(IrOperand::Constant(*val as i32)),
            Expr::CharLiteral(val) => Some(IrOperand::CharConstant(*val as u8)),
            Expr::Ident(name) => {
                let addr = self.local_vars.get(name).cloned()?;
                let temp = self.new_temp();
                self.emit(IrInstruction::Load {
                    dest: temp.clone(),
                    ptr: addr,
                    ty: IrType::I32,
                });
                Some(IrOperand::Register(temp))
            }
            Expr::BinaryOp { lhs, op, rhs } => {
                match op {
                    AstBinaryOp::And => return self.generate_logical_and(lhs, rhs),
                    AstBinaryOp::Or => return self.generate_logical_or(lhs, rhs),
                    _ => {}
                }

                let left_val = self.generate_expr(lhs)?;
                let right_val = self.generate_expr(rhs)?;
                let dest = self.new_temp();

                let ir_op = match op {
                    AstBinaryOp::Add => BinaryOp::Add,
                    AstBinaryOp::Sub => BinaryOp::Sub,
                    AstBinaryOp::Mul => BinaryOp::Mul,
                    AstBinaryOp::Div => BinaryOp::Div,
                    AstBinaryOp::Rem => BinaryOp::Mod,
                    AstBinaryOp::BitAnd => BinaryOp::BitwiseAnd,
                    AstBinaryOp::BitOr => BinaryOp::BitwiseOr,
                    AstBinaryOp::BitXor => BinaryOp::BitwiseXor,
                    AstBinaryOp::Eq => BinaryOp::Eq,
                    AstBinaryOp::Ne => BinaryOp::Ne,
                    AstBinaryOp::Lt => BinaryOp::Lt,
                    AstBinaryOp::Le => BinaryOp::Le,
                    AstBinaryOp::Gt => BinaryOp::Gt,
                    AstBinaryOp::Ge => BinaryOp::Ge,
                    _ => unreachable!(),
                };

                self.emit(IrInstruction::Binary {
                    dest: dest.clone(),
                    op: ir_op,
                    left: left_val,
                    right: right_val,
                });
                Some(IrOperand::Register(dest))
            }
            Expr::UnaryPrefixOp { op, rhs } => match op {
                PrefixOp::Neg => {
                    let val = self.generate_expr(rhs)?;
                    let dest = self.new_temp();
                    self.emit(IrInstruction::Unary {
                        dest: dest.clone(),
                        op: UnaryOp::Neg,
                        operand: val,
                    });
                    Some(IrOperand::Register(dest))
                }
                PrefixOp::Not => {
                    let val = self.generate_expr(rhs)?;
                    let dest = self.new_temp();
                    self.emit(IrInstruction::Unary {
                        dest: dest.clone(),
                        op: UnaryOp::LogicalNot,
                        operand: val,
                    });
                    Some(IrOperand::Register(dest))
                }
                PrefixOp::Address => {
                    if let Expr::Ident(name) = &**rhs {
                        let addr = self.local_vars.get(name).cloned()?;
                        Some(IrOperand::Register(addr))
                    } else {
                        None
                    }
                }
                PrefixOp::Deref => {
                    let ptr = self.generate_expr(rhs)?;
                    let dest = self.new_temp();
                    if let IrOperand::Register(ptr_reg) = ptr {
                        self.emit(IrInstruction::Load {
                            dest: dest.clone(),
                            ptr: ptr_reg,
                            ty: IrType::I32,
                        });
                        Some(IrOperand::Register(dest))
                    } else {
                        None
                    }
                }
                PrefixOp::PreInc => {
                    if let Expr::Ident(name) = &**rhs {
                        let addr = self.local_vars.get(name).cloned()?;
                        let old_val = self.new_temp();
                        self.emit(IrInstruction::Load {
                            dest: old_val.clone(),
                            ptr: addr.clone(),
                            ty: IrType::I32,
                        });

                        let new_val = self.new_temp();
                        self.emit(IrInstruction::Binary {
                            dest: new_val.clone(),
                            op: BinaryOp::Add,
                            left: IrOperand::Register(old_val),
                            right: IrOperand::Constant(1),
                        });

                        self.emit(IrInstruction::Store {
                            value: IrOperand::Register(new_val.clone()),
                            ptr: addr,
                        });

                        Some(IrOperand::Register(new_val))
                    } else {
                        None
                    }
                }
                PrefixOp::PreDec => {
                    if let Expr::Ident(name) = &**rhs {
                        let addr = self.local_vars.get(name).cloned()?;
                        let old_val = self.new_temp();
                        self.emit(IrInstruction::Load {
                            dest: old_val.clone(),
                            ptr: addr.clone(),
                            ty: IrType::I32,
                        });

                        let new_val = self.new_temp();
                        self.emit(IrInstruction::Binary {
                            dest: new_val.clone(),
                            op: BinaryOp::Sub,
                            left: IrOperand::Register(old_val),
                            right: IrOperand::Constant(1),
                        });

                        self.emit(IrInstruction::Store {
                            value: IrOperand::Register(new_val.clone()),
                            ptr: addr,
                        });

                        Some(IrOperand::Register(new_val))
                    } else {
                        None
                    }
                }
            },
            Expr::UnaryPostfixOp { lhs, op } => match op {
                PostfixOp::PostInc => {
                    if let Expr::Ident(name) = &**lhs {
                        let addr = self.local_vars.get(name).cloned()?;
                        let old_val = self.new_temp();
                        self.emit(IrInstruction::Load {
                            dest: old_val.clone(),
                            ptr: addr.clone(),
                            ty: IrType::I32,
                        });

                        let new_val = self.new_temp();
                        self.emit(IrInstruction::Binary {
                            dest: new_val.clone(),
                            op: BinaryOp::Add,
                            left: IrOperand::Register(old_val.clone()),
                            right: IrOperand::Constant(1),
                        });

                        self.emit(IrInstruction::Store {
                            value: IrOperand::Register(new_val),
                            ptr: addr,
                        });

                        Some(IrOperand::Register(old_val))
                    } else {
                        None
                    }
                }
                PostfixOp::PostDec => {
                    if let Expr::Ident(name) = &**lhs {
                        let addr = self.local_vars.get(name).cloned()?;
                        let old_val = self.new_temp();
                        self.emit(IrInstruction::Load {
                            dest: old_val.clone(),
                            ptr: addr.clone(),
                            ty: IrType::I32,
                        });

                        let new_val = self.new_temp();
                        self.emit(IrInstruction::Binary {
                            dest: new_val.clone(),
                            op: BinaryOp::Sub,
                            left: IrOperand::Register(old_val.clone()),
                            right: IrOperand::Constant(1),
                        });

                        self.emit(IrInstruction::Store {
                            value: IrOperand::Register(new_val),
                            ptr: addr,
                        });

                        Some(IrOperand::Register(old_val))
                    } else {
                        None
                    }
                }
            },
            Expr::Assignment { left, op, right } => {
                let rhs = self.generate_expr(right)?;

                match &**left {
                    Expr::Ident(name) => {
                        let addr = self.local_vars.get(name).cloned()?;
                        let final_value = match op {
                            AssignOp::Assign => rhs,
                            _ => {
                                let lhs = self.generate_expr(left)?;
                                let dest = self.new_temp();
                                let ir_op = match op {
                                    AssignOp::PlusAssign => BinaryOp::Add,
                                    AssignOp::MinusAssign => BinaryOp::Sub,
                                    AssignOp::MulAssign => BinaryOp::Mul,
                                    AssignOp::DivAssign => BinaryOp::Div,
                                    AssignOp::RemAssign => BinaryOp::Mod,
                                    AssignOp::BitAndAssign => BinaryOp::BitwiseAnd,
                                    AssignOp::BitOrAssign => BinaryOp::BitwiseOr,
                                    AssignOp::BitXorAssign => BinaryOp::BitwiseXor,
                                    _ => return None,
                                };
                                self.emit(IrInstruction::Binary {
                                    dest: dest.clone(),
                                    op: ir_op,
                                    left: lhs,
                                    right: rhs,
                                });
                                IrOperand::Register(dest)
                            }
                        };
                        self.emit(IrInstruction::Store {
                            value: final_value.clone(),
                            ptr: addr,
                        });
                        Some(final_value)
                    }
                    Expr::UnaryPrefixOp {
                        op: PrefixOp::Deref,
                        rhs: deref_target,
                    } => {
                        if let Some(ptr) = self.generate_expr(deref_target) {
                            if let IrOperand::Register(ptr_reg) = ptr {
                                self.emit(IrInstruction::Store {
                                    value: rhs.clone(),
                                    ptr: ptr_reg,
                                });
                                Some(rhs)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Expr::ArrayIndex { array, index } => {
                        if let Expr::Ident(name) = &**array {
                            let base_addr = self.local_vars.get(name).cloned()?;
                            let idx = self.generate_expr(index)?;
                            let elem_addr = self.new_temp();
                            self.emit(IrInstruction::GetElementPtr {
                                dest: elem_addr.clone(),
                                ptr: base_addr,
                                indices: vec![IrOperand::Constant(0), idx],
                                ty: IrType::Array(Box::new(IrType::I32), 0),
                            });
                            self.emit(IrInstruction::Store {
                                value: rhs.clone(),
                                ptr: elem_addr,
                            });
                            Some(rhs)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            Expr::Call { func, args } => {
                let func_name = if let Expr::Ident(name) = &**func {
                    name.clone()
                } else {
                    return None;
                };

                let arguments: Vec<IrOperand> = args
                    .iter()
                    .filter_map(|arg| self.generate_expr(arg))
                    .collect();

                let dest = if self.is_void_function(&func_name) {
                    None
                } else {
                    Some(self.new_temp())
                };

                self.emit(IrInstruction::Call {
                    dest: dest.clone(),
                    func: func_name,
                    args: arguments,
                });

                dest.map(IrOperand::Register)
            }
            Expr::ArrayIndex { array, index } => {
                if let Expr::Ident(name) = &**array {
                    let base_addr = self.local_vars.get(name).cloned()?;
                    let idx = self.generate_expr(index)?;
                    let elem_addr = self.new_temp();
                    self.emit(IrInstruction::GetElementPtr {
                        dest: elem_addr.clone(),
                        ptr: base_addr,
                        indices: vec![IrOperand::Constant(0), idx],
                        ty: IrType::Array(Box::new(IrType::I32), 0),
                    });

                    let result = self.new_temp();
                    self.emit(IrInstruction::Load {
                        dest: result.clone(),
                        ptr: elem_addr,
                        ty: IrType::I32,
                    });
                    Some(IrOperand::Register(result))
                } else {
                    None
                }
            }
            Expr::InitializerList(_) => None,
        }
    }

    fn generate_logical_and(&mut self, left: &Expr, right: &Expr) -> Option<IrOperand> {
        let and_right_label = self.new_label("and.right");
        let false_label = self.new_label("and.false");
        let end_label = self.new_label("and.end");
        let result = self.new_temp();

        let left_val = self.generate_expr(left)?;
        self.emit(IrInstruction::CondJump {
            cond: left_val,
            true_label: and_right_label.clone(),
            false_label: false_label.clone(),
        });

        self.new_block(and_right_label);
        let right_val = self.generate_expr(right)?;
        self.emit(IrInstruction::Store {
            value: right_val,
            ptr: result.clone(),
        });
        self.emit(IrInstruction::Jump(end_label.clone()));

        self.new_block(false_label);
        self.emit(IrInstruction::Store {
            value: IrOperand::Constant(0),
            ptr: result.clone(),
        });
        self.emit(IrInstruction::Jump(end_label.clone()));

        self.new_block(end_label);
        let final_result = self.new_temp();
        self.emit(IrInstruction::Load {
            dest: final_result.clone(),
            ptr: result,
            ty: IrType::I32,
        });
        Some(IrOperand::Register(final_result))
    }

    fn generate_logical_or(&mut self, left: &Expr, right: &Expr) -> Option<IrOperand> {
        let or_right_label = self.new_label("or.right");
        let true_label = self.new_label("or.true");
        let end_label = self.new_label("or.end");
        let result = self.new_temp();

        let left_val = self.generate_expr(left)?;
        self.emit(IrInstruction::CondJump {
            cond: left_val,
            true_label: true_label.clone(),
            false_label: or_right_label.clone(),
        });

        self.new_block(or_right_label);
        let right_val = self.generate_expr(right)?;
        self.emit(IrInstruction::Store {
            value: right_val,
            ptr: result.clone(),
        });
        self.emit(IrInstruction::Jump(end_label.clone()));

        self.new_block(true_label);
        self.emit(IrInstruction::Store {
            value: IrOperand::Constant(1),
            ptr: result.clone(),
        });
        self.emit(IrInstruction::Jump(end_label.clone()));

        self.new_block(end_label);
        let final_result = self.new_temp();
        self.emit(IrInstruction::Load {
            dest: final_result.clone(),
            ptr: result,
            ty: IrType::I32,
        });
        Some(IrOperand::Register(final_result))
    }

    fn convert_type(&self, ty: &TypeSpecifier) -> IrType {
        match ty {
            TypeSpecifier::Int => IrType::I32,
            TypeSpecifier::Char => IrType::I8,
            TypeSpecifier::Void => IrType::Void,
            TypeSpecifier::Pointer(inner) => IrType::Ptr(Box::new(self.convert_type(inner))),
        }
    }

    fn is_void_function(&self, name: &str) -> bool {
        self.functions
            .iter()
            .any(|f| f.name == name && f.return_type == IrType::Void)
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}.{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    fn new_block(&mut self, label: String) {
        self.current_blocks.push(IrBlock {
            label,
            instructions: Vec::new(),
        });
        self.current_block_index = self.current_blocks.len() - 1;
    }

    fn emit(&mut self, inst: IrInstruction) {
        self.current_blocks[self.current_block_index]
            .instructions
            .push(inst);
    }

    fn has_terminator(&self) -> bool {
        self.current_blocks[self.current_block_index]
            .instructions
            .iter()
            .any(|inst| {
                matches!(
                    inst,
                    IrInstruction::Return(_)
                        | IrInstruction::Jump(_)
                        | IrInstruction::CondJump { .. }
                )
            })
    }

    fn find_alloca_insert_point(&self) -> usize {
        for (i, inst) in self.current_blocks[0].instructions.iter().enumerate() {
            if !matches!(inst, IrInstruction::Alloca { .. }) {
                return i;
            }
        }
        self.current_blocks[0].instructions.len()
    }
}
