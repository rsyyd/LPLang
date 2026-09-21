// 05_error_handling.lp — Result/Option from stdlib

import result;

fn divide(a: int, b: int) -> Result {
    if b == 0 {
        return Result::Err("division by zero");
    }
    return Result::Ok(a / b);
}

fn find_positive(x: int) -> Option {
    if x > 0 {
        return Option::Some(x);
    }
    return Option::None;
}

let r1 = divide(10, 3);
let r2 = divide(10, 0);

assert(is_ok(r1) == true, "10/3 is ok");
assert(is_ok(r2) == false, "10/0 is err");
assert(unwrap_result_or(r1, -1) == 3, "10/3 = 3");
assert(unwrap_result_or(r2, -1) == -1, "10/0 fallback -1");

let o1 = find_positive(5);
let o2 = find_positive(-3);

assert(is_some(o1) == true, "5 is some");
assert(is_some(o2) == false, "-3 is none");
assert(unwrap_or(o1, 0) == 5, "unwrap some(5)");
assert(unwrap_or(o2, 0) == 0, "unwrap none fallback 0");

println("Error handling tests passed!");
