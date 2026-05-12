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

    #[test]
    fn single_arrays() {
        auto_array!(
            pub const ARRAY_1_IN: [i32; _] = [1, 2, 3];
        );
        const ARRAY_1_OUT: [i32; 3] = [1, 2, 3];
        assert_eq!(ARRAY_1_IN, ARRAY_1_OUT);

        auto_array!(
            pub(super) static ARRAY_2_IN: [i32; _] = [4, 4, 4, 4];
        );
        const ARRAY_2_OUT: [i32; 4] = [4, 4, 4, 4];
        assert_eq!(ARRAY_2_IN, ARRAY_2_OUT)
    }
}
