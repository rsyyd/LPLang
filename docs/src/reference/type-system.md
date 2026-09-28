# Type System

## Gradual Typing
Types are optional. The compiler infers types when omitted.

```lp
let x = 42        # inferred as i64
let y: i32 = 10   # explicit
```

## Traits
```lp
trait Display {
    fn fmt(&self) -> String;
}
```

## Generics
```lp
fn first<T>(vec: Vec<T>) -> Option<T> {
    vec.get(0).cloned()
}
```