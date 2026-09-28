//! LPLang Compiler — Frontend + Codegen Backends
//!
//! Pipeline: Source → Lexer → Parser → AST → Resolver → HIR → Typecheck → MIR → Codegen

pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod driver;
pub mod hir;
pub mod lexer;
// pub mod parser;  // disabled until parser is fixed
pub mod resolver;
pub mod typecheck;
pub mod versioning;

use anyhow::Result;
use std::path::Path;

pub use driver::{CompileOptions, CompilerDriver};
pub use hir::{HirItem, HirModule, HirPackage, Target, Artifact};

pub fn compile_file(path: &Path, opts: CompileOptions) -> Result<()> {
    let mut driver = CompilerDriver::new(opts);
    driver.compile_file(path)
}

pub fn compile_source(source: &str, filename: &str, opts: CompileOptions) -> Result<()> {
    let mut driver = CompilerDriver::new(opts);
    driver.compile_source(source, filename)
}