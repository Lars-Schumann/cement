#![no_std]

#[macro_export]
macro_rules! auto_array {
    ($($vis:vis const $name:ident: [$ty:ty; _] = $array:expr;)+) => {
        $($vis const $name: [$ty;<[$ty]>::len(&($array))] = $array;)+
    };
    ($($vis:vis static $name:ident: [$ty:ty; _] = $array:expr;)+) => {
        $($vis static $name: [$ty; <[$ty]>::len(&($array))] = $array;)+
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
        const _: [i32; 3] = ARRAY_1;
        assert_type!(ARRAY_1, [i32; 3]);
        assert_eq!(ARRAY_1, [1, 2, 3]);

        auto_array!(
            pub(super) static ARRAY_2: [i32; _] = [4, 4, 4, 4];
        );
        assert_type!(ARRAY_2, [i32; 4]);
        assert_eq!(ARRAY_2, [4, 4, 4, 4])
    }

    #[test]
    fn multiple_arrays() {
        auto_array!(
            pub const ARRAY_1: [i32; _] = [1, 2, 3];
            pub const ARRAY_2: [i32; _] = [];
            pub const ARRAY_3: [i32; _] = [4, 4, 4, 4];
            pub const ARRAY_4: [i32; _] = [6, 6, 6, 6, 6, 6];
        );
        assert_type!(ARRAY_1, [i32; 3]);
        assert_type!(ARRAY_2, [i32; 0]);
        assert_type!(ARRAY_3, [i32; 4]);
        assert_type!(ARRAY_4, [i32; 6]);
        assert_eq!(ARRAY_1, [1, 2, 3]);
        assert_eq!(ARRAY_2, []);
        assert_eq!(ARRAY_3, [4, 4, 4, 4]);
        assert_eq!(ARRAY_4, [6, 6, 6, 6, 6, 6]);

        auto_array!(
            pub static ARRAY_5: [i32; _] = [1];
            pub static ARRAY_6: [i32; _] = [2, 2];
        );
        assert_type!(ARRAY_5, [i32; 1]);
        assert_type!(ARRAY_6, [i32; 2]);
        assert_eq!(ARRAY_5, [1]);
        assert_eq!(ARRAY_6, [2, 2]);
    }
}
