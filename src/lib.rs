#![allow(unused_features)]
#![feature(const_heap)]
#![feature(const_clone)]
#![feature(const_trait_impl)]
#![feature(maybe_uninit_array_assume_init)]
#![no_std]

use core::mem::MaybeUninit;

extern crate alloc;
#[cfg(test)]
extern crate std;

#[macro_export]
macro_rules! auto_array {
    ($vis:vis const $name:ident: [$ty:ty; ?] = $collection:expr) => {
        $vis const $name: [$ty; auto_array!(ඞLEN: collection=$collection)] = {
            auto_array!(ඞBODY: ty = $ty, collection = $collection, len = auto_array!(ඞLEN: collection = $collection))
        };
    };
    ($vis:vis static $name:ident: [$ty:ty; ?] = $collection:expr) => {
        $vis static $name: [$ty; auto_array!(ඞLEN: collection = $collection)] = {
            auto_array!(ඞBODY: ty = $ty, collection = $collection, len = auto_array!(ඞLEN: collection = $collection))
        };
    };
    ($vis:vis const $name:ident: [$ty:ty; $len:expr] = $collection:expr) => {
        $vis const $name: [$ty; $len] = {
            auto_array!(ඞBODY: ty = $ty, collection = $collection, len = $len)
        };
    };
    ($vis:vis static $name:ident: [$ty:ty; $len:expr] = $collection:expr) => {
        $vis static $name: [$ty; $len] = {
            auto_array!(ඞBODY: ty = $ty, collection = $collection, len = $len)
        };
    };
    ($vis:vis static $name:ident: &[$ty:ty] = $collection:expr) => {
        $vis static $name: &'static [$ty; auto_array!(ඞLEN: collection = $collection)] = &{
            auto_array!(ඞBODY: ty = $ty, collection = $collection, len = auto_array!(ඞLEN: collection = $collection))
        };
    };
    (ඞLEN: collection = $collection:expr) => {
        {
            let collection = $collection;
            let len = collection.len();
            ::core::mem::forget(collection);
            len
        }
    };
    (ඞBODY: ty = $ty:ty, collection = $collection:expr, len = $len:expr) => {
        {
            const LEN: usize = $len;
            let mut array_uninit: [::core::mem::MaybeUninit<$ty>; LEN] = [::core::mem::MaybeUninit::uninit(); LEN];
            let collection = $collection;
            let mut i: usize = 0;
            while i < LEN {
                array_uninit[i] = ::core::mem::MaybeUninit::new(collection.as_slice()[i]);
                i += 1;
            }
            ::core::mem::forget(collection);
            unsafe { ::core::mem::MaybeUninit::array_assume_init(array_uninit) }
        }
    };
}

#[expect(clippy::len_without_is_empty)]
pub const trait Len {
    fn len(&self) -> usize;
}

pub const trait auto_array {
    fn auto_array<T: [const] Clone, Index, const N: usize>(vec: alloc::vec::Vec<T>) -> [T; N] {
        panic!()
    }
}

const fn auto_array<T: [const] Clone, const N: usize>(vec: alloc::vec::Vec<T>) -> [T; N] {
    if vec.len() != N {
        panic!("Vec's len does not match expected N")
    }
    let mut array = [const { MaybeUninit::uninit() }; N];

    let slice = vec.as_slice();

    let mut i = 0;

    while i < vec.len() {
        array[i] = ::core::mem::MaybeUninit::new(slice[i].clone());
        i += 1;
    }

    ::core::mem::forget(vec);
    unsafe { ::core::mem::MaybeUninit::array_assume_init(array) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use std::dbg;

    #[expect(clippy::vec_init_then_push)]
    const fn v_1_2_3() -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec
    }

    #[expect(clippy::needless_pub_self)]
    #[test]
    fn it_works() {
        auto_array!(pub const ARRAY1: [i32; ?] = v_1_2_3());
        auto_array!(static ARRAY2: [i32; ?] = v_1_2_3());

        auto_array!(pub(in self) const ARRAY3: [i32; 3] = v_1_2_3());
        auto_array!(pub(in super) static ARRAY4: [i32; 3] = v_1_2_3());

        auto_array!(static ARRAY5: &[i32] = v_1_2_3());

        const A1: [i32; 3] = auto_array(v_1_2_3());

        let _ = ARRAY1;

        dbg!(ARRAY1);
        dbg!(ARRAY2);
        dbg!(ARRAY3);
        dbg!(ARRAY4);
        dbg!(ARRAY5);
    }
}
