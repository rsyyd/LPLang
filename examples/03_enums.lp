// Enums and Pattern Matching in LPLang

enum Option {
    Some(int),
    None,
}

enum Shape {
    Circle(int),
    Rectangle(int, int),
    Point,
}

fn area(s: Shape) -> int {
    match s {
        Circle(r) => {
            return 3 * r * r
        }
        Rectangle(w, h) => {
            return w * h
        }
        Point => {
            return 0
        }
    }
    return 0
}

fn unwrap_or(opt: Option, fallback: int) -> int {
    match opt {
        Some(v) => { return v }
        None => { return fallback }
    }
    return fallback
}

let c = Shape::Circle(5)
let r = Shape::Rectangle(4, 6)
let p = Shape::Point

let area_c = area(c)
let area_r = area(r)
let area_p = area(p)

println("Circle area (approx):", area_c)
println("Rectangle area:", area_r)
println("Point area:", area_p)

assert(area_c == 75, "circle area")
assert(area_r == 24, "rectangle area")
assert(area_p == 0, "point area")

let some_val = Option::Some(100)
let none_val = Option::None

assert(unwrap_or(some_val, -1) == 100, "unwrap some")
assert(unwrap_or(none_val, -1) == -1, "unwrap none")

println("All enum and pattern matching tests passed!")