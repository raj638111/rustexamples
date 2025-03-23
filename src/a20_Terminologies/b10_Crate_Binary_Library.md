
# Crate

- Is the smallest amount of code rust compiler considers at a time
   1. Example: Even passing a single file to `rustc`, that file is considered as a `crate`
- Crates can contain `modules`
  - Modules may be defined in other files that get compiled with crate 
- Is a collection of Rust source code files
- Types of crate
   1. `Binary` crate (Which contains binary that can be run)
      - A package can have **multiple binary crates** by placing files on `src/bin` 
   2. `Library` crate (Which contains code that can be used by other programs)
- `crate root`
  - Is a source file rust compile starts with & makes up the `root module`
- `Crates.io` is the public repo from which `Cargo` fetches a given crate