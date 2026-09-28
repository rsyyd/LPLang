# Metaprogramming

## Comptime
Code that runs at compile time:

```lp
comptime {
    const TABLE: [u8; 256] = generate_lookup_table();
}
```

## Proc Macros
Custom derive macros for code generation.