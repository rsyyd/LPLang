# Lumpo Language (LPLang)

General-purpose programming language. Source extension: `.lp`.

## Status

| Component | State |
|---|---|
| Lexer | IMPLEMENTED (Python bootstrap) |
| Parser | PLANNED |
| Type checker | PLANNED |
| Codegen | PLANNED |
| Runtime | PLANNED |
| Standard library | PLANNED |
| Package/build tooling (`lp`) | PLANNED |

Nothing here is "better than X" until benchmarks exist. Claims are backed by tests in `tests/`.

## Repository layout

```
LPLang/
├── compiler/     # bootstrap compiler (Python) → later self-hosted
├── runtime/      # target runtime
├── std/          # standard library (.lp sources)
├── tools/        # lp CLI, formatter, linter, lsp
├── tests/        # unit / integration / conformance / negative
├── examples/     # runnable .lp programs
├── docs/         # prose documentation (English)
├── spec/         # normative language specification
├── benchmarks/   # perf harness (kept separate from correctness)
├── scripts/      # dev helpers
├── integrations/ # editor/CI bindings
└── playground/   # web/wasm demo
```

Empty directories are placeholders for planned phases; do not treat their existence as implementation status.

## Bootstrap plan

Stage 0: Python-stdlib compiler (current).
Stage 1–5: see `spec/bootstrap.md` (planned).

No self-hosting claim until Stage 5 passes reproducibly.

## Development

Run tests:

```
python3 -m unittest discover -s tests -v
```

## License

TBD — see LICENSE file once chosen. Until then, all rights reserved.