// 12_for_in_loop.lp — for-in loop and range operator

// Range 0..5 (inclusive)
println("Counting 0 to 5:");
for i in 0..5 {
    println(i);
}

// Iterate over list
let fruits = ["apple", "banana", "cherry"];
println("\\nFruits:");
for fruit in fruits {
    println(fruit);
}

// Sum of numbers 1..10
var sum = 0;
for n in 1..10 {
    sum = sum + n;
}
println("Sum 1..10:", sum);
assert(sum == 55, "sum should be 55");

// Nested loops
println("\\nMultiplication table:");
for i in 1..3 {
    for j in 1..3 {
        print(i * j);
        print(" ");
    }
    println("");
}

println("\\nFor-in loop tests passed!");