use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lp-version", about = "LPLang version tooling")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Check,
    Suggest,
    Changelog { since: Option<String> },
    Tier,
}