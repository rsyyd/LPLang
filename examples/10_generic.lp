// 10_generic.lp — generic functions

fn identity<T>(x: T) -> T {
    return x;
}

fn first<T>(a: T, b: T) -> T {
    return a;
}

fn max_int(a: int, b: int) -> int {
    if a > b {
        return a;
    }
    return b;
}

// Generic identity works with different types
let xi = identity(42);
let xs = identity("lumpo");
let xb = identity(true);

println("identity(42):", xi);
println("identity(lumpo):", xs);
println("identity(true):", xb);

assert(xi == 42, "identity int");
assert(xs == "lumpo", "identity string");
assert(xb == true, "identity bool");

// first<T> picks first arg
let fi = first(10, 20);
let fs = first("hello", "world");
assert(fi == 10, "first int");
assert(fs == "hello", "first string");

println("Generic function tests passed!");
