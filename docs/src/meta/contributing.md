# Contributing

This is a solo project. PRs welcome but no instant review expected.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`