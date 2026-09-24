// 09_try_operator.lp — `?` operator for Result/Option propagation

import result

fn divide(a: int, b: int) -> Result {
    if b == 0 {
        return Result::Err("division by zero")
    }
    return Result::Ok(a / b)
}

// Fails if any step divides by zero
fn compute_pipeline(a: int, b: int, c: int) -> Result {
    let r1 = divide(a, b)?
    let r2 = divide(r1, c)?
    return Result::Ok(r2)
}

fn unwrap_or_404(res: Result) -> int {
    return match res {
        Ok(v) => v,
        Err(_) => 404,
    }
}

let ok = compute_pipeline(100, 2, 5)
let err1 = compute_pipeline(100, 0, 5)
let err2 = compute_pipeline(100, 2, 0)

let v_ok = unwrap_or_404(ok)
let v_err1 = unwrap_or_404(err1)
let v_err2 = unwrap_or_404(err2)

println("pipeline(100, 2, 5) =", v_ok)
println("pipeline(100, 0, 5) =", v_err1)
println("pipeline(100, 2, 0) =", v_err2)

assert(v_ok == 10, "100 / 2 / 5 = 10")
assert(v_err1 == 404, "first step fails -> 404")
assert(v_err2 == 404, "second step fails -> 404")

println("Result propagation with `?` works!")