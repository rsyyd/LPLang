// 07_native.lp — subset supported by the EXPERIMENTAL native C backend

fn square(x: int) -> int {
    return x * x;
}

fn fib(n: int) -> int {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

var total = 0;
var i = 1;
while i <= 10 {
    total = total + i;
    i = i + 1;
}

println("Sum 1..10:", total);
println("square(7):", square(7));
println("fib(10):", fib(10));

if total == 55 && square(7) == 49 {
    println("Conditions OK");
} else {
    println("Conditions FAILED");
}
