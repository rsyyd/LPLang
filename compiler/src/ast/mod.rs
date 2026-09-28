use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstNode<T> {
    pub node: T,
    pub span: Span,
}

impl<T> AstNode<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn merge(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Fn(FnItem),
    Struct(StructItem),
    Enum(EnumItem),
    Trait(TraitItem),
    Impl(ImplItem),
    Const(ConstItem),
    TypeAlias(TypeAliasItem),
    Use(UseItem),
    Mod(ModItem),
    ExternBlock(ExternBlock),
    ComptimeBlock(ComptimeBlock),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnItem {
    pub name: String,
    pub sig: FnSig,
    pub body: BlockExpr,
    pub is_async: bool,
    pub is_comptime: bool,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnSig {
    pub params: Vec<FnParam>,
    pub ret_ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnParam {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructItem {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<StructField>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumItem {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitItem {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub items: Vec<TraitItemKind>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitItemKind {
    Fn(FnSig),
    TypeAlias { name: String, ty: Option<Type> },
    Const { name: String, ty: Type, value: Expr },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImplItem {
    pub generics: Vec<GenericParam>,
    pub trait_ref: Option<TraitRef>,
    pub for_ty: Type,
    pub items: Vec<ImplItemKind>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImplItemKind {
    Fn(FnItem),
    TypeAlias { name: String, ty: Type },
    Const { name: String, ty: Type, value: Expr },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstItem {
    pub name: String,
    pub ty: Type,
    pub value: Expr,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAliasItem {
    pub name: String,
    pub ty: Type,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UseItem {
    Simple { name: String, alias: Option<String> },
    Glob { items: Vec<UseItem> },
    Path { prefix: String, rest: Box<UseItem> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModItem {
    pub name: String,
    pub items: Vec<Item>,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternBlock {
    pub abi: String,
    pub items: Vec<ExternItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternItem {
    Fn { name: String, sig: FnSig },
    Static { name: String, ty: Type },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComptimeBlock {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TraitBound>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitBound {
    pub trait_name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraitRef {
    pub trait_name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Path(TypePath),
    Group(Box<Type>),
    Array(Box<Type>, Option<Box<Expr>>),
    Ref(Box<Type>, Mutability),
    Fn(Vec<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Never,
    Infer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypePath {
    pub segments: Vec<String>,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mutability {
    Immutable,
    Mutable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Crate,
    Super,
    Self_,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Var(String),
    Call(Box<Expr>, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
    Field(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Cast(Box<Expr>, Type),
    If(Box<Expr>, Box<BlockExpr>, Option<Box<BlockExpr>>),
    Match(Box<Expr>, Vec<MatchArm>),
    While(Box<Expr>, Box<BlockExpr>),
    For(Pattern, Box<Expr>, Box<BlockExpr>),
    Loop(Box<BlockExpr>),
    Break(Option<Box<Expr>>),
    Continue,
    Return(Option<Box<Expr>>),
    Block(BlockExpr),
    AsyncBlock(BlockExpr),
    Await(Box<Expr>),
    Spawn(Box<Expr>),
    Nursery(Vec<Stmt>),
    UnsafeBlock(Vec<Stmt>),
    ComptimeBlock(Vec<Stmt>),
    StructLit { path: TypePath, fields: Vec<StructFieldExpr> },
    ArrayLit(Vec<Expr>),
    TupleLit(Vec<Expr>),
    Closure { params: Vec<FnParam>, body: Box<Expr>, is_async: bool },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructFieldExpr {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Var(String),
    Literal(Literal),
    Wildcard,
    Tuple(Vec<Pattern>),
    Array(Vec<Pattern>),
    Constructor { path: TypePath, args: Vec<Pattern> },
    Binding { name: String, pattern: Box<Pattern> },
    Rest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    Item(Item),
    Expr(Expr),
    Let { pattern: Pattern, ty: Option<Type>, value: Expr, mutable: bool },
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),
    Bool(bool),
    Underscore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
    Assign,
    AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    Neg, Not, BitNot,
    Deref, Ref, RefMut,
}