

# `Result`

1. Is an `enumeration`
2. Contains two `variants`
   - `ok`
   - `Err`

# Useful methods

1. `except("error message")`
   1. Causes the program to crash if the variant is `Err` and displays the error message provided
   2. When not called, results in compiler warning (This warning can be suppressed by writing an error handling code)
      3. ```
         warning: unused `Result` that must be used
           --> src/main.rs:10:5
            |
         10 |     io::stdin().read_line(&mut guess);
            |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            |
            = note: this `Result` may be an `Err` variant, which should be handled
         ```