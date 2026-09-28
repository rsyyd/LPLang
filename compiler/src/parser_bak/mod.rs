use crate::{
    ast::{self, AstNode, Expr, Item, Pattern, Stmt, Type, Visibility},
    diagnostics::{DiagnosticBag, Span},
    lexer::{SpannedToken, Token},
};
use ariadne::Label;
use anyhow::Result;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
    diagnostics: DiagnosticBag,
    filename: String,
    source: String,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>, filename: &str, source: &str) -> Self {
        Self {
            tokens,
            pos: 0,
            diagnostics: DiagnosticBag::new(),
            filename: filename.to_string(),
            source: source.to_string(),
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode<Item>> {
        let mut items = Vec::new();
        while !self.is_at_end() {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.advance();
            }
        }
        items
    }

    pub fn take_diagnostics(self) -> DiagnosticBag {
        self.diagnostics
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token(), Token::Error)
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.pos].token
    }

    fn current_span(&self) -> Span {
        self.tokens[self.pos].span
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1].token
    }

    fn check(&self, token: &Token) -> bool {
        !self.is_at_end() && std::mem::discriminant(self.current_token()) == std::mem::discriminant(token)
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: &Token, msg: &str) -> Result<()> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            let span = self.current_span();
            self.diagnostics.error(format!("Expected {}, found {:?}", msg, self.current_token()), span);
            Err(anyhow::anyhow!("Parse error"))
        }
    }

    fn parse_item(&mut self) -> Option<AstNode<Item>> {
        let visibility = self.parse_visibility();
        let start = self.current_span();

        let item = if self.match_token(&Token::Fn) {
            self.parse_fn_item(visibility, start)
        } else if self.match_token(&Token::Struct) {
            self.parse_struct_item(visibility, start)
        } else if self.match_token(&Token::Enum) {
            self.parse_enum_item(visibility, start)
        } else if self.match_token(&Token::Trait) {
            self.parse_trait_item(visibility, start)
        } else if self.match_token(&Token::Impl) {
            self.parse_impl_item(visibility, start)
        } else if self.match_token(&Token::Const) {
            self.parse_const_item(visibility, start)
        } else if self.match_token(&Token::Type) {
            self.parse_type_alias_item(visibility, start)
        } else if self.match_token(&Token::Use) {
            self.parse_use_item(visibility, start)
        } else if self.match_token(&Token::Mod) {
            self.parse_mod_item(visibility, start)
        } else if self.match_token(&Token::Extern) {
            self.parse_extern_block(visibility, start)
        } else if self.match_token(&Token::Comptime) {
            self.parse_comptime_block(visibility, start)
        } else {
            self.diagnostics.error("Expected item", self.current_span());
            self.sync_to_next_item();
            return None;
        };

        item.map(|node| AstNode::new(node, start.merge(self.previous_span())))
    }

    fn parse_visibility(&mut self) -> Visibility {
        if self.match_token(&Token::Pub) {
            Visibility::Public
        } else {
            Visibility::Private
        }
    }

    fn parse_fn_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        let sig = self.parse_fn_sig()?;
        let body = self.parse_block_expr()?;
        Some(Item::Fn(ast::FnItem {
            name,
            sig,
            body,
            is_async: false,
            is_comptime: false,
            visibility,
        }))
    }

    fn parse_fn_sig(&mut self) -> Option<ast::FnSig> {
        self.expect(&Token::LParen, "(")?;
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                params.push(self.parse_fn_param()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.expect(&Token::RParen, ")")?;

        let ret_ty = if self.match_token(&Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        Some(ast::FnSig { params, ret_ty })
    }

    fn parse_fn_param(&mut self) -> Option<ast::FnParam> {
        let mutable = self.match_token(&Token::Mut);
        let name = self.parse_ident()?;
        self.expect(&Token::Colon, ":")?;
        let ty = self.parse_type()?;
        Some(ast::FnParam { name, ty, mutable })
    }

    fn parse_struct_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        let generics = self.parse_generic_params().unwrap_or_default();
        self.expect(&Token::LBrace, "{")?;
        let mut fields = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            fields.push(self.parse_struct_field()?);
            self.match_token(&Token::Comma);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::Struct(ast::StructItem {
            name,
            generics,
            fields,
            visibility,
        }))
    }

    fn parse_struct_field(&mut self) -> Option<ast::StructField> {
        let name = self.parse_ident()?;
        self.expect(&Token::Colon, ":")?;
        let ty = self.parse_type()?;
        Some(ast::StructField { name, ty, visibility: Visibility::Private })
    }

    fn parse_enum_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        let generics = self.parse_generic_params().unwrap_or_default();
        self.expect(&Token::LBrace, "{")?;
        let mut variants = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            variants.push(self.parse_enum_variant()?);
            self.match_token(&Token::Comma);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::Enum(ast::EnumItem {
            name,
            generics,
            variants,
            visibility,
        }))
    }

    fn parse_enum_variant(&mut self) -> Option<ast::EnumVariant> {
        let name = self.parse_ident()?;
        let ty = if self.match_token(&Token::LParen) {
            let t = self.parse_type()?;
            self.expect(&Token::RParen, ")")?;
            Some(t)
        } else {
            None
        };
        Some(ast::EnumVariant { name, ty })
    }

    fn parse_trait_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        let generics = self.parse_generic_params().unwrap_or_default();
        self.expect(&Token::LBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            items.push(self.parse_trait_item_kind()?);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::Trait(ast::TraitItem {
            name,
            generics,
            items,
            visibility,
        }))
    }

    fn parse_trait_item_kind(&mut self) -> Option<ast::TraitItemKind> {
        if self.check(&Token::Fn) {
            self.advance();
            let name = self.parse_ident()?;
            let sig = self.parse_fn_sig()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::TraitItemKind::Fn(ast::FnSig { params: sig.params, ret_ty: sig.ret_ty }))
        } else if self.match_token(&Token::Type) {
            let name = self.parse_ident()?;
            let ty = if self.match_token(&Token::Assign) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(&Token::Semi, ";")?;
            Some(ast::TraitItemKind::TypeAlias { name, ty })
        } else if self.match_token(&Token::Const) {
            let name = self.parse_ident()?;
            self.expect(&Token::Colon, ":")?;
            let ty = self.parse_type()?;
            self.expect(&Token::Assign, "=")?;
            let value = self.parse_expr()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::TraitItemKind::Const { name, ty, value })
        } else {
            None
        }
    }

    fn parse_impl_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let generics = self.parse_generic_params().unwrap_or_default();
        let trait_ref = if self.check(&Token::Ident("".into())) && !self.check(&Token::For) {
            Some(self.parse_trait_ref()?)
        } else {
            None
        };
        self.expect(&Token::For, "for")?;
        let for_ty = self.parse_type()?;
        self.expect(&Token::LBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            items.push(self.parse_impl_item_kind()?);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::Impl(ast::ImplItem {
            generics,
            trait_ref,
            for_ty,
            items,
        }))
    }

    fn parse_impl_item_kind(&mut self) -> Option<ast::ImplItemKind> {
        if self.check(&Token::Fn) {
            self.advance();
            let name = self.parse_ident()?;
            let sig = self.parse_fn_sig()?;
            let body = self.parse_block_expr()?;
            Some(ast::ImplItemKind::Fn(ast::FnItem {
                name,
                sig,
                body,
                is_async: false,
                is_comptime: false,
                visibility: Visibility::Public,
            }))
        } else if self.match_token(&Token::Type) {
            let name = self.parse_ident()?;
            self.expect(&Token::Assign, "=")?;
            let ty = self.parse_type()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::ImplItemKind::TypeAlias { name, ty })
        } else if self.match_token(&Token::Const) {
            let name = self.parse_ident()?;
            self.expect(&Token::Colon, ":")?;
            let ty = self.parse_type()?;
            self.expect(&Token::Assign, "=")?;
            let value = self.parse_expr()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::ImplItemKind::Const { name, ty, value })
        } else {
            None
        }
    }

    fn parse_const_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        self.expect(&Token::Colon, ":")?;
        let ty = self.parse_type()?;
        self.expect(&Token::Assign, "=")?;
        let value = self.parse_expr()?;
        self.expect(&Token::Semi, ";")?;
        Some(Item::Const(ast::ConstItem { name, ty, value, visibility }))
    }

    fn parse_type_alias_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        self.expect(&Token::Assign, "=")?;
        let ty = self.parse_type()?;
        self.expect(&Token::Semi, ";")?;
        Some(Item::TypeAlias(ast::TypeAliasItem { name, ty, visibility }))
    }

    fn parse_use_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let tree = self.parse_use_tree()?;
        self.expect(&Token::Semi, ";")?;
        Some(Item::Use(ast::UseItem::Simple { name: tree.to_string(), alias: None }))
    }

    fn parse_use_tree(&mut self) -> Option<String> {
        let ident = self.parse_ident()?;
        if self.match_token(&Token::ColonColon) {
            let rest = self.parse_use_tree()?;
            Some(format!("{}::{}", ident, rest))
        } else {
            Some(ident)
        }
    }

    fn parse_mod_item(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let name = self.parse_ident()?;
        self.expect(&Token::LBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.advance();
            }
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::Mod(ast::ModItem { name, items, visibility }))
    }

    fn parse_extern_block(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        let abi = if self.check(&Token::String("".into())) {
            let s = self.parse_string_literal()?;
            s
        } else {
            "C".to_string()
        };
        self.expect(&Token::LBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            items.push(self.parse_extern_item()?);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::ExternBlock(ast::ExternBlock { abi, items }))
    }

    fn parse_extern_item(&mut self) -> Option<ast::ExternItem> {
        if self.match_token(&Token::Fn) {
            let name = self.parse_ident()?;
            let sig = self.parse_fn_sig()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::ExternItem::Fn { name, sig })
        } else if self.match_token(&Token::Static) {
            let name = self.parse_ident()?;
            self.expect(&Token::Colon, ":")?;
            let ty = self.parse_type()?;
            self.expect(&Token::Semi, ";")?;
            Some(ast::ExternItem::Static { name, ty })
        } else {
            None
        }
    }

    fn parse_comptime_block(&mut self, visibility: Visibility, start: Span) -> Option<Item> {
        self.expect(&Token::LBrace, "{")?;
        let mut items = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.advance();
            }
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Item::ComptimeBlock(ast::ComptimeBlock { items }))
    }

    fn parse_generic_params(&mut self) -> Option<Vec<ast::GenericParam>> {
        if !self.match_token(&Token::Lt) {
            return Some(Vec::new());
        }
        let mut params = Vec::new();
        if !self.check(&Token::Gt) {
            loop {
                params.push(self.parse_generic_param()?);
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        self.expect(&Token::Gt, ">")?;
        Some(params)
    }

    fn parse_generic_param(&mut self) -> Option<ast::GenericParam> {
        let name = self.parse_ident()?;
        let bounds = if self.match_token(&Token::Colon) {
            let mut bounds = Vec::new();
            loop {
                bounds.push(self.parse_trait_bound()?);
                if !self.match_token(&Token::Plus) {
                    break;
                }
            }
            bounds
        } else {
            Vec::new()
        };
        Some(ast::GenericParam { name, bounds })
    }

    fn parse_trait_bound(&mut self) -> Option<ast::TraitBound> {
        let trait_name = self.parse_ident()?;
        let args = if self.match_token(&Token::Lt) {
            let mut args = Vec::new();
            if !self.check(&Token::Gt) {
                loop {
                    args.push(self.parse_type()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::Gt, ">")?;
            args
        } else {
            Vec::new()
        };
        Some(ast::TraitBound { trait_name, args })
    }

    fn parse_trait_ref(&mut self) -> Option<ast::TraitRef> {
        let trait_name = self.parse_ident()?;
        let args = if self.match_token(&Token::Lt) {
            let mut args = Vec::new();
            if !self.check(&Token::Gt) {
                loop {
                    args.push(self.parse_type()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::Gt, ">")?;
            args
        } else {
            Vec::new()
        };
        Some(ast::TraitRef { trait_name, args })
    }

    fn parse_type(&mut self) -> Option<Type> {
        self.parse_type_path()
    }

    fn parse_type_path(&mut self) -> Option<Type> {
        let ident = self.parse_ident()?;
        let mut segments = vec![ident];
        while self.match_token(&Token::ColonColon) {
            segments.push(self.parse_ident()?);
        }
        let args = if self.match_token(&Token::Lt) {
            let mut args = Vec::new();
            if !self.check(&Token::Gt) {
                loop {
                    args.push(self.parse_type()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::Gt, ">")?;
            args
        } else {
            Vec::new()
        };
        Some(Type::Path(ast::TypePath { segments, args }))
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Option<Expr> {
        let left = self.parse_or()?;
        if self.match_token(&Token::Assign) {
            let right = self.parse_assignment()?;
            let span = left.span.merge(self.previous_span());
            Some(Expr::Assign(Box::new(left), Box::new(right)))
        } else {
            Some(left)
        }
    }

    fn parse_or(&mut self) -> Option<Expr> {
        let mut left = self.parse_and()?;
        while self.match_token(&Token::OrOr) {
            let right = self.parse_and()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(ast::BinOp::Or, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<Expr> {
        let mut left = self.parse_equality()?;
        while self.match_token(&Token::AndAnd) {
            let right = self.parse_equality()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(ast::BinOp::And, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison()?;
        while self.match_token(&Token::EqEq) || self.match_token(&Token::NotEq) {
            let op = if self.previous_token_was(&Token::EqEq) {
                ast::BinOp::Eq
            } else {
                ast::BinOp::Ne
            };
            let right = self.parse_comparison()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut left = self.parse_addition()?;
        while self.match_token(&Token::Lt)
            || self.match_token(&Token::LtEq)
            || self.match_token(&Token::Gt)
            || self.match_token(&Token::GtEq)
        {
            let op = match self.previous_token() {
                Token::Lt => ast::BinOp::Lt,
                Token::LtEq => ast::BinOp::Le,
                Token::Gt => ast::BinOp::Gt,
                Token::GtEq => ast::BinOp::Ge,
                _ => unreachable!(),
            };
            let right = self.parse_addition()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_addition(&mut self) -> Option<Expr> {
        let mut left = self.parse_multiplication()?;
        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let op = if self.previous_token_was(&Token::Plus) {
                ast::BinOp::Add
            } else {
                ast::BinOp::Sub
            };
            let right = self.parse_multiplication()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_multiplication(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;
        while self.match_token(&Token::Star)
            || self.match_token(&Token::Slash)
            || self.match_token(&Token::Percent)
        {
            let op = match self.previous_token() {
                Token::Star => ast::BinOp::Mul,
                Token::Slash => ast::BinOp::Div,
                Token::Percent => ast::BinOp::Rem,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            let span = left.span.merge(self.previous_span());
            left = Expr::Binary(op, Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if self.match_token(&Token::Not) {
            let expr = self.parse_unary()?;
            Some(Expr::Unary(ast::UnOp::Not, Box::new(expr)))
        } else if self.match_token(&Token::Minus) {
            let expr = self.parse_unary()?;
            Some(Expr::Unary(ast::UnOp::Neg, Box::new(expr)))
        } else if self.match_token(&Token::Star) {
            let expr = self.parse_unary()?;
            Some(Expr::Unary(ast::UnOp::Deref, Box::new(expr)))
        } else if self.match_token(&Token::And) {
            let expr = self.parse_unary()?;
            Some(Expr::Unary(ast::UnOp::Ref, Box::new(expr)))
        } else if self.match_token(&Token::And) && self.match_token(&Token::Mut) {
            let expr = self.parse_unary()?;
            Some(Expr::Unary(ast::UnOp::RefMut, Box::new(expr)))
        } else {
            self.parse_await()
        }
    }

    fn parse_await(&mut self) -> Option<Expr> {
        let mut expr = self.parse_postfix()?;
        while self.match_token(&Token::Await) {
            expr = Expr::Await(Box::new(expr));
        }
        Some(expr)
    }

    fn parse_postfix(&mut self) -> Option<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&Token::Dot) {
                let field = self.parse_ident()?;
                expr = Expr::Field(Box::new(expr), field);
            } else if self.match_token(&Token::LParen) {
                let mut args = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&Token::RParen, ")")?;
                expr = Expr::Call(Box::new(expr), args);
            } else if self.match_token(&Token::LBracket) {
                let index = self.parse_expr()?;
                self.expect(&Token::RBracket, "]")?;
                expr = Expr::Index(Box::new(expr), Box::new(index));
            } else {
                break;
            }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let span = self.current_span();

        if let Token::Integer(n) = self.current_token().clone() {
            self.advance();
            return Some(Expr::Literal(ast::Literal::Int(n)));
        }
        if let Token::Float(n) = self.current_token().clone() {
            self.advance();
            return Some(Expr::Literal(ast::Literal::Float(n)));
        }
        if let Token::String(s) = self.current_token().clone() {
            self.advance();
            return Some(Expr::Literal(ast::Literal::Str(s)));
        }
        if let Token::Char(c) = self.current_token().clone() {
            self.advance();
            return Some(Expr::Literal(ast::Literal::Char(c)));
        }
        if self.match_token(&Token::True) {
            return Some(Expr::Literal(ast::Literal::Bool(true)));
        }
        if self.match_token(&Token::False) {
            return Some(Expr::Literal(ast::Literal::Bool(false)));
        }
        if self.match_token(&Token::Underscore) {
            return Some(Expr::Literal(ast::Literal::Underscore));
        }
        if self.match_token(&Token::Ident("".into())) {
            let name = self.previous_ident()?;
            return Some(Expr::Var(name));
        }
        if self.match_token(&Token::LParen) {
            let expr = self.parse_expr()?;
            self.expect(&Token::RParen, ")")?;
            return Some(Expr::Group(Box::new(expr)));
        }
        if self.match_token(&Token::LBrace) {
            let block = self.parse_block_expr()?;
            return Some(Expr::Block(block));
        }
        if self.match_token(&Token::If) {
            return self.parse_if_expr();
        }
        if self.match_token(&Token::Match) {
            return self.parse_match_expr();
        }
        if self.match_token(&Token::While) {
            return self.parse_while_expr();
        }
        if self.match_token(&Token::For) {
            return self.parse_for_expr();
        }
        if self.match_token(&Token::Loop) {
            return self.parse_loop_expr();
        }
        if self.match_token(&Token::Break) {
            let value = if !self.check(&Token::Semi) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            self.expect(&Token::Semi, ";")?;
            return Some(Expr::Break(value.map(Box::new)));
        }
        if self.match_token(&Token::Continue) {
            self.expect(&Token::Semi, ";")?;
            return Some(Expr::Continue);
        }
        if self.match_token(&Token::Return) {
            let value = if !self.check(&Token::Semi) {
                Some(self.parse_expr()?)
            } else {
                None
            };
            self.expect(&Token::Semi, ";")?;
            return Some(Expr::Return(value.map(Box::new)));
        }
        if self.match_token(&Token::Async) {
            let block = self.parse_block_expr()?;
            return Some(Expr::AsyncBlock(block));
        }
        if self.match_token(&Token::Await) {
            let expr = self.parse_expr()?;
            return Some(Expr::Await(Box::new(expr)));
        }
        if self.match_token(&Token::Spawn) {
            let expr = self.parse_expr()?;
            return Some(Expr::Spawn(Box::new(expr)));
        }
        if self.match_token(&Token::Nursery) {
            self.expect(&Token::LBrace, "{")?;
            let mut stmts = Vec::new();
            while !self.check(&Token::RBrace) && !self.is_at_end() {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(&Token::RBrace, "}")?;
            return Some(Expr::Nursery(stmts));
        }
        if self.match_token(&Token::Unsafe) {
            self.expect(&Token::LBrace, "{")?;
            let mut stmts = Vec::new();
            while !self.check(&Token::RBrace) && !self.is_at_end() {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(&Token::RBrace, "}")?;
            return Some(Expr::UnsafeBlock(stmts));
        }
        if self.match_token(&Token::Comptime) {
            self.expect(&Token::LBrace, "{")?;
            let mut stmts = Vec::new();
            while !self.check(&Token::RBrace) && !self.is_at_end() {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(&Token::RBrace, "}")?;
            return Some(Expr::ComptimeBlock(stmts));
        }

        self.diagnostics.error("Expected expression", span);
        None
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        let cond = self.parse_expr()?;
        let then_branch = self.parse_block_expr()?;
        let else_branch = if self.match_token(&Token::Else) {
            Some(self.parse_block_expr()?)
        } else {
            None
        };
        Some(Expr::If(Box::new(cond), Box::new(then_branch), else_branch.map(Box::new)))
    }

    fn parse_match_expr(&mut self) -> Option<Expr> {
        self.expect(&Token::LBrace, "{")?;
        let mut arms = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            arms.push(self.parse_match_arm()?);
        }
        self.expect(&Token::RBrace, "}")?;
        Some(Expr::Match(Box::new(Expr::Var("todo".into())), arms))
    }

    fn parse_match_arm(&mut self) -> Option<ast::MatchArm> {
        let pattern = self.parse_pattern()?;
        let guard = if self.match_token(&Token::If) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(&Token::FatArrow, "=>")?;
        let body = self.parse_expr()?;
        self.expect(&Token::Comma, ",")?;
        Some(ast::MatchArm { pattern, guard, body })
    }

    fn parse_while_expr(&mut self) -> Option<Expr> {
        let cond = self.parse_expr()?;
        self.expect(&Token::Do, "do")?;
        let body = self.parse_block_expr()?;
        self.expect(&Token::End, "end")?;
        Some(Expr::While(Box::new(cond), Box::new(body)))
    }

    fn parse_for_expr(&mut self) -> Option<Expr> {
        let pattern = self.parse_pattern()?;
        self.expect(&Token::In, "in")?;
        let iter = self.parse_expr()?;
        self.expect(&Token::Do, "do")?;
        let body = self.parse_block_expr()?;
        self.expect(&Token::End, "end")?;
        Some(Expr::For(pattern, Box::new(iter), Box::new(body)))
    }

    fn parse_loop_expr(&mut self) -> Option<Expr> {
        let body = self.parse_block_expr()?;
        self.expect(&Token::End, "end")?;
        Some(Expr::Loop(Box::new(body)))
    }

    fn parse_pattern(&mut self) -> Option<Pattern> {
        if self.check(&Token::Underscore) {
            self.advance();
            return Some(Pattern::Wildcard);
        }
        if self.check(&Token::Ident("".into())) {
            let ident = self.parse_ident()?;
            if self.check(&Token::At) {
                self.advance();
                let pattern = self.parse_pattern()?;
                return Some(Pattern::Binding { name: ident, pattern: Box::new(pattern) });
            }
            if self.check(&Token::LParen) {
                self.advance();
                let mut args = Vec::new();
                if !self.check(&Token::RParen) {
                    loop {
                        args.push(self.parse_pattern()?);
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&Token::RParen, ")")?;
                return Some(Pattern::Constructor { path: ident, args });
            }
            return Some(Pattern::Var(ident));
        }
        if let Token::Integer(n) = self.current_token().clone() {
            self.advance();
            return Some(Pattern::Literal(ast::Literal::Int(n)));
        }
        if self.match_token(&Token::LParen) {
            let mut patterns = Vec::new();
            if !self.check(&Token::RParen) {
                loop {
                    patterns.push(self.parse_pattern()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::RParen, ")")?;
            return Some(Pattern::Tuple(patterns));
        }
        if self.match_token(&Token::LBracket) {
            let mut patterns = Vec::new();
            if !self.check(&Token::RBracket) {
                loop {
                    patterns.push(self.parse_pattern()?);
                    if !self.match_token(&Token::Comma) {
                        break;
                    }
                }
            }
            self.expect(&Token::RBracket, "]")?;
            return Some(Pattern::Array(patterns));
        }
        self.diagnostics.error("Expected pattern", self.current_span());
        None
    }

    fn parse_block_expr(&mut self) -> Option<ast::BlockExpr> {
        self.expect(&Token::LBrace, "{")?;
        let mut stmts = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                self.advance();
            }
        }
        self.expect(&Token::RBrace, "}")?;
        Some(ast::BlockExpr { stmts })
    }

    fn parse_stmt(&mut self) -> Option<AstNode<Stmt>> {
        let start = self.current_span();
        let stmt = if self.check(&Token::Let) {
            self.advance();
            let mutable = self.match_token(&Token::Mut);
            let pattern = self.parse_pattern()?;
            let ty = if self.match_token(&Token::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(&Token::Assign, "=")?;
            let value = self.parse_expr()?;
            self.expect(&Token::Semi, ";")?;
            Stmt::Let { pattern, ty, value, mutable }
        } else if self.check(&Token::Semi) {
            self.advance();
            Stmt::Empty
        } else {
            let expr = self.parse_expr()?;
            self.expect(&Token::Semi, ";")?;
            Stmt::Expr(expr)
        };
        Some(AstNode::new(stmt, start.merge(self.previous_span())))
    }

    fn parse_ident(&mut self) -> Option<String> {
        if let Token::Ident(s) = self.current_token().clone() {
            self.advance();
            Some(s)
        } else {
            self.diagnostics.error("Expected identifier", self.current_span());
            None
        }
    }

    fn previous_ident(&mut self) -> Option<String> {
        if let Token::Ident(s) = self.previous_token().clone() {
            Some(s)
        } else {
            None
        }
    }

    fn parse_string_literal(&mut self) -> Option<String> {
        if let Token::String(s) = self.current_token().clone() {
            self.advance();
            Some(s)
        } else {
            None
        }
    }

    fn previous_token(&self) -> &Token {
        if self.pos > 0 {
            &self.tokens[self.pos - 1].token
        } else {
            &Token::Error
        }
    }

    fn previous_span(&self) -> Span {
        if self.pos > 0 {
            self.tokens[self.pos - 1].span
        } else {
            Span { start: 0, end: 0 }
        }
    }

    fn previous_token_was(&self, token: &Token) -> bool {
        std::mem::discriminant(self.previous_token()) == std::mem::discriminant(token)
    }

    fn sync_to_next_item(&mut self) {
        while !self.is_at_end() {
            if self.check(&Token::Fn)
                || self.check(&Token::Struct)
                || self.check(&Token::Enum)
                || self.check(&Token::Trait)
                || self.check(&Token::Impl)
                || self.check(&Token::Const)
                || self.check(&Token::Type)
                || self.check(&Token::Use)
                || self.check(&Token::Mod)
                || self.check(&Token::Extern)
                || self.check(&Token::Comptime)
            {
                break;
            }
            self.advance();
        }
    }
}