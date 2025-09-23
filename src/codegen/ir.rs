use std::fmt;

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: IrType,
    pub locals: Vec<(String, IrType)>,
    pub blocks: Vec<IrBlock>,
}

#[derive(Debug, Clone)]
pub struct IrBlock {
    pub label: String,
    pub instructions: Vec<IrInstruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    I32,
    I8,
    Void,
    Ptr(Box<IrType>),
    Array(Box<IrType>, usize),
}

#[derive(Debug, Clone)]
pub enum IrInstruction {
    Label(String),
    Alloca {
        dest: String,
        ty: IrType,
    },
    Store {
        value: IrOperand,
        ptr: String,
    },
    Load {
        dest: String,
        ptr: String,
        ty: IrType,
    },
    Binary {
        dest: String,
        op: BinaryOp,
        left: IrOperand,
        right: IrOperand,
    },
    Unary {
        dest: String,
        op: UnaryOp,
        operand: IrOperand,
    },
    Call {
        dest: Option<String>,
        func: String,
        args: Vec<IrOperand>,
    },
    Jump(String),
    CondJump {
        cond: IrOperand,
        true_label: String,
        false_label: String,
    },
    Return(Option<IrOperand>),
    GetElementPtr {
        dest: String,
        ptr: String,
        indices: Vec<IrOperand>,
        ty: IrType,
    },
    Cast {
        dest: String,
        value: IrOperand,
        from_ty: IrType,
        to_ty: IrType,
    },
}

#[derive(Debug, Clone)]
pub enum IrOperand {
    Register(String),
    Constant(i32),
    CharConstant(u8),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    BitwiseNot,
    LogicalNot,
}

impl fmt::Display for IrProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

impl fmt::Display for IrFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "define {} @{}(", self.return_type, self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "%{}", param)?;
        }
        writeln!(f, ") {{")?;

        for (name, ty) in &self.locals {
            writeln!(f, "  %{} = alloca {}", name, ty)?;
        }

        for block in &self.blocks {
            if !block.label.is_empty() && block.label != "entry" {
                writeln!(f, "{}:", block.label)?;
            }
            for inst in &block.instructions {
                writeln!(f, "  {}", inst)?;
            }
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for IrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrType::I32 => write!(f, "i32"),
            IrType::I8 => write!(f, "i8"),
            IrType::Void => write!(f, "void"),
            IrType::Ptr(ty) => write!(f, "{}*", ty),
            IrType::Array(ty, size) => write!(f, "[{} x {}]", size, ty),
        }
    }
}

impl fmt::Display for IrInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrInstruction::Label(label) => write!(f, "{}:", label),
            IrInstruction::Alloca { dest, ty } => write!(f, "%{} = alloca {}", dest, ty),
            IrInstruction::Store { value, ptr } => write!(f, "store {}, %{}", value, ptr),
            IrInstruction::Load { dest, ptr, ty } => write!(f, "%{} = load {}, %{}", dest, ty, ptr),
            IrInstruction::Binary {
                dest,
                op,
                left,
                right,
            } => {
                write!(f, "%{} = {} {}, {}", dest, op, left, right)
            }
            IrInstruction::Unary { dest, op, operand } => {
                write!(f, "%{} = {} {}", dest, op, operand)
            }
            IrInstruction::Call { dest, func, args } => {
                if let Some(dest) = dest {
                    write!(f, "%{} = call @{}(", dest, func)?;
                } else {
                    write!(f, "call @{}(", func)?;
                }
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            IrInstruction::Jump(label) => write!(f, "br label %{}", label),
            IrInstruction::CondJump {
                cond,
                true_label,
                false_label,
            } => {
                write!(
                    f,
                    "br {}, label %{}, label %{}",
                    cond, true_label, false_label
                )
            }
            IrInstruction::Return(value) => {
                if let Some(val) = value {
                    write!(f, "ret {}", val)
                } else {
                    write!(f, "ret void")
                }
            }
            IrInstruction::GetElementPtr {
                dest,
                ptr,
                indices,
                ty,
            } => {
                write!(f, "%{} = getelementptr {}, %{}", dest, ty, ptr)?;
                for idx in indices {
                    write!(f, ", {}", idx)?;
                }
                Ok(())
            }
            IrInstruction::Cast {
                dest,
                value,
                from_ty,
                to_ty,
            } => {
                let op = match (from_ty, to_ty) {
                    (IrType::I8, IrType::I32) => "zext",
                    (IrType::I32, IrType::I8) => "trunc",
                    (IrType::Ptr(_), IrType::Ptr(_)) => "bitcast",
                    _ => "cast",
                };
                write!(f, "%{} = {} {} {} to {}", dest, op, from_ty, value, to_ty)
            }
        }
    }
}

impl fmt::Display for IrOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrOperand::Register(name) => write!(f, "%{}", name),
            IrOperand::Constant(val) => write!(f, "{}", val),
            IrOperand::CharConstant(val) => write!(f, "{}", val),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            BinaryOp::Add => "add",
            BinaryOp::Sub => "sub",
            BinaryOp::Mul => "mul",
            BinaryOp::Div => "sdiv",
            BinaryOp::Mod => "srem",
            BinaryOp::BitwiseAnd => "and",
            BinaryOp::BitwiseOr => "or",
            BinaryOp::BitwiseXor => "xor",
            BinaryOp::Shl => "shl",
            BinaryOp::Shr => "ashr",
            BinaryOp::Eq => "icmp eq",
            BinaryOp::Ne => "icmp ne",
            BinaryOp::Lt => "icmp slt",
            BinaryOp::Le => "icmp sle",
            BinaryOp::Gt => "icmp sgt",
            BinaryOp::Ge => "icmp sge",
        };
        write!(f, "{}", op)
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            UnaryOp::Neg => "sub 0,",
            UnaryOp::BitwiseNot => "xor -1,",
            UnaryOp::LogicalNot => "icmp eq 0,",
        };
        write!(f, "{}", op)
    }
}
