
# Enum: 101

```rust
// Define enum
enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    // Create instance
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
```

# Enum with data

```rust
fn main() {    
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1")); // Enum with data
    let loopback = IpAddr::V6(String::from("::1"));

    // Another example with different data types associated with enum
    enum IpAddrV2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }    
    
}
```

# Enum with methods

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();    
}
```

# Useful enums

### `Option` enum

```rust
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

