pub mod bytecode {
    pub fn generate(_hir: &crate::hir::HirPackage) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub mod wasm {
    pub fn generate(_hir: &crate::hir::HirPackage) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

pub mod js {
    pub fn generate(_hir: &crate::hir::HirPackage) -> anyhow::Result<String> {
        Ok(String::new())
    }
}

pub mod cranelift {
    pub fn generate(_hir: &crate::hir::HirPackage) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}