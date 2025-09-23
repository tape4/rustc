pub mod ir;
pub mod ir_gen;

pub use ir::{IrFunction, IrInstruction, IrOperand, IrProgram};
pub use ir_gen::IrGenerator;
