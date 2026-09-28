# LPLang

> Familiar syntax, modern semantics. A pragmatic programming language for every scale.

[![CI](https://github.com/rsyyd/LPLang/workflows/CI/badge.svg)](https://github.com/rsyyd/LPLang/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/github/v/tag/rsyyd/LPLang?label=version)](https://github.com/rsyyd/LPLang/tags)

## Status: Pre-alpha — Active Development

LPLang is in early development. Expect breaking changes, missing features, and bugs.

## Quickstart

```bash
# Build from source
git clone https://github.com/rsyyd/LPLang.git
cd LPLang
cargo build --release
./target/release/lp run examples/hello.lp
```

## Example

```lp
# hello.lp
use std::net::http::{Server, Request, Response}

async fn handler(req: Request) -> Response {
    let name = req.query("name").unwrap_or("world");
    Response::ok(format!("Hello, {name}!"))
}

async fn main() -> Result<(), Error> {
    let server = Server::bind("127.0.0.1:8080").await?;
    server.serve(handler).await?;
    Ok(())
}
```

```bash
lp run hello.lp
# Server running on http://127.0.0.1:8080
```

## Features

| Feature | Status |
|---------|--------|
| Gradual typing (Python-like + Rust-like) | 🚧 In progress |
| ARC + cycle detector (no GC pauses) | 🚧 In progress |
| Structured concurrency (nurseries) | 🚧 In progress |
| Comptime metaprogramming (Zig-style) | 📋 Planned |
| WASM / JS backends | 📋 Planned |
| Native AOT (Cranelift) | 📋 Planned |
| Built-in package manager (`lpm`) | 🚧 In progress |
| LSP / VS Code support | 📋 Planned |
| Zero-config formatter | 📋 Planned |

## Design Principles

1. **Familiar > Novel** — Syntax from Python/Rust/Go
2. **Gradual typing default** — Types when you want them
3. **ARC + cycles, not GC/ownership** — Deterministic, no pauses
4. **Structured concurrency** — Nurseries, not goroutines
5. **Comptime = metaprogramming** — Zig proved this works
6. **Batteries included, swappable** — HTTP, JSON, crypto in stdlib
7. **Tooling is part of the language** — `lp fmt`, `lp test`, `lp lsp`, `lpm`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) — it's a solo project right now. Open an issue or discussion if you want to chat.

## License

MIT — see [LICENSE](LICENSE) for details.