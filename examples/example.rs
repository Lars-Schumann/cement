#![feature(const_heap)]
#![feature(const_trait_impl)]
#![feature(maybe_uninit_array_assume_init)]

use cement::cement;

fn main() {
    #[expect(clippy::vec_init_then_push)]
    const fn one_two_three() -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec
    }

    cement!(pub const ARRAY1: [i32; ?] = one_two_three());
    cement!(static ARRAY2: [i32; ?] = one_two_three());

    cement!(pub(in self) const ARRAY3: [i32; 3] = one_two_three());
    cement!(pub static ARRAY4: [i32; 3] = one_two_three());

    cement!(static ARRAY5: &[i32] = one_two_three());

    dbg!(ARRAY1, ARRAY2, ARRAY3, ARRAY4, ARRAY5);
}
