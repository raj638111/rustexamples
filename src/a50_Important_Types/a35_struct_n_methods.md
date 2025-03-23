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

# Methods (Aka `Associated Functions`)

[Reference](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html)

1. Multiple `impl` blocks can be used

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

# Associated function without self (ie without instance)

1. Example: `String::from(...)`

```rust
impl Rectangle {
    // Here two self's are an alias for Rectangle type
    //  First self is return type, second self is used to create an instance
    fn square(size: u32) -> Self { 
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let sq = Rectange::square(3);
}
```

# Using `function call` syntax on struct methods

[Ref](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html#method-calls-are-syntactic-sugar-for-function-calls)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
       self.width * self.height
     }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let mut r = Rectangle { 
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r); // Function call syntax
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
}
```

# Methods & Ownership

[Ref](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html#methods-and-ownership)

1. Methods must be called on structs that have necessary ownership
