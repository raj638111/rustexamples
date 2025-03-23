
# [Closures: 101](https://rust-book.cs.brown.edu/ch13-01-closures.html)

- Are **anonymous functions** that also **saves variable**
- Can also be passed as argument to another function
- How to specify closure expression
  - `|| <closure expression>`
  - Example: `|| self.most_stocked()`
- Example:
  - Here `unwrap_or_else` method takes a closure
```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        ...
    }
}

```

# [Type inference](https://rust-book.cs.brown.edu/ch13-01-closures.html#closure-type-inference-and-annotation)

- Compiler can infer parameter or return type for closures (whereas functions need explicit type annotation)
- Example: **Explicit type annotation to type inference in closure**
```rust
fn main() {
    fn add_one_v1(x: u32) -> u32 { x + 1 }  // Function
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure 1
    let add_one_v3 = |x| { x + 1 }; // Closure 2
    let add_one_v4 = |x| x + 1  ;   // Closure 3
}
```