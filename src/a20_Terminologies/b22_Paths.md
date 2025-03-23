
# Paths: 101
 
- A way to find item such as struct, function or module in a module tree
- Two types:
  1. Absolute
  2. Relative
- `pub` keyword
  - Is a way to make an item public for access

# Example: Absolute & Relative path

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

# Example: Exposing items using `pub` keyword

```rust
mod front_of_house {
    // Note here: we need to make both the module & the function `pub` to expose the `add_to_waitlist` function 
    pub mod hosting {  
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

# Example: Bring a path in scope with `use` keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

# Example: Error 

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // NOTE: This is wrong as hosting path is defined in `crate` module (ie the root module) scope
        hosting::add_to_waitlist(); 
    }
}
```

```shell
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted


```

```rust
// Error fixed code
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

# Alias name example

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

# Example:: Reexporting names with `pub use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Note: Now external modules can the name `hosting` to access `add_to_waitlist()`
pub use crate::front_of_house::hosting; 

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

# Example: Using external packages

`Cargo.toml`
```toml
rand = "0.8.5"
```

```rust
// Here `rand` is the crate name
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```


# Example: `Nested paths` & `Glob` operator 

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
use std::io::{self, Write};
use std::collections::*;
```
