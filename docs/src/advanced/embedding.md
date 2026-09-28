# Embedding LPLang

```rust
use lplang_compiler::Compiler;

let compiler = Compiler::new();
let hir = compiler.check("main.lp")?;
```