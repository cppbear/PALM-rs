// Translated from C to Rust. The original C code can be found at
// https://github.com/ulfjack/ryu and carries the following license:
//
// Copyright 2018 Ulf Adams
//
// The contents of this file may be used under the terms of the Apache License,
// Version 2.0.
//
//    (See accompanying file LICENSE-Apache or copy at
//     http://www.apache.org/licenses/LICENSE-2.0)
//
// Alternatively, the contents of this file may be used under the terms of
// the Boost Software License, Version 1.0.
//    (See accompanying file LICENSE-Boost or copy at
//     https://www.boost.org/LICENSE_1_0.txt)
//
// Unless required by applicable law or agreed to in writing, this software
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.

use core::ptr;

#[cfg_attr(feature = "no-panic", inline)]
pub fn div5(x: u64) -> u64 {
    x / 5
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn div10(x: u64) -> u64 {
    x / 10
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn div100(x: u64) -> u64 {
    x / 100
}

#[cfg_attr(feature = "no-panic", inline)]
pub(crate) fn pow5_factor(mut value: u64) -> u32 {
    const M_INV_5: u64 = 14757395258967641293; // 5 * m_inv_5 = 1 (mod 2^64)
    const N_DIV_5: u64 = 3689348814741910323; // #{ n | n = 0 (mod 2^64) } = 2^64 / 5
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        value = value.wrapping_mul(M_INV_5);
        if value > N_DIV_5 {
            break;
        }
        count += 1;
    }
    count
}

// Returns true if value is divisible by 5^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5(value: u64, p: u32) -> bool {
    // I tried a case distinction on p, but there was no performance difference.
    pow5_factor(value) >= p
}

// Returns true if value is divisible by 2^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2(value: u64, p: u32) -> bool {
    debug_assert!(value != 0);
    debug_assert!(p < 64);
    // __builtin_ctzll doesn't appear to be faster here.
    (value & ((1u64 << p) - 1)) == 0
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_shift_64(m: u64, mul: &(u64, u64), j: u32) -> u64 {
    let b0 = m as u128 * mul.0 as u128;
    let b2 = m as u128 * mul.1 as u128;
    (((b0 >> 64) + b2) >> (j - 64)) as u64
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn mul_shift_all_64(
    m: u64,
    mul: &(u64, u64),
    j: u32,
    vp: *mut u64,
    vm: *mut u64,
    mm_shift: u32,
) -> u64 {
    ptr::write(vp, mul_shift_64(4 * m + 2, mul, j));
    ptr::write(vm, mul_shift_64(4 * m - 1 - mm_shift as u64, mul, j));
    mul_shift_64(4 * m, mul, j)
}

#[cfg(test)]
mod tests_llm_16_20 {
    use crate::d2s_intrinsics::div10;

    #[test]
    fn test_div10() {
        assert_eq!(div10(10), 1);
        assert_eq!(div10(25), 2);
        assert_eq!(div10(100), 10);
        assert_eq!(div10(1), 0);
        assert_eq!(div10(0), 0);
        assert_eq!(div10(99), 9);
        assert_eq!(div10(1000), 100);
    }
}

#[cfg(test)]
mod tests_llm_16_21 {
    use crate::d2s_intrinsics::div100;

    #[test]
    fn test_div100() {
        assert_eq!(div100(100), 1);
        assert_eq!(div100(250), 2);
        assert_eq!(div100(99), 0);
        assert_eq!(div100(0), 0);
        assert_eq!(div100(500), 5);
        assert_eq!(div100(10000), 100);
    }
}

#[cfg(test)]
mod tests_llm_16_22 {
    use crate::d2s_intrinsics::div5;

    #[test]
    fn test_div5() {
        assert_eq!(div5(0), 0);
        assert_eq!(div5(5), 1);
        assert_eq!(div5(10), 2);
        assert_eq!(div5(20), 4);
        assert_eq!(div5(25), 5);
        assert_eq!(div5(100), 20);
        assert_eq!(div5(1_000_000), 200_000);
    }
}

#[cfg(test)]
mod tests_llm_16_23 {
    use crate::d2s_intrinsics::mul_shift_64;

    #[test]
    fn test_mul_shift_64() {
        // Test case 1
        let m1: u64 = 2;
        let mul1 = (3, 1);
        let j1: u32 = 64;
        assert_eq!(mul_shift_64(m1, &mul1, j1), 6);

        // Test case 2
        let m2: u64 = 1;
        let mul2 = (4, 2);
        let j2: u32 = 64;
        assert_eq!(mul_shift_64(m2, &mul2, j2), 4);

        // Test case 3
        let m3: u64 = 5;
        let mul3 = (2, 3);
        let j3: u32 = 64;
        assert_eq!(mul_shift_64(m3, &mul3, j3), 10);

        // Test case 4
        let m4: u64 = 10;
        let mul4 = (1, 1);
        let j4: u32 = 65;
        assert_eq!(mul_shift_64(m4, &mul4, j4), 5);

        // Test case 5
        let m5: u64 = 0;
        let mul5 = (10, 5);
        let j5: u32 = 64;
        assert_eq!(mul_shift_64(m5, &mul5, j5), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_26 {
    use super::*;

use crate::*;
    use crate::d2s_intrinsics::multiple_of_power_of_5;

    #[test]
    fn test_multiple_of_power_of_5() {
        assert!(multiple_of_power_of_5(25, 2)); // 25 = 5^2
        assert!(multiple_of_power_of_5(125, 3)); // 125 = 5^3
        assert!(multiple_of_power_of_5(5, 1)); // 5 = 5^1
        assert!(!multiple_of_power_of_5(6, 1)); // 6 is not a multiple of 5
        assert!(multiple_of_power_of_5(0, 0)); // 0 is a multiple of any power
    }
}
