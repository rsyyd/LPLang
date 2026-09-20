# Lumpo Language Specification (Draft v0.1)

Status: DRAFT
Scope: Stage 0 Subset

---

## 1. Lexical Structure

- Source files: UTF-8, extension `.lp`.
- Comments: `//` single-line comments extending to the end of the line. Block comments: TBD.
- Whitespace: spaces, tabs, carriage returns, and newlines separate tokens and are otherwise insignificant.
- Semicolons: optional at statement ends; consumed if present.

### 1.1 Identifiers & Keywords
Identifiers match `[a-zA-Z_][a-zA-Z0-9_]*`.
Reserved keywords:
`let`, `var`, `fn`, `return`, `if`, `else`, `while`, `for`, `in`, `break`, `continue`, `true`, `false`, `struct`, `enum`, `match`, `import`, `export`, `and`, `or`, `not`.

### 1.2 Literals
- **Integer**: sequence of decimal digits, optional single `_` between digits (e.g. `1_000_000`).
- **Float**: decimal digits, a single `.`, followed by decimal digits.
- **String**: double-quoted `"..."`. Escape sequences supported: `\n`, `\t`, `\r`, `\0`, `\\`, `\"`, `\'`.
- **Boolean**: `true`, `false`.

---

## 2. Types & Semantics

### 2.1 Primitive Types
- `int`: signed integers.
- `float`: IEEE 754 floating point numbers.
- `string`: UTF-8 encoded text sequences.
- `bool`: boolean value (`true` or `false`).

### 2.2 Struct Types
Structs are nominal, fixed-layout data types.
```
struct Name {
    field1: type1,
    field2: type2,
}
```
- Fields are typed and must be initialized in struct literals.
- Struct literals: `Name { field1: expr1, field2: expr2 }`.
- Field access: `expr.field`.
- Structs are compared by value (field-wise equality).

### 2.3 Bindings & Mutability
- `let name [: type] = expr;`: immutable binding. Reassignment is a compile-time (and runtime) error.
- `var name [: type] = expr;`: mutable binding. Reassignment allowed with matching type.
- If type annotation is omitted, the type is statically inferred from the initializer.

### 2.3 Functions
```
fn name(param1: type, param2: type) -> return_type {
    // statements
    return value;
}
```
- First-class values.
- Explicit return with `return <expr>;`.

---

## 3. Built-in Functions

- `println(...)`: print arguments space-separated with trailing newline.
- `print(...)`: print arguments space-separated without trailing newline.
- `assert(cond: bool, [msg: string])`: verify condition; abort with message if false.

---

## 4. Diagnostics Contract

- Every error must contain a 1-based line and column.
- Errors must explain: what failed, location, and an actionable hint when possible.
