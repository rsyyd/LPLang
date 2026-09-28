use crate::ast::Visibility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Target {
    #[default]
    Bytecode,
    Wasm,
    Js,
    Native,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub bytes: Vec<u8>,
    pub target: Target,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirPackage {
    pub modules: Vec<HirModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirModule {
    pub name: String,
    pub items: Vec<HirItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirItem {
    Fn(HirFunction),
    Struct(HirStruct),
    Trait(HirTrait),
    Impl(HirImpl),
    Const(HirConst),
    Use(HirUse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirFunction {
    pub name: String,
    pub params: Vec<HirParam>,
    pub ret_ty: HirType,
    pub body: HirExpr,
    pub is_async: bool,
    pub is_comptime: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirParam {
    pub name: String,
    pub ty: HirType,
    pub mutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirStruct {
    pub name: String,
    pub fields: Vec<HirField>,
    pub generics: Vec<HirGenericParam>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirField {
    pub name: String,
    pub ty: HirType,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirTrait {
    pub name: String,
    pub generics: Vec<HirGenericParam>,
    pub items: Vec<HirTraitItem>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirTraitItem {
    Fn(HirFunction),
    TypeAlias { name: String, ty: HirType },
    Const { name: String, ty: HirType, value: HirExpr },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirImpl {
    pub generics: Vec<HirGenericParam>,
    pub trait_ref: Option<HirTraitRef>,
    pub for_ty: HirType,
    pub items: Vec<HirImplItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirImplItem {
    Fn(HirFunction),
    TypeAlias { name: String, ty: HirType },
    Const { name: String, ty: HirType, value: HirExpr },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirTraitRef {
    pub trait_name: String,
    pub args: Vec<HirType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirConst {
    pub name: String,
    pub ty: HirType,
    pub value: HirExpr,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirUse {
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirGenericParam {
    pub name: String,
    pub bounds: Vec<HirTraitBound>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirTraitBound {
    pub trait_ref: HirTraitRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirExpr {
    Literal(HirLiteral),
    Var(String),
    Call(Box<HirExpr>, Vec<HirExpr>),
    MethodCall(Box<HirExpr>, String, Vec<HirExpr>),
    Field(Box<HirExpr>, String),
    Index(Box<HirExpr>, Box<HirExpr>),
    Binary(HirBinOp, Box<HirExpr>, Box<HirExpr>),
    Unary(HirUnOp, Box<HirExpr>),
    Cast(Box<HirExpr>, HirType),
    If(Box<HirExpr>, Box<HirBlockExpr>, Option<Box<HirBlockExpr>>),
    Match(Box<HirExpr>, Vec<HirMatchArm>),
    While(Box<HirExpr>, Box<HirBlockExpr>),
    For(HirPattern, Box<HirExpr>, Box<HirBlockExpr>),
    Loop(Box<HirBlockExpr>),
    Break(Option<Box<HirExpr>>),
    Continue,
    Return(Option<Box<HirExpr>>),
    Block(HirBlockExpr),
    AsyncBlock(HirBlockExpr),
    Await(Box<HirExpr>),
    Spawn(Box<HirExpr>),
    Nursery(Vec<HirStmt>),
    UnsafeBlock(Vec<HirStmt>),
    ComptimeBlock(Vec<HirStmt>),
    StructLit { path: HirTypePath, fields: Vec<HirStructFieldExpr> },
    ArrayLit(Vec<HirExpr>),
    TupleLit(Vec<HirExpr>),
    Closure { params: Vec<HirParam>, body: Box<HirExpr>, is_async: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirStructFieldExpr {
    pub name: String,
    pub value: HirExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirMatchArm {
    pub pattern: HirPattern,
    pub guard: Option<HirExpr>,
    pub body: HirExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirPattern {
    Var(String),
    Literal(HirLiteral),
    Wildcard,
    Tuple(Vec<HirPattern>),
    Array(Vec<HirPattern>),
    Constructor { path: HirTypePath, args: Vec<HirPattern> },
    Binding { name: String, pattern: Box<HirPattern> },
    Rest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirBlockExpr {
    pub stmts: Vec<HirStmt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirStmt {
    Expr(HirExpr),
    Let { pattern: HirPattern, ty: Option<HirType>, value: HirExpr, mutable: bool },
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirLiteral {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Unit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HirType {
    Primitive(PrimitiveTy),
    Named(HirTypePath),
    Fn(Vec<HirType>, Box<HirType>),
    Tuple(Vec<HirType>),
    Array(Box<HirType>, Option<usize>),
    Reference(Box<HirType>, Mutability),
    Var(usize),
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirTypePath {
    pub segments: Vec<String>,
    pub args: Vec<HirType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveTy {
    Bool,
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
    F32, F64,
    Char,
    Str,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mutability {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HirBinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HirUnOp {
    Neg, Not, BitNot,
    Deref, Ref, RefMut,
}