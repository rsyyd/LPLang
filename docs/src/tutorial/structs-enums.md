# Structs & Enums

```lp
struct User {
    id: u64,
    name: String,
    email: String,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```