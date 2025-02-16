
# Integer 

1. Rust number types can have a value b/w 1 and 100
2. `i32` is the default type
3. Available types
   1. i8, u8
   2. i16, u16
   3. i32, u32
   4. i64, u64
   5. i128, u128
   6. isize, usize
4. Handling overflow
   1. Wrap with `wrapping*` methods
   2. Return `None` using `checked_*` methods
   3. Return value & boolean using `overflowing_*` methods
   4. Saturate the min & max with `saturating_*` methods
5. Number literal examples
   Decimal	      98_222 (Note: _ can be used for better readability)
   Hex	          0xff
   Octal	      0o77
   Binary	      0b1111_0000
   Byte (u8 only) b'A'
 
# Float points

1. Available types
   1. f32 (Single precision float)
   2. f64 (Default - Double precision float) 

# Boolean

`let f: bool = false;`

# Character

`let c: char = 'z'`