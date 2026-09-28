use crate::ast::{Expr, Item, Pattern, Stmt, Type, UseItem};
use crate::diagnostics::{DiagnosticBag, Span};
use std::collections::HashMap;

pub struct Resolver {
    scopes: Vec<Scope>,
    current_scope: usize,
    diagnostics: DiagnosticBag,
    next_scope_id: usize,
}

#[derive(Debug, Clone)]
struct Scope {
    id: usize,
    parent: Option<usize>,
    items: HashMap<String, ResolvedItem>,
}

#[derive(Debug, Clone)]
enum ResolvedItem {
    Function { name: String, generics: Vec<String> },
    Struct { name: String, generics: Vec<String> },
    Enum { name: String, generics: Vec<String> },
    Trait { name: String, generics: Vec<String> },
    TypeAlias { name: String },
    Const { name: String },
    Module { name: String },
    Variable { name: String, mutable: bool },
    Parameter { name: String, mutable: bool },
}

impl Resolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            scopes: Vec::new(),
            current_scope: 0,
            diagnostics: DiagnosticBag::new(),
            next_scope_id: 1,
        };
        resolver.enter_scope(); // global scope
        resolver
    }

    pub fn resolve(mut self, _items: Vec<crate::ast::AstNode<Item>>) -> Result<Vec<crate::hir::HirItem>, DiagnosticBag> {
        // TODO: implement resolver
        if self.diagnostics.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(Vec::new())
        }
    }

    fn enter_scope(&mut self) {
        let id = self.next_scope_id;
        self.next_scope_id += 1;
        let parent = if self.scopes.is_empty() { None } else { Some(self.current_scope) };
        self.scopes.push(Scope {
            id,
            parent,
            items: HashMap::new(),
        });
        self.current_scope = id - 1;
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            self.current_scope = scope.parent.unwrap_or(0);
        }
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        &mut self.scopes[self.current_scope]
    }

    fn declare(&mut self, name: String, item: ResolvedItem, span: Span) -> Result<(), ()> {
        let scope = self.current_scope_mut();
        if scope.items.contains_key(&name) {
            self.diagnostics.error(format!("'{}' already declared in this scope", name), span);
            return Err(());
        }
        scope.items.insert(name, item);
        Ok(())
    }

    fn _resolve_name(&self, _name: &str) -> Option<&ResolvedItem> {
        None
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}