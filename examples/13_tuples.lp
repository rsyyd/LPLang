// 13_tuples.lp — tuple literals, indexing, and function returns

fn get_user() -> tuple {
    return ("alice", 25, true);
}

// Basic tuple
let pair = (10, 20);
println("Pair:", pair);
println("pair[0]:", pair[0]);
println("pair[1]:", pair[1]);
assert(pair[0] == 10, "pair[0] should be 10");
assert(pair[1] == 20, "pair[1] should be 20");

// Mixed-type tuple
let user = get_user();
println("User:", user);
println("Name:", user[0]);
println("Age:", user[1]);
println("Active:", user[2]);
assert(user[0] == "alice", "name check");
assert(user[1] == 25, "age check");
assert(user[2] == true, "active check");

// Nested tuples
let nested = ((1, 2), (3, 4));
println("Nested:", nested);
let first_inner = nested[0];
println("First inner[1]:", first_inner[1]);
assert(first_inner[1] == 2, "nested check");

// Empty tuple
let empty = ();
println("Empty tuple:", empty);

println("All tuple tests passed!");
