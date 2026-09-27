# Contributing to LPLang

## Reality Check

This is a **solo project** driven by one person (and an AI assistant).
There's no team, no company, no funding. PRs are welcome but don't expect
instant review.

## Ways to Contribute

1. **File issues** — Bugs, feature requests, RFCs
2. **Test nightlies** — Run `lp` on your code, report breakage
3. **Write examples** — Real-world code in `examples/`
4. **Fix typos** — Docs, error messages, comments
5. **Port libraries** — When LPM exists, publish packages

## RFC Process

For significant changes (syntax, semantics, stdlib APIs):

1. Open an issue with the RFC template
2. Discussion period: 2 weeks minimum
3. Decision: Accept / Reject / Defer (by BDFL)
4. Implementation tracked in linked issue

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` — fix all warnings
- Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- No `unwrap()`/`expect()` in production code — use `?` and proper errors

## Testing

```bash
# Unit + integration tests
cargo test --workspace

# Specific crate
cargo test -p lplang-compiler

# With output
cargo test -- --nocapture
```

## Questions?

Open a GitHub Discussion — no Slack, no Discord, no mailing list.