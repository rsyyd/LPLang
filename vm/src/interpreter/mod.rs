use crate::bytecode::{Bytecode, Instruction, OpCode};
use anyhow::Result;

pub struct Interpreter {
    bytecode: Bytecode,
    stack: Vec<Value>,
    locals: Vec<Value>,
    pc: usize,
    call_stack: Vec<CallFrame>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Struct(Vec<Value>),
    Array(Vec<Value>),
    Function(FunctionValue),
    Null,
}

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub func_idx: usize,
    pub captured: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub func_idx: usize,
    pub return_pc: usize,
    pub base_ptr: usize,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            stack: Vec::with_capacity(1024),
            locals: Vec::with_capacity(256),
            pc: 0,
            call_stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<Value> {
        loop {
            if self.pc >= self.bytecode.instructions.len() {
                break;
            }

            let instr = &self.bytecode.instructions[self.pc];
            self.pc += 1;

            match instr.opcode {
                OpCode::PushConst => {
                    let constant = self.bytecode.constants[instr.operand as usize].clone();
                    self.stack.push(self.constant_to_value(constant));
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Dup => {
                    let val = self.stack.last().cloned().unwrap();
                    self.stack.push(val);
                }
                OpCode::Add => self.binary_op(|a, b| Value::Int(a + b))?,
                OpCode::Sub => self.binary_op(|a, b| Value::Int(a - b))?,
                OpCode::Mul => self.binary_op(|a, b| Value::Int(a * b))?,
                OpCode::Div => self.binary_op(|a, b| Value::Int(a / b))?,
                OpCode::Return => {
                    let val = self.stack.pop().unwrap_or(Value::Null);
                    if let Some(frame) = self.call_stack.pop() {
                        self.pc = frame.return_pc;
                        self.locals.truncate(frame.base_ptr);
                        self.stack.push(val);
                    } else {
                        return Ok(val);
                    }
                }
                OpCode::Call => {
                    let func_idx = instr.operand as usize;
                    let func = self.stack.pop().unwrap();
                    let arity = self.bytecode.functions[func_idx].arity as usize;
                    let mut args = Vec::with_capacity(arity);
                    for _ in 0..arity {
                        args.push(self.stack.pop().unwrap());
                    }
                    args.reverse();

                    self.call_stack.push(CallFrame {
                        func_idx,
                        return_pc: self.pc,
                        base_ptr: self.locals.len(),
                    });

                    for arg in args {
                        self.locals.push(arg);
                    }
                    self.pc = self.bytecode.functions[func_idx].start_pc as usize;
                }
                _ => todo!("OpCode {:?} not implemented", instr.opcode),
            }
        }
        Ok(self.stack.pop().unwrap_or(Value::Null))
    }

    fn binary_op<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(i64, i64) -> Value,
    {
        let b = self.pop_int()?;
        let a = self.pop_int()?;
        self.stack.push(f(a, b));
        Ok(())
    }

    fn pop_int(&mut self) -> Result<i64> {
        match self.stack.pop() {
            Some(Value::Int(i)) => Ok(i),
            _ => anyhow::bail!("Expected integer"),
        }
    }

    fn constant_to_value(&self, c: crate::bytecode::Constant) -> Value {
        match c {
            crate::bytecode::Constant::Int(i) => Value::Int(i),
            crate::bytecode::Constant::Float(f) => Value::Float(f),
            crate::bytecode::Constant::String(s) => Value::String(s),
            crate::bytecode::Constant::Bytes(_) => Value::Null,
        }
    }
}