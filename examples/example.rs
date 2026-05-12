use auto_array::auto_array;

fn main() {
    auto_array!(
        pub const ARRAY1: [i32; _] = [1, 2, 3];
    );
    auto_array!(
        static ARRAY2: [i32; _] = [4, 4, 4, 4];
    );

    assert_eq!(ARRAY1, [1, 2, 3]);
    assert_eq!(ARRAY2, [4, 4, 4, 4]);
}
