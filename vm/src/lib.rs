//! LPLang Bytecode VM

pub mod bytecode;
pub mod gc;
pub mod interpreter;
pub mod runtime;

use anyhow::Result;
use std::path::Path;

/// VM configuration.
#[derive(Debug, Clone)]
pub struct VmConfig {
    pub stack_size: usize,
    pub heap_size: usize,
    pub gc_threshold: usize,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self {
            stack_size: 1024 * 1024,      // 1MB
            heap_size: 64 * 1024 * 1024,  // 64MB
            gc_threshold: 1024 * 1024,    // 1MB
        }
    }
}

/// Virtual machine instance.
pub struct Vm {
    config: VmConfig,
    // TODO: registers, stack, heap, gc
}

impl Vm {
    pub fn new(config: VmConfig) -> Self {
        Self { config }
    }

    /// Execute bytecode from file.
    pub fn run_file(&mut self, path: &Path) -> Result<i32> {
        let bytecode = std::fs::read(path)?;
        self.run_bytecode(&bytecode)
    }

    /// Execute bytecode from memory.
    pub fn run_bytecode(&mut self, bytecode: &[u8]) -> Result<i32> {
        // TODO: implement interpreter
        unimplemented!("VM not yet implemented")
    }

    /// Start REPL.
    pub fn repl(&mut self) -> Result<()> {
        unimplemented!("REPL not yet implemented")
    }
}