// 11_generic_struct.lp — generic structs with inferred instantiation

struct Point<T> {
    x: T,
    y: T,
}

fn make_point<T>(x: T, y: T) -> Point {
    return Point { x: x, y: y };
}

let p1 = make_point(42, 99);
println("Point 1 (int):", p1);
assert(p1.x == 42 && p1.y == 99, "int point");

let p2 = make_point("left", "right");
println("Point 2 (string):", p2);
assert(p2.x == "left" && p2.y == "right", "string point");

println("Generic struct tests passed!");
