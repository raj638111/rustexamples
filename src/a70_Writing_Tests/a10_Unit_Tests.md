

# Unit Test: [101](https://rust-book.cs.brown.edu/ch11-01-writing-tests.html#the-anatomy-of-a-test-function)

- Use `test` attribute to create test function
- Use `cargo test` to run all tests
- New library project & tests
  - When new lib project is created with `cargo`, test module with test functions are created for us
- Idiomatic test structure
  - **Unit tests**: Goes in the same file as the code
  - **Integration tests**: Goes in a separate directory

```shell
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
The #[cfg(test)] annotation on the tests module tells Rust to compile and run 
the test code only when you run cargo test, not when you run cargo build. 
This saves compile time when you only want to build the library and saves 
space in the resultant compiled artifact because the tests are not included.
*/
#[cfg(test)]
mod tests {
    /*
    Because the tests module is an inner module, we need to bring the code under test in 
    the outer module into the scope of the inner module. We use a glob here, so 
    anything we define in the outer module is available to this tests module.
     */
    use super::*; 

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

```shell
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


```