

# Error handling: [101](https://rust-book.cs.brown.edu/ch09-00-error-handling.html)

- Two error types
  - `Recoverable`
    - Examples:
      - File not found error
    - Rust provides `Result<T, E>` for recoverable errors 
  - `Unrecoverable`
    - Examples 
      - Accessing location beyond end of array
    - [`panic!` macro](https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html)
      - Explicit: Can be used to stop execution when program encounters unrecoverable errors
      - Implicit: Caused when job encounters unrecoverable errors 

# `panic!` & `abort`
    
- `panic!`: Stack unwind happens (Also cleans up the data) before the job terminates
- `abort`: Immediately aborts the job with any cleanup

# `RUST_BACKTRACE` 

- Provides the complete backtrace

```shell
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: rust_begin_unwind
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/std/src/panicking.rs:645:5
   1: core::panicking::panic_fmt
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:72:14
   2: core::panicking::panic_bounds_check
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/panicking.rs:208:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:255:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/alloc/src/vec/mod.rs:2770:9
   6: panic::main
             at ./src/main.rs:4:6
   7: core::ops::function::FnOnce::call_once
             at /rustc/07dca489ac2d933c78d3c5158e3f43beefeb02ce/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

# Recoverable errors (Result enum `Result<T, E>`)

### Example

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

### Matching different errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

### `unwrap` & `except`: Example

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}

use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

### [Propagating errors](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#propagating-errors) with match

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

### Propagating errors with [?](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) (Idiomatic): Example

- `?` acts like an equivalent of `return` statement we have used in `match`
- `from` function
  - `?` evaluates to `from` function which converts error type to function (the function where `?` is used) return type
  - Custom implementation for this conversion is done using `impl From<T>`
- Can also be used with `Option<T>` type
- [Limitation](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used)
  - The ? operator can only be used in functions whose return type is compatible with the value the ? is used on

Example
```rust
// Example 1
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Example 2
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

```



