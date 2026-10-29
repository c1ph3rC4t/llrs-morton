// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Copyright (c) 2026 c1ph3rC4t

//! A morton code lib

const M21: u32 = 0b0000_0000_0001_1111_1111_1111_1111_1111;

mod ffi {
    #[link(name = "morton")]
    unsafe extern "C" {
        pub unsafe fn morton_3d(x: u32, y: u32, z: u32) -> u64;
    }
}

/// Encodes a 3D coordinate into a 64-bit Morton code
///
/// # Panics
/// Will panic if one of `x`, `y` and `z` are above 21 bits
#[must_use]
pub fn morton_3d(x: u32, y: u32, z: u32) -> u64 {
    debug_assert!(x <= M21, "x is too large");
    debug_assert!(y <= M21, "y is too large");
    debug_assert!(z <= M21, "z is too large");
    unsafe { ffi::morton_3d(x, y, z) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weaves() {
        assert_eq!(morton_3d(M21, 0, 0), 0x1249_2492_4924_9249, "x_stripe");
        assert_eq!(morton_3d(0, M21, 0), 0x2492_4924_9249_2492, "y_stripe");
        assert_eq!(morton_3d(0, 0, M21), 0x4924_9249_2492_4924, "z_stripe");
        assert_eq!(morton_3d(M21, M21, 0), 0x36db_6db6_db6d_b6db, "xy_weave");
        assert_eq!(morton_3d(0, M21, M21), 0x6db6_db6d_b6db_6db6, "yz_weave");
        assert_eq!(morton_3d(M21, 0, M21), 0x5b6d_b6db_6db6_db6d, "zx_weave");
        assert_eq!(morton_3d(M21, M21, M21), 0x7fff_ffff_ffff_ffff, "full");
    }

    #[test]
    fn naive() {
        let mut s: u64 = 0x9e37_79b9_7f4a_7c15;
        #[allow(clippy::cast_possible_truncation)]
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            (s as u32) & M21
        };
        let naive_morton = |x: u32, y: u32, z: u32| -> u64 {
            (0..21).fold(0, |acc, i| {
                acc | ((u64::from(x) >> i & 1) << (3 * i))
                    | ((u64::from(y) >> i & 1) << (3 * i + 1))
                    | ((u64::from(z) >> i & 1) << (3 * i + 2))
            })
        };
        for _ in 0..100_000 {
            let (x, y, z) = (next(), next(), next());
            assert_eq!(
                morton_3d(x, y, z),
                naive_morton(x, y, z),
                "{x:#x} {y:#x} {z:#x}"
            );
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "x is too large")]
    fn x_too_large() {
        let _ = morton_3d(M21 + 1, 0, 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "y is too large")]
    fn y_too_large() {
        let _ = morton_3d(0, M21 + 1, 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "z is too large")]
    fn z_too_large() {
        let _ = morton_3d(0, 0, M21 + 1);
    }
}
