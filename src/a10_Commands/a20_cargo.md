# `cargo`

Available options,

1.`cargo --version`

# `init`

`cargo init`
- Is used to convert a plain project use `cargo`
- This creates `Cargo.toml` file for us

# `build`

`cargo build`
- Creates a debug build binary in /target/debug/<binary name>
- Option
  - `cargo build --release`
    - Builds binary with optimizations to run faster
    - Why not use --release all the time? Build is slower compared to `cargo build`
    - Creates the release binary in `/target/release`

# `run`

`cargo run`
- This both build and run the executable

# `check`
- Check code to make sure it compiles
- ^ why needed? Much faster than compiling the code 


