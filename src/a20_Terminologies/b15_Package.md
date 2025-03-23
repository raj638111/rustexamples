
# Package: 101

- Single package can contain multiple binary crates & one library crate
- Contains `cargo.toml` that specify how to build those crates
  - Example package:
    - `Cargo` itself is a package that contains binary crate
- A package must contain at least one crate
- Defaults
  - `src/main.rs`: Is the root of a binary crate with same name as package
  - `src/lib.rs`: Is the root of a lib crate with same name as package
  - `src/bin` directory: Each file in the directory will be treated as a separate binary crate

# Example: Creating a package

```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```