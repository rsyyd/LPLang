use clap::{Parser, Subcommand};
use lplang_compiler::{CompileOptions, CompilerDriver};
use lplang_fmt; // workspace member
use lplang_compiler::hir::Target;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lp", version, about = "LPLang compiler and toolchain")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Typecheck a file
    Check {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Run a file
    Run {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Build a project
    Build {
        #[arg(short, long, value_name = "TARGET")]
        target: Option<String>,
        #[arg(short, long)]
        release: bool,
    },
    /// Format code
    Fmt {
        #[arg(value_name = "PATH")]
        path: Option<PathBuf>,
        #[arg(long)]
        check: bool,
    },
    /// Run tests
    Test {
        #[arg(value_name = "FILTER")]
        filter: Option<String>,
    },
    /// Start REPL
    Repl,
    /// Generate documentation
    Doc,
    /// Package manager commands
    Pkg {
        #[command(subcommand)]
        cmd: PkgCmd,
    },
    /// LSP server
    Lsp {
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
    },
    /// Version info
    Version,
}

#[derive(Subcommand)]
enum PkgCmd {
    /// Initialize new package
    Init { name: Option<String> },
    /// Add dependency
    Add { spec: String },
    /// Remove dependency
    Remove { name: String },
    /// Install dependencies
    Install,
    /// Update dependencies
    Update,
    /// Publish package
    Publish,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { file } => {
            let mut driver = CompilerDriver::new(CompileOptions {
                target: Target::Bytecode,
                check_only: true,
                ..Default::default()
            });
            driver.compile_file(&file)?;
        }
        Commands::Run { file, args: _ } => {
            let mut driver = CompilerDriver::new(CompileOptions {
                target: Target::Bytecode,
                ..Default::default()
            });
            driver.compile_file(&file)?;
            // TODO: actually run via VM
        }
        Commands::Build { target, release: _ } => {
            let target = target.map(|t| match t.as_str() {
                "wasm" => Target::Wasm,
                "js" => Target::Js,
                "native" => Target::Native,
                _ => Target::Bytecode,
            }).unwrap_or(Target::Bytecode);

            let mut driver = CompilerDriver::new(CompileOptions {
                target,
                ..Default::default()
            });
            driver.compile_file(&PathBuf::from("."))?;
        }
        Commands::Fmt { path, check } => {
            let path = path.unwrap_or_else(|| PathBuf::from("."));
            lplang_fmt::format_path(&path, check)?;
        }
        Commands::Test { filter: _ } => {
            // TODO: run tests
            println!("Tests not yet implemented");
        }
        Commands::Repl => {
            println!("REPL not yet implemented");
        }
        Commands::Doc => {
            println!("Doc generation not yet implemented");
        }
        Commands::Pkg { cmd } => {
            match cmd {
                PkgCmd::Init { name: _ } => println!("lpm init not implemented"),
                PkgCmd::Add { spec: _ } => println!("lpm add not implemented"),
                PkgCmd::Remove { name: _ } => println!("lpm remove not implemented"),
                PkgCmd::Install => println!("lpm install not implemented"),
                PkgCmd::Update => println!("lpm update not implemented"),
                PkgCmd::Publish => println!("lpm publish not implemented"),
            }
        }
        Commands::Lsp { addr } => {
            println!("LSP server on {} not implemented", addr);
        }
        Commands::Version => {
            println!("{}", CompilerDriver::version_info().display());
        }
    }

    Ok(())
}