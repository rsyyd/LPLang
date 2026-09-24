// 06_lists.lp — List operations

var numbers = [10, 20, 30, 40, 50]

println("List:", numbers)
println("Length:", len(numbers))
println("First:", numbers[0])
println("Last:", numbers[4])

// Mutation via index assignment
numbers[2] = 99
assert(numbers[2] == 99, "index assignment")

// Append
append(numbers, 60)
assert(len(numbers) == 6, "length after append")
assert(numbers[5] == 60, "last after append")

// Build a list with a loop
var squares = []
var i = 0
while i < 5 {
    append(squares, i * i)
    i = i + 1
}
println("Squares:", squares)
assert(len(squares) == 5, "5 squares")
assert(squares[3] == 9, "3^2 = 9")
assert(squares[4] == 16, "4^2 = 16")

// String indexing
let greeting = "LUMPO"
assert(greeting[0] == "L", "first char")
assert(greeting[4] == "O", "last char")
assert(len(greeting) == 5, "string length")

println("All list tests passed!")