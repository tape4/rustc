use crate::ast::{Function, TypeSpecifier};
use crate::semantic::resolver::error::ResolveError;
use crate::semantic::symbol::error::SymbolError;
use crate::semantic::symbol::symbol::{Symbol, SymbolKind, SymbolTable};

pub struct Resolver {
    pub table: SymbolTable,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            table: SymbolTable::new(),
        }
    }

    // 함수 선언을 심볼 테이블에 추가
    pub fn declare_function(&mut self, func: &Function) -> Result<(), SymbolError> {
        let name = func.name.clone();
        let return_ty = func.return_ty.clone();
        let param_types: Vec<TypeSpecifier> =
            func.params.iter().map(|param| param.ty.clone()).collect();

        let symbol = Symbol {
            ty: return_ty,
            kind: SymbolKind::Function { param_types },
        };

        self.table.declare(name, symbol)
    }

    // 변수 선언을 심볼 테이블에 추가
    pub fn declare_variable(&mut self, name: &str, ty: &TypeSpecifier) -> Result<(), SymbolError> {
        let symbol = Symbol {
            ty: ty.clone(),
            kind: SymbolKind::Variable,
        };

        let result = self.table.declare(name.to_string(), symbol);
        result
    }

    // 식별자 참조 시 심볼 테이블 조회
    pub fn resolve_identifier(&mut self, identifier: &str) -> Result<&Symbol, ResolveError> {
        if let Some(found_symbol) = self.table.lookup(identifier) {
            Ok(found_symbol)
        } else {
            Err(ResolveError::UndefinedSymbol {
                name: identifier.to_string(),
            })
        }
    }

    pub fn push_scope(&mut self) {
        self.table.push_scope();
    }

    pub fn pop_scope(&mut self) {
        self.table.pop_scope();
    }
}
