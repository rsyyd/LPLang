// math_utils.lp — imported by 04_modules.lp

fn square(x: int) -> int {
    return x * x;
}

fn cube(x: int) -> int {
    return x * x * x;
}

fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    }
    return b;
}

var MAGIC: int = 42;