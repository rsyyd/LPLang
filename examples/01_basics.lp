// Hello World and basic control flow in LPLang

fn add(a: int, b: int) -> int {
    return a + b;
}

fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

// Immutable binding
let greeting = "Hello from Lumpo Language (LPLang)!";
println(greeting);

// Mutable binding and while loop
var count = 0;
var total = 0;
while count < 5 {
    total = add(total, count);
    count = count + 1;
}

println("Sum 0..4 =", total);
println("Factorial(5) =", factorial(5));

// Assertion self-check
assert(total == 10, "Total must be 10");
assert(factorial(5) == 120, "Factorial 5 must be 120");
println("All assertions passed!");
