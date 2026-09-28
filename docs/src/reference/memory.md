# Memory Model

LPLang uses **ARC (Atomic Reference Counting)** with a **cycle detector**.

- No GC pauses — deterministic drop
- No borrow checker — familiar for Python/JS devs
- Cycle detector runs incrementally (epoch-based)
- Escape hatch: `unsafe { ... }` for FFI