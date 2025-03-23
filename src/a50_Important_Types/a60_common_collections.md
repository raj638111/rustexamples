

# Vector: [101](https://rust-book.cs.brown.edu/ch08-01-vectors.html)

- `Heap`
  - All data in collections are stored in heap 

### Vector (`Vec`)

```rust
fn main() {
    // Using `new`
    let v: Vec<i32> = Vec::new();
    // Using `vec!` macro
    let v = vec![1, 2, 3];
    
}
```

# `String`: [101](https://rust-book.cs.brown.edu/ch08-02-strings.html)

- A String is a wrapper over a `Vec<u8>`

```rust
fn main() {
    let mut s = String::new();
    let s = String::from("initial contents");

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
}
```

### Rust does not support indexing

```rust
fn main() {
    let s1 = String::from("hello");
    let h = s1[0]; // Error: Not supported
}
```

# `HashMap`: [101](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html#storing-keys-with-associated-values-in-hash-maps)

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```