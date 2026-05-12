#![no_std]

#[macro_export]
macro_rules! auto_array {
    ($($(#[$attr:meta])* $vis:vis $const_or_static:ident $name:ident: [$ty:ty; _] = $array:expr;)+) => {
        $($(#[$attr])* $vis $const_or_static $name: [$ty; <[$ty]>::len(&($array))] = $array;)+
    };
}

#[cfg(test)]
mod tests {

    macro_rules! assert_type {
        ($path:path, $ty:ty ) => {
            let _: $ty = $path;
        };
    }

    #[test]
    fn single_arrays() {
        auto_array!(
            pub const ARRAY_1: [i32; _] = [1, 2, 3];
        );
        assert_type!(ARRAY_1, [i32; 3]);
        assert_eq!(ARRAY_1, [1, 2, 3]);

        auto_array!(
            pub(super) static ARRAY_2: [i8; _] = [4, 4, 4, 4];
        );
        assert_type!(ARRAY_2, [i8; 4]);
        assert_eq!(ARRAY_2, [4, 4, 4, 4]);
    }

    #[test]
    fn multiple_arrays() {
        auto_array!(
            pub const ARRAY_1: [i8; _] = [1, 2, 3];
            pub const ARRAY_2: [i16; _] = [];
            pub const ARRAY_3: [i32; _] = [4, 4, 4, 4];
            pub const ARRAY_4: [i64; _] = [6, 6, 6, 6, 6, 6];
        );
        assert_type!(ARRAY_1, [i8; 3]);
        assert_type!(ARRAY_2, [i16; 0]);
        assert_type!(ARRAY_3, [i32; 4]);
        assert_type!(ARRAY_4, [i64; 6]);
        assert_eq!(ARRAY_1, [1, 2, 3]);
        assert_eq!(ARRAY_2, []);
        assert_eq!(ARRAY_3, [4, 4, 4, 4]);
        assert_eq!(ARRAY_4, [6, 6, 6, 6, 6, 6]);

        auto_array!(
            pub static ARRAY_5: [u32; _] = [1];
            pub static ARRAY_6: [u64; _] = [2, 2];
        );
        assert_type!(ARRAY_5, [u32; 1]);
        assert_type!(ARRAY_6, [u64; 2]);
        assert_eq!(ARRAY_5, [1]);
        assert_eq!(ARRAY_6, [2, 2]);
    }

    #[test]
    fn mix_const_and_static() {
        auto_array!(
            pub const ARRAY_1: [u8; _] = [2, 2];
            pub static ARRAY_2: [u8; _] = [3, 3, 3];
        );
        assert_type!(ARRAY_1, [u8; 2]);
        assert_type!(ARRAY_2, [u8; 3]);
        assert_eq!(ARRAY_1, [2, 2]);
        assert_eq!(ARRAY_2, [3, 3, 3]);
    }

    #[test]
    #[deny(dead_code)]
    fn attributes() {
        auto_array!(
            #[expect(dead_code)]
            pub const UGLY_ARRAY: [f32; _] = [5.0];
        );
    }
}
