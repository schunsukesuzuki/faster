// This file is part of faster, the SIMD library for humans.
// Copyright 2017 Adam Niederer <adam.niederer@gmail.com>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::std::ops::BitXor;
use crate::intrin::eq::*;
use crate::arch::current::vecs::*;
use crate::vecs::*;


rust_fallback_eq! {
    impl Eq for u8x16 where "__undefined" {
        eq_mask, eq => u8x16, u8, __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_eq! {
    impl Eq for i8x16 where "__undefined" {
        eq_mask, eq => u8x16, u8, __undefined(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    }
}

rust_fallback_eq! {
    impl Eq for u16x8 where "__undefined" {
        eq_mask, eq => u16x8, u16, __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl Eq for i16x8 where "__undefined" {
        eq_mask, eq => u16x8, u16, __undefined(), [0, 1, 2, 3, 4, 5, 6, 7];
    }
}

rust_fallback_eq! {
    impl Eq for u32x4 where "__undefined" {
        eq_mask, eq => u32x4, u32, __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl Eq for i32x4 where "__undefined" {
        eq_mask, eq => u32x4, u32, __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl Eq for f32x4 where "__undefined" {
        eq_mask, eq => u32x4, u32, __undefined(), [0, 1, 2, 3];
    }
}

rust_fallback_eq! {
    impl Eq for f64x2 where "__undefined" {
        eq_mask, eq => u64x2, u64, __undefined(), [0, 1];
    }
}

rust_fallback_eq! {
    impl Eq for u64x2 where "__undefined" {
        eq_mask, eq => u64x2, u64, __undefined(), [0, 1];
    }
}

rust_fallback_eq! {
    impl Eq for i64x2 where "__undefined" {
        eq_mask, eq => u64x2, u64, __undefined(), [0, 1];
    }
}

mod tests {
    use crate::prelude::*;
    use crate::arch::current::vecs::*;

    macro_rules! test_packed_eq {
        ($vec:tt, $el:tt, $mask:tt, $maskel:tt, $name:tt) => {
            #[test]
            fn $name() {
                assert_eq!($vec::halfs(1 as $el, 0 as $el).eq_mask($vec::splat(0 as $el)),
                           $mask::halfs(0, $maskel::max_value()));

                assert_eq!($vec::interleave(1 as $el, 0 as $el).eq_mask($vec::splat(1 as $el)),
                           $mask::interleave($maskel::max_value(), 0));

                assert_eq!($vec::halfs(1 as $el, 0 as $el).ne_mask($vec::splat(0 as $el)),
                           $mask::halfs($maskel::max_value(), 0));

                assert_eq!($vec::interleave(1 as $el, 0 as $el).ne_mask($vec::splat(1 as $el)),
                           $mask::interleave(0, $maskel::max_value()));
            }
        }
    }

    // test_packed_eq!(u8x64, u8, u8x64, u8, test_eq_u8x64);
    // test_packed_eq!(u8x32, u8, u8x32, u8, test_eq_u8x32);
    test_packed_eq!(u8x16, u8, u8x16, u8, test_eq_u8x16);
    // test_packed_eq!(i8x64, i8, u8x64, u8, test_eq_i8x64);
    // test_packed_eq!(i8x32, i8, u8x32, u8, test_eq_i8x32);
    test_packed_eq!(i8x16, i8, u8x16, u8, test_eq_i8x16);
    // test_packed_eq!(u16x32, u16, u16x32, u16, test_eq_u16x32);
    // test_packed_eq!(u16x16, u16, u16x16, u16, test_eq_u16x16);
    test_packed_eq!(u16x8, u16, u16x8, u16, test_eq_u16x8);
    // test_packed_eq!(i16x32, i16, u16x32, u16, test_eq_i16x32);
    // test_packed_eq!(i16x16, i16, u16x16, u16, test_eq_i16x16);
    test_packed_eq!(i16x8, i16, u16x8, u16, test_eq_i16x8);
    // test_packed_eq!(u32x16, u32, u32x16, u32, test_eq_u32x16);
    // test_packed_eq!(u32x8, u32, u32x8, u32, test_eq_u32x8);
    test_packed_eq!(u32x4, u32, u32x4, u32, test_eq_u32x4);
    // test_packed_eq!(i32x16, i32, u32x16, u32, test_eq_i32x16);
    // test_packed_eq!(i32x8, i32, u32x8, u32, test_eq_i32x8);
    test_packed_eq!(i32x4, i32, u32x4, u32, test_eq_i32x4);
    // test_packed_eq!(f32x16, f32, u32x16, u32, test_eq_f32x16);
    // test_packed_eq!(f32x8, f32, u32x8, u32, test_eq_f32x8);
    test_packed_eq!(f32x4, f32, u32x4, u32, test_eq_f32x4);
    // test_packed_eq!(u64x8, u64, u64x8, u64, test_eq_u64x8);
    // test_packed_eq!(u64x4, u64, u64x4, u64, test_eq_u64x4);
    test_packed_eq!(u64x2, u64, u64x2, u64, test_eq_u64x2);
    // test_packed_eq!(i64x8, i64, u64x8, u64, test_eq_i64x8);
    // test_packed_eq!(i64x4, i64, u64x4, u64, test_eq_i64x4);
    test_packed_eq!(i64x2, i64, u64x2, u64, test_eq_i64x2);
    // test_packed_eq!(f64x8, f64, u64x8, u64, test_eq_f64x8);
    // test_packed_eq!(f64x4, f64, u64x4, u64, test_eq_f64x4);
    test_packed_eq!(f64x2, f64, u64x2, u64, test_eq_f64x2);
}
