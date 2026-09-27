use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lpm", about = "LPLang Package Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init { name: Option<String> },
    Add { spec: String },
    Remove { name: String },
    Install,
    Update,
    Publish,
    Lockfile { #[command(subcommand)] cmd: LockfileCmd },
}

#[derive(Subcommand)]
pub enum LockfileCmd {
    Check,
    Generate,
}