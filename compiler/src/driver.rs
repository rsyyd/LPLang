use anyhow::{Context, Result};
use std::path::Path;

use crate::{
    diagnostics::DiagnosticBag,
    hir::Target,
    lexer::Lexer,
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

        // TODO: parser
        // let mut parser = Parser::new(tokens, filename);
        // let ast = parser.parse();

        // if self.diagnostics.has_errors() {
        //     self.diagnostics.emit();
        //     return Err(anyhow::anyhow!("Parse errors"));
        // }

        // let mut resolver = Resolver::new();
        // let resolved = resolver.resolve(ast)?;

        // if self.diagnostics.has_errors() {
        //     self.diagnostics.emit();
        //     return Err(anyhow::anyhow!("Resolution errors"));
        // }

        // let mut typechecker = TypeChecker::new();
        // let hir = typechecker.check(resolved)?;

        // if self.diagnostics.has_errors() {
        //     self.diagnostics.emit();
        //     return Err(anyhow::anyhow!("Type errors"));
        // }

        if self.opts.emit_hir {
            println!("HIR not yet implemented");
        }

        if self.opts.check_only {
            println!("OK: {} (lexed {} tokens)", filename, tokens.len());
            return Ok(());
        }

        match self.opts.target {
            Target::Bytecode => {
                println!("Bytecode: not yet implemented");
            }
            Target::Wasm => {
                println!("WASM: not yet implemented");
            }
            Target::Js => {
                println!("JS: not yet implemented");
            }
            Target::Native => {
                println!("Native object: not yet implemented");
            }
        }

        Ok(())
    }

    pub fn version_info() -> VersionInfo {
        VersionInfo::from_cargo()
    }
}