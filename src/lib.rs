#![no_std]

/// # Examples
/// ```rust
/// use auto_array::auto_array;
/// auto_array! {
///     // Additional attributes, docs, and visibility are supported.
///     /// A const array.
///     #[allow(unused)]
///     const A: [u8; _] = [3, 3, 3];
///     /// A static array with conditional compilation and pub(crate) visibility.
///     pub(crate) static B: [u8; _] = [1, #[cfg(unix)] 2];
/// }
/// assert_eq!(A, [3, 3, 3]);
/// assert_eq!(B, [1, #[cfg(unix)] 2]);
/// ```
#[macro_export]
macro_rules! auto_array {
    ($($(#[$attr:meta])* $vis:vis $const_or_static:ident $name:ident: [$ty:ty; _] = $array:expr;)*) => {
        $($(#[$attr])* $vis $const_or_static $name: [$ty; <[$ty]>::len(<[$ty; _]>::as_slice(&$array))] = $array;)*
    };
}

#[cfg(test)]
mod tests {
    macro_rules! assert_type_and_value {
        ($($lhs:path: $ty:ty = $rhs:expr;)*) => {
            $(let _: $ty = $lhs;
            assert_eq!($lhs, $rhs);)*
        };
    }

    #[test]
    fn single_arrays() {
        auto_array!(
            pub const A: [i32; _] = [1, 2, 3];
        );
        auto_array!(
            pub(super) static B: [i8; _] = [4, 4, 4, 4];
        );
        assert_type_and_value!(
            A: [i32; 3] = [1, 2, 3];
            B: [i8; 4] = [4, 4, 4, 4];
        );
    }

    #[test]
    fn multiple_arrays() {
        auto_array!(
            const A: [i8; _] = [1, 2, 3];
            const B: [i16; _] = [];
            const C: [i32; _] = [4, 4, 4, 4];
            const D: [i64; _] = [6, 6, 6, 6, 6, 6];
        );
        assert_type_and_value!(
            A: [i8; 3] = [1, 2, 3];
            B: [i16; 0] = [];
            C: [i32; 4] = [4, 4, 4, 4];
            D: [i64; 6] = [6, 6, 6, 6, 6, 6];
        );

        auto_array!(
            static E: [u32; _] = [1];
            static F: [u64; _] = [2, 2];
        );
        assert_type_and_value!(
            E: [u32; 1] = [1];
            F: [u64; 2] = [2, 2];
        );
    }

    #[test]
    fn mix_const_and_static() {
        auto_array!(
            const A: [u8; _] = [2, 2];
            static B: [u8; _] = [3, 3, 3];
        );
        assert_type_and_value!(
            A: [u8; 2] = [2, 2];
            B: [u8; 3] = [3, 3, 3];
        );
    }

    #[test]
    #[deny(dead_code, clippy::approx_constant)]
    fn attributes() {
        auto_array!(
            #[expect(dead_code)]
            #[expect(clippy::approx_constant)]
            pub(crate) const UNUSED: [f32; _] = [3.14];
        );
    }

    #[test]
    fn empty_invocation() {
        auto_array!();
    }
}
