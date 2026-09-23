# Changelog

All notable changes to LPLang will be documented in this file.

## [0.1.0-alpha.15] - 2026-09-23

### Added
- Generic functions with call-site type inference (`fn name<T>(...)`)
- Generic struct declarations and inferred instantiation (`struct Box<T>`)
- For-in loop and inclusive range operator (`for x in 0..5 { ... }`)
- Tuple literals, indexing, and function returns (`(a, b)`, `t[0]`)

### Changed
- Struct type registration now tracks `type_params`
- TypeChecker `_LIT_TYPES` extended with `TupleLit` and `ListLit`
- IndexAccess now supports tuple, list, and string targets
- Interpreter `TupleLit` evaluates to Python tuple

### Fixed
- Fixed `unwrap_or_404` returning `None` bug (debugging + correct `match` payload extraction)
- Fixed `safe_compute(0)` zero-division case
- Fixed `match` exhaustiveness checking edge cases
- Fixed parser tuple literal grammar without commas (parenthesized expression)

### Deprecated
- None

### Removed
- None

## [0.1.0-alpha.14] - 2026-09-23

### Added
- For-in loop (`for VAR in EXPR { BODY }`)
- Inclusive range operator (`..` in `0..5`)

### Fixed
- None

## [0.1.0-alpha.13] - 2026-09-23

### Added
- Generic functions (`fn identity<T>(x: T) -> T`)

### Fixed
- None

## [0.1.0-alpha.12] - 2026-09-23

### Added
- Exhaustiveness checking for enum `match` statements

### Fixed
- None

## [0.1.0-alpha.11] - 2026-09-23

### Added
- `?` operator for Result/Option error propagation

### Fixed
- Fixed math error in example `09_try_operator.lp` (12 → 4)
- Fixed `unwrap_or_404` returning `None` bug

## [0.1.0-alpha.10] - 2026-09-23

### Added
- Match expressions (`match expr { ... }`)

### Fixed
- None

## [0.1.0-alpha.9] - 2026-09-23

### Added
- Native C backend (experimental)

### Fixed
- None

## [0.1.0-alpha.8] - 2026-09-23

### Added
- Collections (List literals, indexing, mutation, `len`/`append`)

### Fixed
- None

## [0.1.0-alpha.7] - 2026-09-23

### Added
- Modules/import system with stdlib path resolution

### Fixed
- None

## [0.1.0-alpha.6] - 2026-09-23

### Added
- Result/Option error handling stdlib

### Fixed
- None

## [0.1.0-alpha.5] - 2026-09-23

### Added
- Enums + pattern matching

### Fixed
- None

## [0.1.0-alpha.4] - 2026-09-23

### Added
- Struct declarations, literals, field access

### Fixed
- None

## [0.1.0-alpha.3] - 2026-09-23

### Added
- Static primitive type checker

### Fixed
- None

## [0.1.0-alpha.2] - 2026-09-23

### Added
- Parser and lexer stage 0 bootstrap

### Fixed
- None

## [0.1.0-alpha.1] - 2026-09-23

### Added
- Initial repository foundation

