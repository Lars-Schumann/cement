use auto_array::auto_array;

fn main() {
    auto_array!(
        pub const A: [i32; _] = [1, 2, 3];
        static B: [u8; _] = [4, 4, 4, 4];
    );
    assert_eq!(A, [1, 2, 3]);
    assert_eq!(B, [4, 4, 4, 4]);
}
