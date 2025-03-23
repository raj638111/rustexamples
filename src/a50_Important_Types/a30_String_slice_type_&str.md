
# 101

Slices let you reference a contiguous sequence of elements in a collection rather than 
the whole collection. A slice is a kind of reference, so it is a non-owning pointer.

# Example

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s;
}
```

