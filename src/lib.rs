#![allow(unused_features)]
#![feature(const_heap)]
#![feature(const_trait_impl)]
#![feature(maybe_uninit_array_assume_init)]

#[macro_export]
macro_rules! cement {
    ($vis:vis const $name:ident: [$ty:ty; ?] = $collection:expr) => {
        $vis const $name: [$ty; cement!(ඞLEN: $collection)] = {
            cement!(ඞBODY: $ty, $collection, cement!(ඞLEN: $collection))
        };
    };
    ($vis:vis static $name:ident: [$ty:ty; ?] = $collection:expr) => {
        $vis static $name: [$ty; cement!(ඞLEN: $collection)] = {
            cement!(ඞBODY: $ty, $collection, cement!(ඞLEN: $collection))
        };
    };
    ($vis:vis const $name:ident: [$ty:ty; $len:expr] = $collection:expr) => {
        $vis const $name: [$ty; $len] = {
            cement!(ඞBODY: $ty, $collection, $len)
        };
    };
    ($vis:vis static $name:ident: [$ty:ty; $len:expr] = $collection:expr) => {
        $vis static $name: [$ty; $len] = {
            cement!(ඞBODY: $ty, $collection, $len)
        };
    };
    (ඞLEN: $collection:expr) => {
        {
            let collection = $collection;
            let len = collection.len();
            ::core::mem::forget(collection);
            len
        }
    };
    (ඞBODY: $ty:ty, $collection:expr ,$len:expr) => {
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

#[cfg(test)]
mod tests {

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
        cement!(pub const ARRAY1: [i32; ?] = v_1_2_3());
        cement!(static ARRAY2: [i32; ?] = v_1_2_3());

        cement!(pub(in self) const ARRAY3: [i32; 3] = v_1_2_3());
        cement!(pub(in super) static ARRAY4: [i32; 3] = v_1_2_3());

        let _ = ARRAY1;

        dbg!(ARRAY1);
        dbg!(ARRAY2);
        dbg!(ARRAY3);
        dbg!(ARRAY4);
    }
}
