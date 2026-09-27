use anyhow::Result;

pub struct Runtime {
    // Builtin functions, syscalls, etc.
}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn register_builtins(&mut self) {
        // print, println, etc.
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}