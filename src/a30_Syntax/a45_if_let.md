
# 101

1. Is pretty much similar to partial function in scala

# Example: Without `let`

```rust
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}
```

# Example: With `let` (Works like partial function)

```rust
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max { // With `let`
        println!("The maximum is configured to be {max}");
    }
}
```

# Example: With `let` (With exhaustive match)

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
```
