
# Trait & Implementation
___
1. In order to use an implementation of a trait, we should import (`use`) that trait
2. Example
```rust
use rand::Rng; // Here Rng is a trait
// Here method `gen_range` is defined in `Rng` trait, so we have to import `Rng` trait
let secret_no = rand::thread_rng().gen_range(1..=100)  
```