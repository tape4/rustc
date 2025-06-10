use crate::ast;
use crate::semantic::symbol::error::SymbolError;
use ast::TypeSpecifier;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: TypeSpecifier,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Function { param_types: Vec<TypeSpecifier> },
    Variable,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }

    // 현재 스코프에 심볼 선언
    pub fn declare(&mut self, name: String, symbol: Symbol) -> Result<(), SymbolError> {
        let current = self.scopes.last_mut().unwrap();
        if current.contains_key(&name) {
            return Err(SymbolError::DuplicateDeclaration { name });
        }
        current.insert(name, symbol);
        Ok(())
    }

    // 이름을 가장 깊은 스코프부터 검색
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    // 새로운 스코프로 진입
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    // 현재 스코프 종료
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
