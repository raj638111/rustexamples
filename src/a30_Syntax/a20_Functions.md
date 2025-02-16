
# Defining a function

```rust
// Function that returns an integer
fn main() -> i32 {
    ...
}
```

# Calling Associated function

`let mut str = String::new()`
- Here we are calling function `new()` from type `String`

# Expression vs statement

- Expression returns a value whereas a statement will not

### Expression

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // Note: NO SEMICOLON: Expression does not include semicolon in last statement
              //    - Adding a semicolon converts an expression into a statement
    };
    println!("The value of y is: {y}");
}
```

