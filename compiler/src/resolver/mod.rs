use crate::ast::{Expr, Item, Pattern, Stmt, Type, Visibility};
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

    pub fn resolve(mut self, items: Vec<crate::ast::AstNode<Item>>) -> Result<Vec<crate::hir::HirItem>, DiagnosticBag> {
        for item in items {
            self.resolve_item(item)?;
        }

        if self.diagnostics.has_errors() {
            Err(self.diagnostics)
        } else {
            // TODO: return resolved HIR
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

    fn resolve_name(&self, name: &str) -> Option<&ResolvedItem> {
        let mut scope_idx = self.current_scope;
        loop {
            let scope = &self.scopes[scope_idx];
            if let Some(item) = scope.items.get(name) {
                return Some(item);
            }
            if let Some(parent) = scope.parent {
                scope_idx = parent;
            } else {
                break;
            }
        }
        None
    }

    fn resolve_item(&mut self, item: crate::ast::AstNode<Item>) -> Result<(), ()> {
        match item.node {
            Item::Fn(f) => {
                self.declare(f.name.clone(), ResolvedItem::Function { name: f.name.clone(), generics: Vec::new() }, item.span)?;
                self.enter_scope();
                for param in f.sig.params {
                    self.declare(param.name, ResolvedItem::Parameter { name: param.name, mutable: param.mutable }, param.ty.span)?;
                }
                self.resolve_block(&f.body)?;
                self.exit_scope();
            }
            Item::Struct(s) => {
                self.declare(s.name.clone(), ResolvedItem::Struct { name: s.name.clone(), generics: s.generics.iter().map(|g| g.name.clone()).collect() }, item.span)?;
            }
            Item::Enum(e) => {
                self.declare(e.name.clone(), ResolvedItem::Enum { name: e.name.clone(), generics: e.generics.iter().map(|g| g.name.clone()).collect() }, item.span)?;
            }
            Item::Trait(t) => {
                self.declare(t.name.clone(), ResolvedItem::Trait { name: t.name.clone(), generics: t.generics.iter().map(|g| g.name.clone()).collect() }, item.span)?;
            }
            Item::TypeAlias(t) => {
                self.declare(t.name.clone(), ResolvedItem::TypeAlias { name: t.name.clone() }, item.span)?;
            }
            Item::Const(c) => {
                self.declare(c.name.clone(), ResolvedItem::Const { name: c.name.clone() }, item.span)?;
                self.resolve_expr(&c.value)?;
            }
            Item::Use(u) => {
                self.resolve_use(&u)?;
            }
            Item::Mod(m) => {
                self.declare(m.name.clone(), ResolvedItem::Module { name: m.name.clone() }, item.span)?;
                self.enter_scope();
                for item in m.items {
                    self.resolve_item(item)?;
                }
                self.exit_scope();
            }
            Item::Impl(i) => {
                // impl doesn't introduce a name
                self.enter_scope();
                for item in i.items {
                    match item {
                        crate::ast::ImplItemKind::Fn(f) => {
                            self.resolve_item(crate::ast::AstNode::new(Item::Fn(f), item.span))?;
                        }
                        _ => {}
                    }
                }
                self.exit_scope();
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_use(&mut self, use_item: &UseItem) -> Result<(), ()> {
        // TODO: implement use resolution
        Ok(())
    }

    fn resolve_block(&mut self, block: &crate::ast::BlockExpr) -> Result<(), ()> {
        self.enter_scope();
        for stmt in &block.stmts {
            self.resolve_stmt(stmt)?;
        }
        self.exit_scope();
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &crate::ast::AstNode<Stmt>) -> Result<(), ()> {
        match &stmt.node {
            Stmt::Let { pattern, ty, value, mutable } => {
                self.bind_pattern(pattern, *mutable, span)?;
                if let Some(ty) = ty {
                    self.resolve_type(ty)?;
                }
                self.resolve_expr(value)?;
            }
            Stmt::Expr(expr) => {
                self.resolve_expr(expr)?;
            }
            Stmt::Item(item) => {
                self.resolve_item(item.clone())?;
            }
            Stmt::Empty => {}
        }
        Ok(())
    }

    fn bind_pattern(&mut self, pattern: &Pattern, mutable: bool, span: Span) -> Result<(), ()> {
        match pattern {
            Pattern::Var(name) => {
                self.declare(name.clone(), ResolvedItem::Variable { name: name.clone(), mutable }, span)?;
            }
            Pattern::Tuple(patterns) | Pattern::Array(patterns) => {
                for p in patterns {
                    self.bind_pattern(p, mutable, span)?;
                }
            }
            Pattern::Binding { name, pattern } => {
                self.declare(name.clone(), ResolvedItem::Variable { name: name.clone(), mutable }, span)?;
                self.bind_pattern(pattern, mutable, span)?;
            }
            Pattern::Constructor { args, .. } => {
                for arg in args {
                    self.bind_pattern(arg, mutable, span)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), ()> {
        match expr {
            Expr::Var(name) => {
                if self.resolve_name(name).is_none() {
                    self.diagnostics.error(format!("undefined variable '{}'", name), Span::default());
                }
            }
            Expr::Call(func, args) => {
                self.resolve_expr(func)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::MethodCall(receiver, _, args) => {
                self.resolve_expr(receiver)?;
                for arg in args {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Field(expr, _) => {
                self.resolve_expr(expr)?;
            }
            Expr::Index(expr, idx) => {
                self.resolve_expr(expr)?;
                self.resolve_expr(idx)?;
            }
            Expr::Binary(_, lhs, rhs) => {
                self.resolve_expr(lhs)?;
                self.resolve_expr(rhs)?;
            }
            Expr::Unary(_, expr) => {
                self.resolve_expr(expr)?;
            }
            Expr::Cast(expr, ty) => {
                self.resolve_expr(expr)?;
                self.resolve_type(ty)?;
            }
            Expr::If(cond, then_block, else_block) => {
                self.resolve_expr(cond)?;
                self.resolve_block(then_block)?;
                if let Some(else_block) = else_block {
                    self.resolve_block(else_block)?;
                }
            }
            Expr::Match(expr, arms) => {
                self.resolve_expr(expr)?;
                for arm in arms {
                    self.bind_pattern(&arm.pattern, false, arm.pattern.span())?;
                    if let Some(guard) = &arm.guard {
                        self.resolve_expr(guard)?;
                    }
                    self.resolve_expr(&arm.body)?;
                }
            }
            Expr::While(cond, body) => {
                self.resolve_expr(cond)?;
                self.resolve_block(body)?;
            }
            Expr::For(pattern, iter, body) => {
                self.bind_pattern(pattern, false, pattern.span())?;
                self.resolve_expr(iter)?;
                self.resolve_block(body)?;
            }
            Expr::Loop(body) => {
                self.resolve_block(body)?;
            }
            Expr::Block(block) => {
                self.resolve_block(block)?;
            }
            Expr::AsyncBlock(block) => {
                self.resolve_block(block)?;
            }
            Expr::Await(expr) => {
                self.resolve_expr(expr)?;
            }
            Expr::Spawn(expr) => {
                self.resolve_expr(expr)?;
            }
            Expr::Nursery(stmts) => {
                for stmt in stmts {
                    self.resolve_stmt(&crate::ast::AstNode::new(stmt.clone(), Span::default()))?;
                }
            }
            Expr::UnsafeBlock(stmts) => {
                for stmt in stmts {
                    self.resolve_stmt(&crate::ast::AstNode::new(stmt.clone(), Span::default()))?;
                }
            }
            Expr::ComptimeBlock(stmts) => {
                for stmt in stmts {
                    self.resolve_stmt(&crate::ast::AstNode::new(stmt.clone(), Span::default()))?;
                }
            }
            Expr::StructLit { fields, .. } => {
                for field in fields {
                    self.resolve_expr(&field.value)?;
                }
            }
            Expr::ArrayLit(items) | Expr::TupleLit(items) => {
                for item in items {
                    self.resolve_expr(item)?;
                }
            }
            Expr::Closure { params, body, .. } => {
                self.enter_scope();
                for param in params {
                    self.declare(param.name.clone(), ResolvedItem::Parameter { name: param.name.clone(), mutable: param.mutable }, param.ty.span)?;
                }
                self.resolve_expr(body)?;
                self.exit_scope();
            }
            Expr::Literal(_) | Expr::Break(_) | Expr::Continue | Expr::Return(_) => {}
        }
        Ok(())
    }

    fn resolve_type(&mut self, ty: &Type) -> Result<(), ()> {
        match ty {
            Type::Path(path) => {
                if self.resolve_name(&path.segments[0]).is_none() && path.segments.len() == 1 {
                    // Could be a primitive
                }
                for arg in &path.args {
                    self.resolve_type(arg)?;
                }
            }
            Type::Group(ty) => self.resolve_type(ty)?,
            Type::Array(ty, size) => {
                self.resolve_type(ty)?;
                if let Some(size) = size {
                    self.resolve_expr(size)?;
                }
            }
            Type::Ref(ty, _) => self.resolve_type(ty)?,
            Type::Fn(params, ret) => {
                for p in params {
                    self.resolve_type(p)?;
                }
                self.resolve_type(ret)?;
            }
            Type::Tuple(tys) => {
                for ty in tys {
                    self.resolve_type(ty)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}