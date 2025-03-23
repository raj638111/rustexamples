
# [Integration tests](https://rust-book.cs.brown.edu/ch11-03-test-organization.html#integration-tests)

- Are added `$root/tests` directory
- **Separate crate**
  - Each file in `tests` directory is a separate crate
- `#[cfg(test)]` not needed,
  - as cargo treats files in `tests` directory separately. 
  - only compiles when `cargo test` is run

```shell
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs

```

```rust
use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
```

- Run all integration tests in specific file (Example: file `integration_test.rs`)
```shell
$ cargo test --test integration_test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```


# [Submodules](https://rust-book.cs.brown.edu/ch11-03-test-organization.html#submodules-in-integration-tests): Where to place common code?... & avoiding tests for common code

- Create a subdirectory in `tests`
  - Codes in subdirectory is not run

**Example:**
```shell
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Here ^ codes in `common` is not run as tests

**Using common code in other tests:**
```rust
use adder::add_two;

mod common; // Enable access to common code

#[test]
fn it_adds_two() {
    common::setup(); // Common code used here

    let result = add_two(2);
    assert_eq!(result, 4);
}
```

# Integration test for binary crates

- Binary crates code cannot be test from code in `tests` directory
- **Good practice**
  - Keep `main.rs` small and move all the code to library crates so that those codes can be tested