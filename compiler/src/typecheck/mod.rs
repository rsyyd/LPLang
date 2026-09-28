use crate::diagnostics::DiagnosticBag;
use crate::hir::{HirItem, HirPackage, HirType};
use std::collections::HashMap;

pub struct TypeChecker {
    diagnostics: DiagnosticBag,
    type_env: TypeEnv,
    next_type_var: usize,
}

#[derive(Debug, Clone)]
struct TypeEnv {
    vars: HashMap<String, HirType>,
    generics: Vec<String>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            generics: Vec::new(),
        }
    }

    fn insert(&mut self, name: String, ty: HirType) {
        self.vars.insert(name, ty);
    }

    fn _get(&self, _name: &str) -> Option<&HirType> {
        None
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            diagnostics: DiagnosticBag::new(),
            type_env: TypeEnv::new(),
            next_type_var: 0,
        }
    }

    pub fn check(self, _items: Vec<HirItem>) -> Result<HirPackage, DiagnosticBag> {
        // TODO: implement type checking
        if self.diagnostics.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(HirPackage { modules: Vec::new() })
        }
    }

    fn _fresh_type_var(&mut self) -> HirType {
        let var = self.next_type_var;
        self.next_type_var += 1;
        HirType::Var(var)
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}