// 04_modules.lp — demonstrates multi-file programs with import

import math_utils

let s = square(7)
let c = cube(3)
let m = max(s, c)

println("square(7) =", s)
println("cube(3) =", c)
println("max =", m)

assert(s == 49, "square of 7")
assert(c == 27, "cube of 3")
assert(m == 49, "max of 49 and 27")

println("Module import works!")