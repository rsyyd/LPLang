# Control Flow

```lp
# If expressions
let result = if x > 0 { "positive" } else { "negative" };

# Match expressions
match value {
    0 => "zero",
    1 => "one",
    n => format!("number: {}", n),
}

# Loops
while condition { body }
for item in collection { body }
loop { body }
```