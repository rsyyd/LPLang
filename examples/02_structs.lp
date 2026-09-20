// Struct example: Point and Rectangle

struct Point {
    x: int,
    y: int,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn make_point(x: int, y: int) -> Point {
    return Point { x: x, y: y };
}

fn rect_area(r: Rectangle) -> int {
    let width = r.bottom_right.x - r.top_left.x;
    let height = r.bottom_right.y - r.top_left.y;
    return width * height;
}

// Test
let p1 = make_point(1, 2);
let p2 = make_point(4, 6);
let r = Rectangle { top_left: p1, bottom_right: p2 };
let area = rect_area(r);
println("Point p1:", p1);
println("Point p2:", p2);
println("Rectangle area:", area);
assert(area == 12, "Expected area 12");
println("All struct tests passed!");