
# `loop` syntax

```rust
loop {
    ...
}
```

# `loop` with `continue` example

```rust
loop {

    let mut gess: String = ""
    # Here,
    #   When parse is good, value is assinged to guess variable
    #   When parse is bad, we continue in the loop
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_)  => continue
    }
}
```

# Returning value from a loop using `break` expression

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

# Loop labels example

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

# Condition loops with `while`

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

# Loop through collections with `for`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}


fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

