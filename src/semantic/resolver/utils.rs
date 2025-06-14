use crate::ast::{Function, TypeSpecifier};
use crate::semantic::resolver::error::ResolveError;
use crate::semantic::resolver::resolver::Resolver;
use crate::semantic::symbol::error::SymbolError;
use crate::semantic::symbol::symbol::{Symbol, SymbolKind, SymbolTable};

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            table: SymbolTable::new(),
            loop_depth: 0,
        }
    }

    // 함수 선언을 심볼 테이블에 추가
    pub fn declare_function(&mut self, func: &Function) -> Result<(), SymbolError> {
        let name = func.name.clone();
        let return_ty = func.return_ty.clone();
        let param_types: Vec<TypeSpecifier> =
            func.params.iter().map(|param| param.ty.clone()).collect();

        if let Some(existing) = self.table.lookup(&name) {
            // 같은 함수 시그니처인지 검사
            if let SymbolKind::Function { param_types: existing_params } = &existing.kind {
                if existing.ty == return_ty && *existing_params == param_types {
                    // 시그니처 일치: 중복 선언이 아니므로 무시
                    return Ok(());
                }
            }
            // 이름이 변수이거나 시그니처 불일치
            return Err(SymbolError::DuplicateDeclaration { name });
        }

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

    pub fn lookup_function(
        &mut self,
        name: &str,
    ) -> Result<(Vec<TypeSpecifier>, TypeSpecifier), ResolveError> {
        let sym = self.resolve_identifier(name)?;
        if let SymbolKind::Function { param_types } = &sym.kind {
            Ok((param_types.clone(), sym.ty.clone()))
        } else {
            Err(ResolveError::NotAFunciton {
                name: name.to_string(),
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
