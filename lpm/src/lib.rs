//! LPLang Package Manager

pub mod cli;
pub mod fetch;
pub mod lockfile;
pub mod publish;
pub mod registry;
pub mod resolver;
pub mod workspace;

use anyhow::Result;
use std::path::Path;

/// LPM client.
pub struct Lpm {
    // TODO: config, registry, cache
}

impl Lpm {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self, path: &Path) -> Result<()> {
        unimplemented!("lpm init not implemented")
    }

    pub fn add(&self, spec: &str) -> Result<()> {
        unimplemented!("lpm add not implemented")
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        unimplemented!("lpm remove not implemented")
    }

    pub fn install(&self) -> Result<()> {
        unimplemented!("lpm install not implemented")
    }

    pub fn update(&self) -> Result<()> {
        unimplemented!("lpm update not implemented")
    }

    pub fn publish(&self) -> Result<()> {
        unimplemented!("lpm publish not implemented")
    }

    pub fn lockfile_check(&self) -> Result<()> {
        unimplemented!("lpm lockfile check not implemented")
    }
}

impl Default for Lpm {
    fn default() -> Self {
        Self::new()
    }
}