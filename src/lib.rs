#![no_std]

#[macro_export]
macro_rules! auto_array {
    ($vis:vis const $name:ident: [$ty:ty; _] = $array:expr;) => {
        $vis const $name: [$ty;<[$ty]>::len(&($array))] = $array;
    };
    ($vis:vis static $name:ident: [$ty:ty; _] = $array:expr;) => {
        $vis static $name: [$ty; <[$ty]>::len(&($array))] = $array;
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
}
