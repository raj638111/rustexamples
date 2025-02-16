# `cargo`

Available options,

1.`cargo --version`

# `init`

`cargo init`
- Is used to convert a plain project use `cargo`
- This creates `Cargo.toml` file for us

# `cargo build`

- Creates a debug build binary in /target/debug/<binary name>
- Option
  - `cargo build --release`
    - Builds binary with optimizations to run faster
    - Why not use --release all the time? Build is slower compared to `cargo build`
    - Creates the release binary in `/target/release`

# `cargo update`

1. Ignores the `Cargo.lock` file, checks if there are any new versions (within the specific version range) of the dependencies, downloads that version and create a new `Cargo.lock` file

# `cargo run`

- This both build and run the executable

# `cargo check`
- Check code to make sure it compiles
- ^ why needed? Much faster than compiling the code 

# `cargo doc`

`cargo doc --open`
1. Builds documentation provided by all the dependencies locally and open it in the browser

