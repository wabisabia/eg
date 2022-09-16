#[allow(unused_imports)]
#[macro_use]
extern crate eg_derive;
#[doc(hidden)]
pub use eg_derive::*;

pub trait Eg {
    fn eg() -> Self;
}

// Primitive types

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

macro_rules! array_impl {
    ($n:expr, $t:ident $($ts:ident)*) => {
        impl<T> Eg for [T; $n] where T: Eg {
            fn eg() -> Self {
                [$t::eg(), $($ts::eg()),*]
            }
        }
        array_impl!{($n - 1), $($ts)*}
    };
    ($n:expr,) => {
        impl<T> Eg for [T; $n] {
            fn eg() -> Self {
                []
            }
        }
    }
}

array_impl! {32, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}

macro_rules! unsigned_int_impl {
    ($($ty:ty),+) => {
        $(
            impl Eg for $ty {
                fn eg() -> Self {
                    42
                }
            }
        )+
    }
}

unsigned_int_impl! {usize, u8, u16, u32, u64, u128}

macro_rules! signed_int_impl {
    ($($ty:ty),+) => {
        $(
            impl Eg for $ty {
                fn eg() -> Self {
                    -1
                }
            }
        )+
    }
}

signed_int_impl! {isize, i8, i16, i32, i64, i128}

macro_rules! float_impl_eg {
    ($($ty:ty),+) => {
        $(
            impl Eg for $ty {
                fn eg() -> Self {
                    3.1415
                }
            }
        )+
    }
}

float_impl_eg! {f32, f64}

impl Eg for bool {
    fn eg() -> Self {
        true
    }
}

impl Eg for char {
    fn eg() -> Self {
        'a'
    }
}

impl Eg for () {
    fn eg() -> Self {
        ()
    }
}

impl<T> Eg for Vec<T>
where
    T: Eg,
{
    fn eg() -> Self {
        vec![T::eg()]
    }
}

macro_rules! containers_from_array_impl_eg {
    ($($ty:ty),+) => {
        $(
            impl<T> Eg for $ty where T: Eg,
            {
                fn eg() -> Self {
                    [T::eg()].into()
                }
            }
        )+
    }
}

containers_from_array_impl_eg! {VecDeque<T>, LinkedList<T>}

macro_rules! containers_from_ord_array_impl_eg {
    ($($ty:ty),+) => {
        $(
            impl <T> Eg for $ty where T: Ord + Eg {
                fn eg() -> Self {
                    [T::eg()].into()
                }
            }
        )+
    }
}

containers_from_ord_array_impl_eg! {BTreeSet<T>, BinaryHeap<T>}

impl<T> Eg for HashSet<T>
where
    T: Eq + Hash + Eg,
{
    fn eg() -> Self {
        [T::eg()].into()
    }
}

impl<K, V> Eg for HashMap<K, V>
where
    K: Eq + Hash + Eg,
    V: Eg,
{
    fn eg() -> Self {
        [(K::eg(), V::eg())].into()
    }
}

impl<K, V> Eg for BTreeMap<K, V>
where
    K: Ord + Eg,
    V: Eg,
{
    fn eg() -> Self {
        [(K::eg(), V::eg())].into()
    }
}

impl Eg for String {
    fn eg() -> Self {
        "foobar".to_string()
    }
}

impl Eg for &str {
    fn eg() -> Self {
        "foobar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_with_defaults() {
        assert_eq!(true, bool::eg());
        assert_eq!('a', char::eg());
        assert_eq!((), <()>::eg());

        assert_eq!(42, usize::eg());
        assert_eq!(42, u8::eg());
        assert_eq!(42, u16::eg());
        assert_eq!(42, u32::eg());
        assert_eq!(42, u64::eg());
        assert_eq!(42, u128::eg());

        assert_eq!(-1, isize::eg());
        assert_eq!(-1, i8::eg());
        assert_eq!(-1, i16::eg());
        assert_eq!(-1, i32::eg());
        assert_eq!(-1, i64::eg());
        assert_eq!(-1, i128::eg());
    }

    #[test]
    fn arrays() {
        let empty: [u8; 0] = [];
        assert_eq!(empty, <[_; 0]>::eg());
        assert_eq!([42], <[u8; 1]>::eg());
        assert_eq!([42, 42], <[u8; 2]>::eg());
        assert_eq!([42, 42, 42], <[u8; 3]>::eg());
        assert_eq!([42, 42, 42, 42], <[u8; 4]>::eg());
        assert_eq!([42, 42, 42, 42, 42], <[u8; 5]>::eg());
        assert_eq!([42, 42, 42, 42, 42, 42], <[u8; 6]>::eg());
        assert_eq!([42, 42, 42, 42, 42, 42, 42], <[u8; 7]>::eg());
        assert_eq!([42, 42, 42, 42, 42, 42, 42, 42], <[u8; 8]>::eg());
        assert_eq!([42, 42, 42, 42, 42, 42, 42, 42, 42], <[u8; 9]>::eg());
        assert_eq!([42, 42, 42, 42, 42, 42, 42, 42, 42, 42], <[u8; 10]>::eg());
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 11]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 12]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 13]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 14]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 15]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 16]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 17]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 18]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 19]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 20]>::eg()
        );
        assert_eq!(
            [42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42],
            <[u8; 21]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42
            ],
            <[u8; 22]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42
            ],
            <[u8; 23]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42
            ],
            <[u8; 24]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42
            ],
            <[u8; 25]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42
            ],
            <[u8; 26]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42
            ],
            <[u8; 27]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42
            ],
            <[u8; 28]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42
            ],
            <[u8; 29]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42
            ],
            <[u8; 30]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42
            ],
            <[u8; 31]>::eg()
        );
        assert_eq!(
            [
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42,
                42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
            ],
            <[u8; 32]>::eg()
        );
    }

    #[test]
    fn containers() {
        assert_eq!(vec![-1], Vec::<_>::eg());
        assert_eq!(VecDeque::from([42]), VecDeque::<u8>::eg());
        assert_eq!(LinkedList::from([42]), LinkedList::<u8>::eg());
        assert_eq!(BTreeSet::from([42]), BTreeSet::<u8>::eg());
        // Cannot compare binary heaps for equality
        // assert_eq!(BinaryHeap::from([0]), BinaryHeap::<u8>::eg());
        assert_eq!(HashMap::from([(42, 42)]), HashMap::<u8, u8>::eg());
        assert_eq!(BTreeMap::from([(42, 42)]), BTreeMap::<u8, u8>::eg());
    }

    #[test]
    fn strings() {
        assert_eq!("foobar", String::eg());
        assert_eq!("foobar", <&str>::eg());
    }
}
