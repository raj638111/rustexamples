# Ownership: 101

- Is all about **heap management** and safety
  - In another sense, we can also say this as **Pointer management**

# Frame

1. A variables live in `frame`
2. Is a mapping from variable to value within single scope (Example: function scope)

# Freeing / Dropping

1. De-allocation of frame is called freeing / dropping

# `Box`, `RawVec`, Pointers and Pointee

1. `Box` & Box like constructs like `RawVec`s are used to put data in heap
2. Box is used by rust data structures like `Vec`, `String` and `HashMap` to hold variable no of elements
3. Box de-allocation
   1. If a variable owns a box, if rust de-allocates the variables frame, rust also de-allocates the box'es heap memory

**`Box` Example:**

```rust
fn main() {
    let a_num = 4;
    make_and_drop();
}

fn make_and_drop() {
    let a_box = Box::new(5);
}
```

# Compile time ownership safety (Move & Borrow)

**Concept**: Once the ownership is moved from variable a to variable b, accessing variable a will result
in compile time error

```rust
fn main() {
    // NOTE: Here Box is not used. `String` is implemented with `Vec` and `Vec`
   //   is implemented with `RawVec` (which is Box like)
    let first = String::from("Ferris"); 
    let full = add_suffix(first);
    println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

```shell
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
```

# `Reference`

- Why needed?
  - Helps to avoid programming difficulty that comes with ownership and move
- What a reference does?
  - A reference variable is a **non-owning pointer**. That means,
    - A reference variable does not own a heap space
    - So, ownership & de-allocation is not applied to heap space pointed by reference variable
- How to create?
  - Using `&` and `&mut`)
- How to deference?
  - Using `*`
- TLDR
  - All variables can read, own, and (optionally) write their data.
  - Creating a reference will transfer permissions from the borrowed place to the reference.
  - Permissions are returned once the reference’s lifetime has ended.
  - Data must outlive all references that point to it.

### Example: A not concise program

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
```

### Example: A concise program using Reference

```rust
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

//NOTE: Here g1 and g2 are reference variable which in turn points to m1 & m2
fn greet(g1: &String, g2: &String) {  
    println!("{} {}!", g1, g2);
}
```

# We `dereference` a pointer to access its data (Using `*` operator)

**Example:**

```rust
fn main() {
   let mut x: Box<i32> = Box::new(1);
   let a: i32 = *x;         // *x reads the heap value, so a = 1
   *x += 1;                 // *x on the left-side modifies the heap value,
                            //     so x points to the value 2

   let r1: &Box<i32> = &x;  // r1 points to x on the stack
   let b: i32 = **r1;       // two dereferences get us to the heap value

   let r2: &i32 = &*x;      // r2 points to the heap value directly
   let c: i32 = *r2;    // so only one dereference is needed to read it
}
```

# Implicit & Explicit Dereferencing

**Convenience**:

- Rust implicitly dereference the value in many cases
- Also does implicit multi level/layers deference

**Example:**

```rust
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);

```

# Data can **aliased** or **mutated**, but both CANNOT be done

**Compile error?**:

```rust
fn main() {
  let mut v: Vec<i32> = vec![1, 2, 3];

  // Here num is an alias of v
  let num: &i32 = &v[2]; 

  // We are mutating v here where the old allocation will be deleted and 
  //  new allocation will be created
  v.push(4); 

  // NOT good: Here we are referencing an old allocation that is no longer available
  println!("Third element is {}", *num);}
```

# References changes Permissions on **Places** (Read, Write, Own)

- `Place`
  - Is anything that is put on left hand side of the assignment
  - Example
    - Variables, like `a
    - Dereferences of places, like `*a`
    - Array accesses of places, like `a[0]`
    - ...
- Who enforce permission / who finds out permission violation?
  - **`Borrow Checker`**

```rust
fn main() {
  let mut v: Vec<i32> = vec![1, 2, 3];  
                                        // v^(+R,+W+O) (After execution of above line)
  let num: &i32 = &v[2]; // num being an immutable reference here   
                                        // v>(+R,-W,-O) (After execution of above line)
                                        // num^(+R, -, +O)
                                        // *num (+R,-,-)  
  println!("Third element is {}", *num);
  println!("Again, the third element is {}", *num);
                                        // v@(R,+W,+O)
                                        // ...
                                        //...
  v.push(4);
}
```

# Mutable reference provide non-owning accessing to data

1. Mutable reference **allow** mutation, but **prevent** aliasing
2. Can also be temporarily downgraded to read-only references

![mutable_reference.png](assets/mutable_reference.png)

# Permissions are returned at the end of reference lifetime

```rust

fn main() {
  let mut x = 1;
  let y = &x;
  let z = *y;
            // As the life of y ends here, the W permission is returned to x
  x += z;
}
```

# Data must outlive all of its references

### **Case 1**: References created & dropped within scope of single function

(Compile error)

```rust
fn main() {
  let s = String::from("Hello world");
  let s_ref = &s;
  drop(s); // s cannot be dropped here, as it does not have O permission 
  println!("{}", s_ref);
}
```

### Case 2: Reference that are either **input** to a function or **output** from a function

- Flow (`F`) Permission
  - `F` permission is expected whenever an expression uses an input reference or returns an output reference
  - `F` does not change throughout the body of the function
  - A reference has `F` permission if it is allowed to be used in a particular expression

**This code compiles**

```rust
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0]; // (RF: expected & available)
    s_ref                    // (RF: expected & available)
}
```

**This code do not compile**

```rust
fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    drop(default);
    println!("{}", s); // s could also be default, but we dropped default earlier
}

fn first_or(strings: &Vec<String>, default: &String) -> &String {
  if strings.len() > 0 {
    &strings[0] // RF expected: Only R is available
  } else {
    default // RF expected: Only R is available
  }
}
```

```shell
error[E0106]: missing lifetime specifier
 --> test.rs:1:57
  |
1 | fn first_or(strings: &Vec<String>, default: &String) -> &String {
  |                      ------------           -------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `strings` or `default`

```
