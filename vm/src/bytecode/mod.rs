#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    // Stack manipulation
    PushConst,
    Pop,
    Dup,
    Swap,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,
    Not,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Shl,
    Shr,

    // Control flow
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    Call,
    Return,
    TailCall,

    // Variables
    LoadLocal,
    StoreLocal,
    LoadGlobal,
    StoreGlobal,

    // Struct/Array
    MakeStruct,
    MakeArray,
    GetField,
    SetField,
    GetIndex,
    SetIndex,

    // Async
    Await,
    Spawn,
    Yield,

    // GC
    GcAlloc,
    GcRoot,
    GcUnroot,
}

pub struct Instruction {
    pub opcode: OpCode,
    pub operand: u32,
}

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub functions: Vec<FunctionInfo>,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub arity: u32,
    pub start_pc: u32,
    pub local_count: u32,
}