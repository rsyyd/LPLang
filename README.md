# Lumpo Language (LPLang)

General-purpose programming language. Source extension: `.lp`.

[![Version](https://img.shields.io/badge/version-0.1.0--alpha.15-blue.svg)](CHANGELOG.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-42%20passing-brightgreen.svg)](tests/)

## Status

| Component | State | Notes |
|---|---|---|
| Lexer | IMPLEMENTED | Stage 0 Python bootstrap |
| Parser | IMPLEMENTED | Pratt precedence parsing |
| AST | IMPLEMENTED | Strongly typed nodes |
| Type checker | IMPLEMENTED | Static primitives, structs, enums, lists, tuples, inference |
| Interpreter | IMPLEMENTED | Tree-walk runtime for Stage 0 |
| Native Codegen | EXPERIMENTAL | C backend emission |
| Concurrency | EXPERIMENTAL | async/await/spawn via threads |
| Error handling | IMPLEMENTED | Result, Option, `?` operator |
| Generics | IMPLEMENTED (Stage 0) | Generic functions & structs with call-site inference |
| Control flow | IMPLEMENTED | if/else, while, for-in with range (`..`) |
| Collections | IMPLEMENTED | List, Tuple literals & indexing |
| Match / Patterns | IMPLEMENTED | Statement, expression, exhaustiveness checking |
| Tooling (`lp`) | IMPLEMENTED | `lp run`, `lp build`, `lp check`, `lp test` |
| Standard library | IMPLEMENTED | `result.lp`, `math_utils.lp` |
| Self-hosting | PLANNED | Target Stage 5 bootstrap |

Nothing here is "better than X" until benchmarks exist. Claims are backed by reproducible tests in `tests/`.

## Quick Start

```bash
# Run an example
./tools/lp run examples/01_basics.lp

# Type check a file
./tools/lp check examples/02_structs.lp

# Run all unit tests
python3 -m unittest discover -s tests
```

## Repository Layout

```
LPLang/
├── compiler/     # Bootstrap compiler (Python)
├── runtime/      # Target runtime
├── std/          # Standard library (.lp sources)
├── tools/        # CLI toolchain (lp runner)
├── tests/        # Unit & integration tests
├── examples/     # Verified .lp runnable programs
├── docs/         # Documentation
├── spec/         # Formal language specification
├── LICENSE       # MIT License
├── CHANGELOG.md  # Version release history
└── README.md
```

## Bootstrap Plan

- **Stage 0:** Python-stdlib compiler (current).
- **Stage 1–5:** Self-hosted compiler written in LPLang.

*No self-hosting claim is made until Stage 5 passes reproducibly.*

## License

MIT — see `LICENSE` file.