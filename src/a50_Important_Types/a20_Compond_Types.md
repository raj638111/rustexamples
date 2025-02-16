
# Tuple

## Example 1

```rust
let tup: (i32, f64) = (500, 6.4);
let (x, y) = tup;
let a = tup.0;
let tup: () = (); // ** Empty tuple is generally called `unit`
```

## Example 2: Mutable tuple

```rust
let mut x: (i32, i32) = (1, 2);
x.0 = 0
x.1 += 5
```

# Array

1. Data is allocated on stack
2. Is fixed length
3. Accessing Invalid index
   1. Safety: Rust checks out of bound index, unlike other low level language which might access invalid index 

## Example 1

```rust
let a = [1, 2, 3];
let b = [3; 5] // 3 is repeated 5 times
```
