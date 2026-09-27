# LPLang — Product Requirements Document

## 1. The Pitch

LPLang is a language for people who are tired of choosing between "quick script" and "production ready." You write it like Python, it runs like Go, and the compiler catches your bugs like Rust.

No GC pauses. No 500-line boilerplate. No "install 47 packages to hello world." Just code that works.

## 2. Why Another Language? (Honest Answer)

Because every existing option annoyed me in a different way:

- **Python**: too slow, GIL, dependency hell, no types until 3.10+
- **Go**: no generics (until recently), error handling is verbose, no sum types, runtime is heavy for small tools
- **Rust**: steep curve, compile times, borrow checker fights you on day 1, overkill for a 200-line script
- **Node**: npm audit fatigue, async coloring, no stdlib, type system is a lie
- **Zig**: still changing, no package manager, comptime is great but the rest is rough

I want: Python syntax + Rust types + Go runtime + Zig comptime. Doesn't exist. Building it.

## 3. Design Principles (with Rejected Alternatives)

1. **Familiar > Novel**  
   Syntax stolen from Python/Rust/Go. If you know any of them, you can read LPLang in 15 minutes.  
   *Rejected: Custom syntax "for expressiveness" — nobody learns it.*

2. **Gradual Typing Default**  
   `let x = 1` works. `let x: i32 = 1` works. No ceremony until you want it.  
   *Rejected: Mandatory types everywhere (Rust) or none (Python).*

3. **ARC + Cycle Detector, Not GC, Not Ownership**  
   Deterministic drop, no pause, no borrow checker.  
   *Rejected: GC (unpredictable latency), Full ownership (steep curve, fights scripting).*

4. **Structured Concurrency or Bust**  
   Nurseries + channels. No `go func()` fire-and-forget.  
   *Rejected: Goroutines (leaks), raw threads (footguns).*

5. **Comptime = Metaprogramming**  
   Zig proved this. Proc macros are a backup.  
   *Rejected: Template metaprogramming (C++), reflection (slow).*

6. **Stdlib = Batteries Included, But Swappable**  
   HTTP, JSON, crypto in the box. But you can `use my::http` and drop ours.  
   *Rejected: Minimal stdlib (Go pre-1.21) — friction for scripts.*

7. **Tooling Is Part of the Language**  
   Formatter, LSP, test runner, package manager — all in repo, all `lp <command>`. No "community will build it."

## 4. Language Sketch (Code First)

```lp
# This is valid LPLang. If it looks like Python with types, that's the point.

use std::net::http::{Server, Request, Response}
use std::collections::HashMap

# Gradual typing — infer or declare
let config = Config::from_env()?  # Result<T, E>, no exceptions

# Structs + traits (like Rust, simpler syntax)
struct User {
    id: u64,
    name: String,
    email: String,
}

trait Display {
    fn fmt(&self) -> String
}

impl Display for User {
    fn fmt(&self) -> String {
        format!("{} <{}>", self.name, self.email)
    }
}

# Async with nurseries (structured concurrency)
async fn handle_request(req: Request, db: &Db) -> Response {
    let user = db.users().find(req.param("id")?).await?;
    Response::json(user)
}

async fn main() -> Result<(), Error> {
    let db = Db::connect(&config.database_url).await?;
    
    # Nursery: all tasks join or all cancel
    nursery! {
        spawn Server::bind("0.0.0.0:8080")?.serve(|req| handle_request(req, &db));
        spawn metrics_server(&config);
        spawn graceful_shutdown();
    }
    
    Ok(())
}

# Comptime: runs at compile time, generates code
comptime {
    const ROUTES: &[(&str, fn(Request) -> Response)] = &[
        ("/users", handle_request),
        ("/health", |_| Response::ok("ok")),
    ];
    generate_router!(ROUTES);  # proc macro, zero runtime cost
}
```

## 5. Type System (What We Stole)

Core: Hindley-Milner + Traits (Rust) + Type Classes (Haskell) — the complex parts stripped.

- Let-polymorphism: `let id = |x| x` works, `id(1); id("hi")` works
- Traits: `trait Add { fn add(self, other: Self) -> Self }`
- Generics: Monomorphized at MIR (like Rust), not type-erased
- Variance: Inferred, not declared (unlike Kotlin/Scala)
- No higher-kinded types v1 — `Functor` can wait
- No GATs v1 — generic associated types are pain

*Rejected: Dependent types (Idris), Effect systems (Koka), Row polymorphism (PureScript) — too much for v1.*

## 6. Memory Model (The One Big Decision)

**ARC + Cycle Detector (Swift-style)**

Why not GC?
- 50ms pause on a 10GB heap kills CLI tools
- Can't embed in Rust/Go/C++ cleanly
- "Stop the world" is a bug in 2024

Why not Ownership (Rust)?
- Borrow checker is a 6-month learning curve
- Scripts become ceremony: `&mut`, `clone()`, `Rc<RefCell<>>`
- FFI is painful — who owns what across the boundary?

Why ARC + Cycles?
- Deterministic drop (RAII works)
- No pause (cycle detector runs incrementally, epoch-based)
- Familiar: Python/JS devs understand refcounting
- Embedding: C API is `retain`/`release` — trivial
- Cost: 8 bytes/pointer, ~5% overhead vs raw — acceptable

Escape hatch: `unsafe { ... }` for FFI and the 1% where you need manual control. Audited, not encouraged.

## 7. Concurrency (What We're Copying)

- **async/await** — syntax from Rust/JS, semantics from Kotlin
- **Nurseries** — structured concurrency from Trio (Python) / Kotlin coroutines
- **Channels** — MPSC, oneshot, broadcast from Go / Rust crossbeam
- **Select** — `select!` macro for multi-channel wait
- **No goroutines** — every async task has a parent nursery

## 8. Tooling (What Ships v1, What Waits)

| Tool | v1 | Later |
|------|-----|-------|
| `lp check` | ✅ Typecheck only | — |
| `lp run` | ✅ Bytecode VM | — |
| `lp build` | ✅ WASM, JS | Native AOT (Cranelift) |
| `lp test` | ✅ Built-in framework | Property-based, snapshots |
| `lp fmt` | ✅ Zero-config | — |
| `lp doc` | ✅ HTML + markdown | — |
| `lp lsp` | ✅ Diagnostics, completion, goto-def | Rename, refactor |
| `lp add/publish` | ✅ LPM with GitHub Packages | Private registries, workspaces |
| `lp repl` | ✅ REPL | — |
| Debugger (DAP) | ❌ | Via LSP |
| Self-hosting compiler | ❌ | Post-1.0 |

## 9. Versioning (No Hallucination Policy)

Version numbers are communication, not marketing.

**Scheme:** `MAJOR.MINOR.PATCH[-TIER.BUILD]`

- **MAJOR** = `lp api diff` shows breaking changes
- **MINOR** = New feature, no breaks
- **PATCH** = Fixes only

**TIER is COMPUTED, not chosen:**
- `alpha`: CI green, compiles
- `beta`: 80% coverage, 2 weeks soak
- `rc`: 90% coverage, 4 weeks soak, benchmarks ±5%
- `stable`: 95% coverage, 8 weeks soak, zero known bugs

No human overrides data gates. Ever.

**CLI tells the story:**
```bash
$ lp --version
lp 0.3.2-beta.14 (2024-01-15)
  → 12 commits since 0.3.1
  → 3 features, 7 fixes, 2 docs
  → Coverage: 87% (need 90% for rc)
  → Perf: +2% http, -1% json
  → Next milestone: rc in ~3 weeks if CI stays green
```

Changelog is generated from conventional commits, verified in CI. No "fixed stuff" entries.

**Prohibited:**
- Bumping version "because it feels like time"
- Skipping tiers (alpha → stable)
- Major version without `lp api diff` evidence
- Changelog written after the fact
- Version in `Cargo.toml` not matching git tag

## 10. Roadmap (Phases with Real Dates)

| Phase | Focus | Target | Gate |
|-------|-------|--------|------|
| **0** | Foundation: PRD, repo, CI, workspace | Week 1 | `cargo check` passes, CI green |
| **1** | Frontend: Lexer → Parser → AST → HIR → Typechecker | Weeks 2-4 | 10 examples typecheck cleanly |
| **2** | Runtime: Bytecode VM + Stdlib core + REPL | Weeks 5-8 | `lp run http_server.lp` works |
| **3** | Ecosystem: LPM + WASM/JS backends | Weeks 9-12 | Publish → add → build roundtrip |
| **4** | Tooling: LSP + Formatter + Test + Doc + AOT | Weeks 13-16 | VS Code extension functional |
| **5** | Release: Stdlib complete + 0.1.0 | Weeks 17-20 | 5+ third-party packages published |

*Solo project, AI-driven. Dates are estimates, not commitments.*

## 11. Non-Goals (Explicit)

Not doing in v1 (maybe ever):
- IDE built-in (use VS Code + LSP)
- GUI framework (use web tech or Tauri)
- Mobile targets (WASM covers web, native later)
- Kernel/embedded (no `no_std` yet)
- IDE debugger (DAP via LSP later)
- Incremental compilation (Cranelift has it, later)
- Self-hosting compiler (Rust until 1.0)
- Windows ARM (CI cost, PRs welcome)

## 12. Open Questions (Still Arguing)

- [ ] Syntax for async closures: `async ||` vs `|| async` vs `async move ||`
- [ ] Default integer: `i64` (Python) vs `i32` (Rust) vs platform `isize`
- [ ] Error union: `Result<T, E>` only vs `T | E` (union types)
- [ ] Package namespaces: `@user/pkg` (npm) vs `user/pkg` (Go) vs flat
- [ ] Edition system: Rust-style `edition 2024` vs semver only

---

## Appendix: Glossary (Only Terms We Actually Use)

| Term | Meaning |
|------|---------|
| **HIR** | High-level IR — typed AST after name resolution |
| **MIR** | Mid-level IR — optimized, monomorphized, ready for codegen |
| **Comptime** | Compile-time execution (Zig-style) |
| **Nursery** | Structured concurrency scope — all children join or cancel |
| **Tier** | Release maturity: alpha/beta/rc/stable (computed, not chosen) |
| **LPM** | LPLang Package Manager |
| **ARC** | Atomic Reference Counting |
| **Cycle Detector** | Epoch-based cycle collection for ARC |