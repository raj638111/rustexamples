

# Attribute: 101

- Implements certain trait for the type being specified 
- Helps to avoid boilerplate code
- Attribute syntax: `#[]`

Example:
```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
```

Here,
- `Debug`
  - Allows enum to be printed in human readable format using `{:?}` format specifier
  - Implements trait `std::fmt::Debug`
```rust
fn main(){
    let my_shirt = ShirtColor::Red;
    println ! ("{:?}", my_shirt);  // Outputs: "Red"
}
```  

- `Copy`
  - Implements the `Copy` trait, which indicates that this type can be 
    duplicated by simply copying its bits (no complex ownership rules apply)


