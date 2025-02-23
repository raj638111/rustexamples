# 101

1. Borrow checker will track ownership in both struct & field level

# Basic example

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Access / mutate a value
    user1.email = String::from("anotheremail@example.com");
}
```

# Using `Field Init Shorthand`

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

# Creating an instance from another instance using `Struct Update Syntax`

### Ownership moved

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        // Struct Update Syntax
        // user1 can no longer be used, as `username` from user1 is moved to user2
        ..user1 
    };
}
```

### Ownership not moved (Fields with `Copy Trait`)

```rust
fn main() {
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("someuser")
        // Struct Update Syntax
        // user1 can still be used, as fields active & sign_in_count implements copy trait when
            // assigned to user2
        ..user1 
    };
}
```

# References in struct do not work without using `lifetimes`

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```

```shell
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
error[E0106]: missing lifetime specifier
 --> src/main.rs:3:15
  |
3 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 ~     username: &'a str,
  |

error[E0106]: missing lifetime specifier
 --> src/main.rs:4:12
  |
4 |     email: &str,
  |            ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 |     active: bool,
3 |     username: &str,
4 ~     email: &'a str,
  |

For more information about this error, try `rustc --explain E0106`.
error: could not compile `structs` (bin "structs") due to 2 previous errors

```
