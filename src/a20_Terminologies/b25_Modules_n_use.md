# Modules and use: 101

- Why needed?
- Lets us organize code in crate for readability & reuse
- Allow us to control privacy of items (Code within module is private by default)
  - Lets you control the organization, scope and privacy of `paths`
- **Module & Crate**:
  - A module in a crate can be accessed anywhere inside the same crate as long as privacy rule allow
  - Syntax to access module:
    - Use `path to the code`
      - Example: crate::garden::vegetables::Asparagus
      - ^ (Here garden is a module, vegetables is a sub-module & Asparagus is a type)
- `crate` module
  - In the module tree, bin & lib files (ie `src/main.rs` & `src/lib.rs`) are part of module named `crate`

# Example module

backyard
├── Cargo.lock
├── Cargo.toml
└── src
├── garden
│   └── vegetables.rs
├── garden.rs
└── main.rs

`src/main.rs`:

```rust
use crate::garden::vegetables::Asparagus;
// Declaring module: 
//  - We declare module in crate root file (In this case binary root file main.rs)
//  - Once declared, compiler will look module code in,
//      - Inline: `pub mod garden {...}`
//      - In file: `src/garden.rs`
//      - In file: `src/garden/mod.rs`
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

`src/garden.rs`:

```rust
// Declaring submodule:
//  - A submodule can be declared in any file other than crate root
//  - Example:
//      . Here we declaring submodule `vegetables` in module `garden`
//  - Where does compiler look for submodule?
//      1. Inline: `pub mod vegetables{...}`
//      2. In file: `src/garden/vegetables.rs`
//      3. In file: `src/garden/vegetables/mod.rs`
pub mod vegetables;
```

# Inline module example


```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
