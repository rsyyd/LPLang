//! LPLang Compiler — Frontend + Codegen Backends
//!
//! Pipeline: Source → Lexer → Parser → AST → Resolver → HIR → Typecheck → MIR → Codegen

pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod driver;
pub mod hir;
pub mod lexer;
pub mod parser;
pub mod resolver;
pub mod typecheck;
pub mod versioning;

use anyhow::Result;
use std::path::Path;

/// High-level compilation driver.
pub struct Compiler {
    // TODO: configuration
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    /// Typecheck a source file, returning HIR or diagnostics.
    pub fn check(&self, path: &Path) -> Result<HirPackage> {
        let source = std::fs::read_to_string(path)?;
        let hir = self.compile_source(&source, path)?;
        Ok(hir)
    }

    /// Full compile to target.
    pub fn build(&self, path: &Path, target: Target) -> Result<Artifact> {
        let hir = self.check(path)?;
        self.codegen(hir, target)
    }

    fn compile_source(&self, source: &str, path: &Path) -> Result<HirPackage> {
        // TODO: implement pipeline
        unimplemented!("compiler pipeline not yet implemented")
    }

    fn codegen(&self, hir: HirPackage, target: Target) -> Result<Artifact> {
        unimplemented!("codegen not yet implemented")
    }
}

/// Compilation targets.
#[derive(Debug, Clone, Copy)]
pub enum Target {
    Bytecode,
    Wasm,
    Js,
    Native,
}

/// Result of successful compilation.
pub struct Artifact {
    pub bytes: Vec<u8>,
    pub target: Target,
}

/// Typechecked package (HIR).
pub struct HirPackage {
    pub modules: Vec<HirModule>,
}

/// Single module in HIR.
pub struct HirModule {
    pub name: String,
    pub items: Vec<HirItem>,
}

/// HIR item (function, struct, trait, etc.).
pub enum HirItem {
    Fn(HirFunction),
    Struct(HirStruct),
    Trait(HirTrait),
    Impl(HirImpl),
    Const(HirConst),
    Use(HirUse),
}

/// Typechecked function.
pub struct HirFunction {
    pub name: String,
    pub params: Vec<HirParam>,
    pub ret_ty: HirType,
    pub body: HirExpr,
    pub is_async: bool,
    pub is_comptime: bool,
}

/// HIR expression.
pub enum HirExpr {
    Literal(HirLiteral),
    Var(String),
    Call(Box<HirExpr>, Vec<HirExpr>),
    Binary(BinOp, Box<HirExpr>, Box<HirExpr>),
    Unary(UnOp, Box<HirExpr>),
    If(Box<HirExpr>, Box<HirExpr>, Option<Box<HirExpr>>),
    Match(Box<HirExpr>, Vec<HirArm>),
    Block(Vec<HirStmt>),
    Return(Option<Box<HirExpr>>),
    Await(Box<HirExpr>),
    Spawn(Box<HirExpr>),
    // ... more variants
}

pub struct HirArm {
    pub pattern: HirPattern,
    pub guard: Option<HirExpr>,
    pub body: HirExpr,
}

pub struct HirPattern {
    // TODO
}

pub struct HirStmt {
    // TODO
}

pub enum HirLiteral {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Unit,
}

pub enum HirType {
    Primitive(PrimitiveTy),
    Named(String, Vec<HirType>),
    Fn(Vec<HirType>, Box<HirType>),
    Tuple(Vec<HirType>),
    Array(Box<HirType>, Option<usize>),
    Reference(Box<HirType>, Mutability),
    // ...
}

pub enum PrimitiveTy {
    Bool,
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    F32, F64,
    Char,
    Str,
    Never,
}

pub enum Mutability {
    Immutable,
    Mutable,
}

pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

pub enum UnOp {
    Neg, Not, BitNot,
}

pub struct HirParam {
    pub name: String,
    pub ty: HirType,
    pub mutable: bool,
}

pub struct HirStruct {
    pub name: String,
    pub fields: Vec<HirField>,
    pub generics: Vec<HirGenericParam>,
}

pub struct HirField {
    pub name: String,
    pub ty: HirType,
}

pub struct HirTrait {
    pub name: String,
    pub generics: Vec<HirGenericParam>,
    pub items: Vec<HirTraitItem>,
}

pub enum HirTraitItem {
    Fn(HirFunction),
    Type(HirTypeAlias),
    Const(HirConst),
}

pub struct HirImpl {
    pub trait_ref: Option<HirTraitRef>,
    pub for_ty: HirType,
    pub items: Vec<HirImplItem>,
}

pub enum HirImplItem {
    Fn(HirFunction),
    Type(HirTypeAlias),
    Const(HirConst),
}

pub struct HirTraitRef {
    pub path: String,
    pub args: Vec<HirType>,
}

pub struct HirTypeAlias {
    pub name: String,
    pub ty: HirType,
}

pub struct HirConst {
    pub name: String,
    pub ty: HirType,
    pub value: HirExpr,
}

pub struct HirUse {
    pub path: String,
    pub alias: Option<String>,
}

pub struct HirGenericParam {
    pub name: String,
    pub bounds: Vec<HirTraitBound>,
}

pub struct HirTraitBound {
    pub trait_ref: HirTraitRef,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}