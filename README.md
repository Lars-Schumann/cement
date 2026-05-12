## What?
Automatically calculates the length of Arrays for you.

## How?
```rust
use auto_array::auto_array;

fn main() {
    auto_array!(
        pub const ARRAY1: [i32; _] = [1, 2, 3];
        static ARRAY2: [u8; _] = [4, 4, 4, 4];
    );

    assert_eq!(ARRAY1, [1, 2, 3]);
    assert_eq!(ARRAY2, [4, 4, 4, 4]);
}
```
