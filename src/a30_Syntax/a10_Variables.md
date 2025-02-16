# Immutable

`let apples = 5;`
Variables by default are immutable

# Mutable

`let mut apples = 5;`

# Explicit type

`let guess: u32 = 5;`

# Type inference

`let mut guess = String::new()`
1. Here we are not explicitly specifying the type of the variable, so Rust use type inference to identify the type

# Shadowing

1. Helps (or Confuses :)) use to shadow a variable of same name with different type
2. Example
```rust
let mut guess = String::new();
let guess: u32 = guess.trime().parse)().except("Please type a number!")
```