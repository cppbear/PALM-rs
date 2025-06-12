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

use crate::common::{log10_pow2, log10_pow5, pow5bits};
use crate::f2s_intrinsics::{
    mul_pow5_div_pow2, mul_pow5_inv_div_pow2, multiple_of_power_of_2_32, multiple_of_power_of_5_32,
};

pub const FLOAT_MANTISSA_BITS: u32 = 23;
pub const FLOAT_EXPONENT_BITS: u32 = 8;
const FLOAT_BIAS: i32 = 127;
pub use crate::f2s_intrinsics::{FLOAT_POW5_BITCOUNT, FLOAT_POW5_INV_BITCOUNT};

// A floating decimal representing m * 10^e.
pub struct FloatingDecimal32 {
    pub mantissa: u32,
    // Decimal exponent's range is -45 to 38
    // inclusive, and can fit in i16 if needed.
    pub exponent: i32,
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32 {
    let (e2, m2) = if ieee_exponent == 0 {
        (
            // We subtract 2 so that the bounds computation has 2 additional bits.
            1 - FLOAT_BIAS - FLOAT_MANTISSA_BITS as i32 - 2,
            ieee_mantissa,
        )
    } else {
        (
            ieee_exponent as i32 - FLOAT_BIAS - FLOAT_MANTISSA_BITS as i32 - 2,
            (1u32 << FLOAT_MANTISSA_BITS) | ieee_mantissa,
        )
    };
    let even = (m2 & 1) == 0;
    let accept_bounds = even;

    // Step 2: Determine the interval of valid decimal representations.
    let mv = 4 * m2;
    let mp = 4 * m2 + 2;
    // Implicit bool -> int conversion. True is 1, false is 0.
    let mm_shift = (ieee_mantissa != 0 || ieee_exponent <= 1) as u32;
    let mm = 4 * m2 - 1 - mm_shift;

    // Step 3: Convert to a decimal power base using 64-bit arithmetic.
    let mut vr: u32;
    let mut vp: u32;
    let mut vm: u32;
    let e10: i32;
    let mut vm_is_trailing_zeros = false;
    let mut vr_is_trailing_zeros = false;
    let mut last_removed_digit = 0u8;
    if e2 >= 0 {
        let q = log10_pow2(e2);
        e10 = q as i32;
        let k = FLOAT_POW5_INV_BITCOUNT + pow5bits(q as i32) - 1;
        let i = -e2 + q as i32 + k;
        vr = mul_pow5_inv_div_pow2(mv, q, i);
        vp = mul_pow5_inv_div_pow2(mp, q, i);
        vm = mul_pow5_inv_div_pow2(mm, q, i);
        if q != 0 && (vp - 1) / 10 <= vm / 10 {
            // We need to know one removed digit even if we are not going to loop below. We could use
            // q = X - 1 above, except that would require 33 bits for the result, and we've found that
            // 32-bit arithmetic is faster even on 64-bit machines.
            let l = FLOAT_POW5_INV_BITCOUNT + pow5bits(q as i32 - 1) - 1;
            last_removed_digit =
                (mul_pow5_inv_div_pow2(mv, q - 1, -e2 + q as i32 - 1 + l) % 10) as u8;
        }
        if q <= 9 {
            // The largest power of 5 that fits in 24 bits is 5^10, but q <= 9 seems to be safe as well.
            // Only one of mp, mv, and mm can be a multiple of 5, if any.
            if mv % 5 == 0 {
                vr_is_trailing_zeros = multiple_of_power_of_5_32(mv, q);
            } else if accept_bounds {
                vm_is_trailing_zeros = multiple_of_power_of_5_32(mm, q);
            } else {
                vp -= multiple_of_power_of_5_32(mp, q) as u32;
            }
        }
    } else {
        let q = log10_pow5(-e2);
        e10 = q as i32 + e2;
        let i = -e2 - q as i32;
        let k = pow5bits(i) - FLOAT_POW5_BITCOUNT;
        let mut j = q as i32 - k;
        vr = mul_pow5_div_pow2(mv, i as u32, j);
        vp = mul_pow5_div_pow2(mp, i as u32, j);
        vm = mul_pow5_div_pow2(mm, i as u32, j);
        if q != 0 && (vp - 1) / 10 <= vm / 10 {
            j = q as i32 - 1 - (pow5bits(i + 1) - FLOAT_POW5_BITCOUNT);
            last_removed_digit = (mul_pow5_div_pow2(mv, (i + 1) as u32, j) % 10) as u8;
        }
        if q <= 1 {
            // {vr,vp,vm} is trailing zeros if {mv,mp,mm} has at least q trailing 0 bits.
            // mv = 4 * m2, so it always has at least two trailing 0 bits.
            vr_is_trailing_zeros = true;
            if accept_bounds {
                // mm = mv - 1 - mm_shift, so it has 1 trailing 0 bit iff mm_shift == 1.
                vm_is_trailing_zeros = mm_shift == 1;
            } else {
                // mp = mv + 2, so it always has at least one trailing 0 bit.
                vp -= 1;
            }
        } else if q < 31 {
            // TODO(ulfjack): Use a tighter bound here.
            vr_is_trailing_zeros = multiple_of_power_of_2_32(mv, q - 1);
        }
    }

    // Step 4: Find the shortest decimal representation in the interval of valid representations.
    let mut removed = 0i32;
    let output = if vm_is_trailing_zeros || vr_is_trailing_zeros {
        // General case, which happens rarely (~4.0%).
        while vp / 10 > vm / 10 {
            vm_is_trailing_zeros &= vm - (vm / 10) * 10 == 0;
            vr_is_trailing_zeros &= last_removed_digit == 0;
            last_removed_digit = (vr % 10) as u8;
            vr /= 10;
            vp /= 10;
            vm /= 10;
            removed += 1;
        }
        if vm_is_trailing_zeros {
            while vm % 10 == 0 {
                vr_is_trailing_zeros &= last_removed_digit == 0;
                last_removed_digit = (vr % 10) as u8;
                vr /= 10;
                vp /= 10;
                vm /= 10;
                removed += 1;
            }
        }
        if vr_is_trailing_zeros && last_removed_digit == 5 && vr % 2 == 0 {
            // Round even if the exact number is .....50..0.
            last_removed_digit = 4;
        }
        // We need to take vr + 1 if vr is outside bounds or we need to round up.
        vr + ((vr == vm && (!accept_bounds || !vm_is_trailing_zeros)) || last_removed_digit >= 5)
            as u32
    } else {
        // Specialized for the common case (~96.0%). Percentages below are relative to this.
        // Loop iterations below (approximately):
        // 0: 13.6%, 1: 70.7%, 2: 14.1%, 3: 1.39%, 4: 0.14%, 5+: 0.01%
        while vp / 10 > vm / 10 {
            last_removed_digit = (vr % 10) as u8;
            vr /= 10;
            vp /= 10;
            vm /= 10;
            removed += 1;
        }
        // We need to take vr + 1 if vr is outside bounds or we need to round up.
        vr + (vr == vm || last_removed_digit >= 5) as u32
    };
    let exp = e10 + removed;

    FloatingDecimal32 {
        exponent: exp,
        mantissa: output,
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut u32_0: u32 = 3641u32;
    let mut u32_1: u32 = 5888u32;
    let mut i32_0: i32 = -10201i32;
    let mut u32_2: u32 = 9636u32;
    let mut u32_3: u32 = 8181u32;
    let mut u64_0: u64 = 3527u64;
    let mut u32_4: u32 = 5418u32;
    let mut u32_5: u32 = 7586u32;
    let mut i32_1: i32 = -20483i32;
    let mut u32_6: u32 = 7706u32;
    let mut u32_7: u32 = 807u32;
    let mut i32_2: i32 = -9769i32;
    let mut u64_1: u64 = 9338u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_8: u32 = 3875u32;
    let mut u64_2: u64 = 99u64;
    let mut u64_3: u64 = 9539u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_4: u64 = 740u64;
    let mut i32_3: i32 = -381i32;
    let mut u32_9: u32 = crate::common::log10_pow2(i32_3);
    let mut u64_5: u64 = crate::d2s_intrinsics::mul_shift_64(u64_4, tuple_0_ref_0, u32_8);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_1, exponent: i32_2};
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_7, u32_6, i32_1);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_5, u32_4);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_0, u32_3);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_2, exponent: i32_0};
    let mut bool_2: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_1, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_0: i32 = 7971i32;
    let mut u32_0: u32 = 1156u32;
    let mut u64_0: u64 = 1508u64;
    let mut u64_1: u64 = 4233u64;
    let mut i32_1: i32 = -4053i32;
    let mut u32_1: u32 = 3003u32;
    let mut u32_2: u32 = 9549u32;
    let mut u64_2: u64 = 1592u64;
    let mut u32_3: u32 = 8761u32;
    let mut u32_4: u32 = 2405u32;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut u32_5: u32 = 7316u32;
    let mut u32_6: u32 = 3875u32;
    let mut i32_2: i32 = 7562i32;
    let mut u32_7: u32 = crate::common::log10_pow5(i32_2);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_6, u32_5);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut buffer_2_ref_0: &crate::buffer::Buffer = &mut buffer_2;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_2_ref_0);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_4, u32_3);
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u64_3: u64 = crate::d2s_intrinsics::div10(u64_2);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_2, u32_1, i32_1);
    let mut u32_9: u32 = crate::d2s::decimal_length17(u64_1);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_0, u32_0);
    let mut i32_3: i32 = crate::common::pow5bits(i32_0);
    let mut buffer_5: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut u64_0: u64 = 6668u64;
    let mut u32_0: u32 = 7996u32;
    let mut i32_0: i32 = -18314i32;
    let mut u32_1: u32 = 2248u32;
    let mut u32_2: u32 = 6796u32;
    let mut u32_3: u32 = 8339u32;
    let mut u64_1: u64 = 2778u64;
    let mut u64_2: u64 = 6177u64;
    let mut i32_1: i32 = -1934i32;
    let mut u32_4: u32 = 1882u32;
    let mut u64_3: u64 = 6587u64;
    let mut u32_5: u32 = 9116u32;
    let mut u64_4: u64 = 9131u64;
    let mut u32_6: u32 = 7491u32;
    let mut u64_5: u64 = 1847u64;
    let mut u32_7: u32 = 6584u32;
    let mut u64_6: u64 = 4329u64;
    let mut u32_8: u32 = 2786u32;
    let mut u64_7: u64 = 8951u64;
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_7, u32_8);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_6, u32_7);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_5, u32_6);
    let mut bool_3: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_4, u32_5);
    let mut bool_4: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_3, u32_4);
    let mut i32_2: i32 = crate::common::pow5bits(i32_1);
    let mut u64_8: u64 = crate::d2s_intrinsics::div100(u64_2);
    let mut bool_5: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_1, u32_3);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_2, u32_1, i32_0);
    let mut u32_10: u32 = crate::common::decimal_length9(u32_0);
    let mut u32_11: u32 = crate::d2s::decimal_length17(u64_0);
    panic!("From RustyUnit with love");
}
}