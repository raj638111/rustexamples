
# `match` expression 101

1. `match` expression is made up of `arms`
2. Each `arm` contains a pattern
3. Example
```rust
// Here `Ordering` is an enum which has three variants: `Less`, `Greater` & `Equal`
use std::cmp::Ordering; 
match guess.cmp(&no) {
    Ordering::Less => println!("Too small")
    Ordering::Greater => println!("Too big")
    Ordering::Equal => {
        print("Good")
        break; // Note: break statement exits the `loop` (`loop` not shown here)
    }
}
```

# Match and Ownership

[Ref](https://rust-book.cs.brown.edu/ch06-02-match.html#how-matches-interact-with-ownership)
