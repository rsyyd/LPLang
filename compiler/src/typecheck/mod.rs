use crate::ast::{Expr, FnParam, FnSig, Item, Pattern, Stmt, Type, Visibility};
use crate::diagnostics::{DiagnosticBag, Span};
use crate::hir::{HirExpr, HirFunction, HirItem, HirModule, HirPackage, HirPattern, HirStmt, HirType, PrimitiveTy};
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

    fn get(&self, name: &str) -> Option<&HirType> {
        self.vars.get(name)
    }

    fn push_generics(&mut self, generics: Vec<String>) {
        self.generics.extend(generics);
    }

    fn pop_generics(&mut self, count: usize) {
        self.generics.truncate(self.generics.len().saturating_sub(count));
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

    pub fn check(mut self, items: Vec<HirItem>) -> Result<HirPackage, DiagnosticBag> {
        // TODO: implement type checking
        if self.diagnostics.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(HirPackage { modules: Vec::new() })
        }
    }

    fn fresh_type_var(&mut self) -> HirType {
        let var = self.next_type_var;
        self.next_type_var += 1;
        HirType::Var(var)
    }

    fn unify(&mut self, expected: &HirType, found: &HirType, span: Span) -> Result<(), ()> {
        // TODO: implement unification
        Ok(())
    }

    fn check_expr(&mut self, expr: &Expr, expected: Option<&HirType>) -> HirType {
        // TODO: implement expression type checking
        HirType::Primitive(PrimitiveTy::Unit)
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        // TODO
    }

    fn check_pattern(&mut self, pattern: &Pattern, expected: &HirType) {
        // TODO
    }
}