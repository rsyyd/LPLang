// 08_async.lp — async/await and spawn concurrency

async fn slow_double(x: int) -> int {
    sleep(50);
    return x * 2;
}

async fn slow_add(a: int, b: int) -> int {
    sleep(30);
    return a + b;
}

// Spawn two tasks concurrently
let t1 = spawn slow_double(21);
let t2 = spawn slow_add(10, 32);

// Await both results
let r1 = await t1;
let r2 = await t2;

println("slow_double(21) =", r1);
println("slow_add(10, 32) =", r2);

assert(r1 == 42, "slow_double result");
assert(r2 == 42, "slow_add result");

// Direct await on async call
let r3 = await slow_double(7);
assert(r3 == 14, "direct await");
println("direct await slow_double(7) =", r3);

println("All async tests passed!");
