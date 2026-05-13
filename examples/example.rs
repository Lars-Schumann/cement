use auto_array::auto_array;

fn main() {
    auto_array! {
        // Additional attributes, docs, and visibility are supported.
        /// A const array.
        #[allow(unused)]
        const A: [u8; _] = [3, 3, 3];
        /// A static array with conditional compilation and pub(crate) visibility.
        pub(crate) static B: [u8; _] = [1, #[cfg(unix)] 2];
    }
    assert_eq!(A, [3, 3, 3]);
    assert_eq!(B, [1, #[cfg(unix)] 2]);
}
