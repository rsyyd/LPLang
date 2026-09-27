use anyhow::{Context, Result};
use std::path::Path;

use crate::{
    ast::AstNode,
    diagnostics::DiagnosticBag,
    hir::HirModule,
    lexer::Lexer,
    parser::Parser,
    resolver::Resolver,
    typecheck::TypeChecker,
    versioning::VersionInfo,
};

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    pub target: Target,
    pub emit_hir: bool,
    pub emit_mir: bool,
    pub check_only: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Target {
    #[default]
    Bytecode,
    Wasm,
    Js,
    Cranelift,
}

pub struct CompilerDriver {
    opts: CompileOptions,
    diagnostics: DiagnosticBag,
}

impl CompilerDriver {
    pub fn new(opts: CompileOptions) -> Self {
        Self {
            opts,
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub fn compile_file(&mut self, path: &Path) -> Result<()> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        self.compile_source(&source, &path.display().to_string())
    }

    pub fn compile_source(&mut self, source: &str, filename: &str) -> Result<()> {
        let mut lexer = Lexer::new(source, filename);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens, filename);
        let ast = parser.parse();

        if self.diagnostics.has_errors() {
            self.diagnostics.emit();
            return Err(anyhow::anyhow!("Parse errors"));
        }

        let mut resolver = Resolver::new();
        let resolved = resolver.resolve(ast)?;

        if self.diagnostics.has_errors() {
            self.diagnostics.emit();
            return Err(anyhow::anyhow!("Resolution errors"));
        }

        let mut typechecker = TypeChecker::new();
        let hir = typechecker.check(resolved)?;

        if self.diagnostics.has_errors() {
            self.diagnostics.emit();
            return Err(anyhow::anyhow!("Type errors"));
        }

        if self.opts.emit_hir {
            println!("{:#?}", hir);
        }

        if self.opts.check_only {
            println!("OK: {} ({} items)", filename, hir.items.len());
            return Ok(());
        }

        match self.opts.target {
            Target::Bytecode => {
                let bytecode = crate::codegen::bytecode::generate(&hir)?;
                println!("Bytecode: {} bytes", bytecode.len());
            }
            Target::Wasm => {
                let wasm = crate::codegen::wasm::generate(&hir)?;
                println!("WASM: {} bytes", wasm.len());
            }
            Target::Js => {
                let js = crate::codegen::js::generate(&hir)?;
                println!("JS: {} chars", js.len());
            }
            Target::Cranelift => {
                let obj = crate::codegen::cranelift::generate(&hir)?;
                println!("Native object: {} bytes", obj.len());
            }
        }

        Ok(())
    }

    pub fn version_info() -> VersionInfo {
        VersionInfo::from_cargo()
    }
}