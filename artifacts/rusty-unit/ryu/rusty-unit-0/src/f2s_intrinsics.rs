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

use crate::d2s;

pub const FLOAT_POW5_INV_BITCOUNT: i32 = d2s::DOUBLE_POW5_INV_BITCOUNT - 64;
pub const FLOAT_POW5_BITCOUNT: i32 = d2s::DOUBLE_POW5_BITCOUNT - 64;

#[cfg_attr(feature = "no-panic", inline)]
fn pow5factor_32(mut value: u32) -> u32 {
    let mut count = 0u32;
    loop {
        debug_assert!(value != 0);
        let q = value / 5;
        let r = value % 5;
        if r != 0 {
            break;
        }
        value = q;
        count += 1;
    }
    count
}

// Returns true if value is divisible by 5^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool {
    pow5factor_32(value) >= p
}

// Returns true if value is divisible by 2^p.
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    // __builtin_ctz doesn't appear to be faster here.
    (value & ((1u32 << p) - 1)) == 0
}

// It seems to be slightly faster to avoid uint128_t here, although the
// generated code for uint128_t looks slightly nicer.
#[cfg_attr(feature = "no-panic", inline)]
fn mul_shift_32(m: u32, factor: u64, shift: i32) -> u32 {
    debug_assert!(shift > 32);

    // The casts here help MSVC to avoid calls to the __allmul library
    // function.
    let factor_lo = factor as u32;
    let factor_hi = (factor >> 32) as u32;
    let bits0 = m as u64 * factor_lo as u64;
    let bits1 = m as u64 * factor_hi as u64;

    let sum = (bits0 >> 32) + bits1;
    let shifted_sum = sum >> (shift - 32);
    debug_assert!(shifted_sum <= u32::max_value() as u64);
    shifted_sum as u32
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        // The inverse multipliers are defined as [2^x / 5^y] + 1; the upper 64
        // bits from the double lookup table are the correct bits for [2^x /
        // 5^y], so we have to add 1 here. Note that we rely on the fact that
        // the added 1 that's already stored in the table never overflows into
        // the upper 64 bits.
        let pow5 = unsafe { d2s::compute_inv_pow5(q) };
        mul_shift_32(m, pow5.1 + 1, j)
    }

    #[cfg(not(feature = "small"))]
    {
        debug_assert!(q < d2s::DOUBLE_POW5_INV_SPLIT.len() as u32);
        unsafe {
            mul_shift_32(
                m,
                d2s::DOUBLE_POW5_INV_SPLIT.get_unchecked(q as usize).1 + 1,
                j,
            )
        }
    }
}

#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_div_pow2(m: u32, i: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
        let pow5 = unsafe { d2s::compute_pow5(i) };
        mul_shift_32(m, pow5.1, j)
    }

    #[cfg(not(feature = "small"))]
    {
        debug_assert!(i < d2s::DOUBLE_POW5_SPLIT.len() as u32);
        unsafe { mul_shift_32(m, d2s::DOUBLE_POW5_SPLIT.get_unchecked(i as usize).1, j) }
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
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut i32_0: i32 = 5062i32;
    let mut u32_0: u32 = 8086u32;
    let mut u32_1: u32 = 9840u32;
    let mut i32_1: i32 = 8139i32;
    let mut u32_2: u32 = 9544u32;
    let mut u32_3: u32 = 9694u32;
    let mut i32_2: i32 = -3146i32;
    let mut u32_4: u32 = 2905u32;
    let mut u32_5: u32 = 4435u32;
    let mut i32_3: i32 = -6644i32;
    let mut i32_4: i32 = -16181i32;
    let mut u32_6: u32 = 5263u32;
    let mut u32_7: u32 = 8554u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_5: i32 = 8734i32;
    let mut u32_8: u32 = 8230u32;
    let mut i32_6: i32 = 18623i32;
    let mut u32_9: u32 = 5148u32;
    let mut u32_10: u32 = 3587u32;
    let mut u32_11: u32 = 1807u32;
    let mut u64_0: u64 = 1495u64;
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_0, u32_11);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_10, u32_9);
    let mut i32_7: i32 = crate::common::ceil_log2_pow5(i32_6);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_8, exponent: i32_5};
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_7, u32_6, i32_4);
    let mut i32_8: i32 = crate::common::log2_pow5(i32_3);
    let mut u32_13: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_5, u32_4, i32_2);
    let mut bool_2: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_3, u32_2);
    let mut i32_9: i32 = crate::common::pow5bits(i32_1);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut i32_0: i32 = 15118i32;
    let mut u64_0: u64 = 3584u64;
    let mut u32_0: u32 = 5129u32;
    let mut u64_1: u64 = 1783u64;
    let mut u64_2: u64 = 5675u64;
    let mut i32_1: i32 = 530i32;
    let mut u32_1: u32 = 9119u32;
    let mut u64_3: u64 = 7540u64;
    let mut u64_4: u64 = 1283u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_5: u64 = 3648u64;
    let mut u32_2: u32 = 7128u32;
    let mut i32_2: i32 = -3094i32;
    let mut u32_3: u32 = 8301u32;
    let mut i32_3: i32 = 11835i32;
    let mut u64_6: u64 = 6843u64;
    let mut i32_4: i32 = 4469i32;
    let mut u32_4: u32 = 1135u32;
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_4, exponent: i32_4};
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_6, exponent: i32_3};
    let mut floatingdecimal32_1: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_3, exponent: i32_2};
    let mut u32_5: u32 = crate::f2s_intrinsics::pow5factor_32(u32_2);
    let mut u64_7: u64 = crate::d2s_intrinsics::mul_shift_64(u64_5, tuple_0_ref_0, u32_1);
    let mut i32_5: i32 = crate::common::log2_pow5(i32_1);
    let mut u64_8: u64 = crate::d2s_intrinsics::div5(u64_2);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_0);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_0, exponent: i32_0};
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut i32_0: i32 = 8767i32;
    let mut i32_1: i32 = -8716i32;
    let mut u64_0: u64 = 983u64;
    let mut u32_0: u32 = 2244u32;
    let mut u32_1: u32 = 8844u32;
    let mut i32_2: i32 = -18335i32;
    let mut i32_3: i32 = -4549i32;
    let mut u32_2: u32 = 3616u32;
    let mut u32_3: u32 = 5411u32;
    let mut u64_1: u64 = 1304u64;
    let mut i32_4: i32 = -5174i32;
    let mut u32_4: u32 = 2660u32;
    let mut i32_5: i32 = -1993i32;
    let mut u64_2: u64 = 3938u64;
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_2, exponent: i32_5};
    let mut u32_5: u32 = crate::f2s_intrinsics::pow5factor_32(u32_4);
    let mut i32_6: i32 = crate::common::log2_pow5(i32_4);
    let mut u32_6: u32 = crate::d2s::decimal_length17(u64_1);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_3, u32_2);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut i32_7: i32 = crate::common::ceil_log2_pow5(i32_3);
    let mut u32_7: u32 = crate::common::log10_pow2(i32_2);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_1, u32_0);
    let mut u32_8: u32 = crate::d2s::decimal_length17(u64_0);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut i32_8: i32 = crate::common::log2_pow5(i32_1);
    let mut u32_9: u32 = crate::common::log10_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut i32_0: i32 = 3217i32;
    let mut i32_1: i32 = -859i32;
    let mut u64_0: u64 = 7896u64;
    let mut u32_0: u32 = 45u32;
    let mut u64_1: u64 = 5875u64;
    let mut u64_2: u64 = 4540u64;
    let mut i32_2: i32 = 2081i32;
    let mut u32_1: u32 = 2648u32;
    let mut u32_2: u32 = 6419u32;
    let mut i32_3: i32 = -11325i32;
    let mut u32_3: u32 = 5426u32;
    let mut i32_4: i32 = -11151i32;
    let mut u32_4: u32 = 5547u32;
    let mut u32_5: u32 = 7911u32;
    let mut u32_6: u32 = 9311u32;
    let mut u32_7: u32 = 1171u32;
    let mut i32_5: i32 = 18410i32;
    let mut i32_6: i32 = 11707i32;
    let mut u64_3: u64 = 160u64;
    let mut u32_8: u32 = 5175u32;
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_shift_32(u32_8, u64_3, i32_6);
    let mut i32_7: i32 = crate::common::pow5bits(i32_5);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_7, u32_6);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_5, u32_4, i32_4);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_3, exponent: i32_3};
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_2, u32_1, i32_2);
    let mut u64_4: u64 = crate::d2s_intrinsics::div5(u64_2);
    let mut u32_12: u32 = crate::d2s::decimal_length17(u64_1);
    let mut u32_13: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_0, i32_1);
    let mut i32_8: i32 = crate::common::ceil_log2_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut i32_0: i32 = -5770i32;
    let mut u32_0: u32 = 7671u32;
    let mut u32_1: u32 = 3356u32;
    let mut u32_2: u32 = 1122u32;
    let mut u64_0: u64 = 8589u64;
    let mut u64_1: u64 = 7732u64;
    let mut u64_2: u64 = 3574u64;
    let mut u32_3: u32 = 2481u32;
    let mut u32_4: u32 = 1635u32;
    let mut i32_1: i32 = 24888i32;
    let mut i32_2: i32 = 1607i32;
    let mut u64_3: u64 = 3981u64;
    let mut i32_3: i32 = -6345i32;
    let mut u32_5: u32 = 7546u32;
    let mut i32_4: i32 = -4519i32;
    let mut u64_4: u64 = 2061u64;
    let mut u32_6: u32 = 4790u32;
    let mut i32_5: i32 = 14140i32;
    let mut i32_6: i32 = crate::common::pow5bits(i32_5);
    let mut u32_7: u32 = crate::f2s_intrinsics::mul_shift_32(u32_6, u64_4, i32_4);
    let mut u32_8: u32 = crate::common::decimal_length9(u32_5);
    let mut i32_7: i32 = crate::common::log2_pow5(i32_3);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_3, exponent: i32_2};
    let mut u32_9: u32 = crate::common::log10_pow2(i32_1);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_4, u32_3);
    let mut u64_5: u64 = crate::d2s_intrinsics::div5(u64_2);
    let mut u64_6: u64 = crate::d2s_intrinsics::div10(u64_1);
    let mut u32_10: u32 = crate::d2s::decimal_length17(u64_0);
    let mut u32_11: u32 = crate::f2s_intrinsics::pow5factor_32(u32_2);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_1, u32_0);
    let mut i32_8: i32 = crate::common::log2_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut u32_0: u32 = 7376u32;
    let mut u64_0: u64 = 1156u64;
    let mut u64_1: u64 = 1758u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 5044u64;
    let mut i32_0: i32 = 3058i32;
    let mut i32_1: i32 = 9772i32;
    let mut u32_1: u32 = 6677u32;
    let mut i32_2: i32 = 5510i32;
    let mut u64_3: u64 = 3948u64;
    let mut u32_2: u32 = 4046u32;
    let mut i32_3: i32 = -3258i32;
    let mut u32_3: u32 = 6807u32;
    let mut u64_4: u64 = 7613u64;
    let mut u64_5: u64 = 747u64;
    let mut tuple_1: (u64, u64) = (u64_5, u64_4);
    let mut tuple_1_ref_0: &(u64, u64) = &mut tuple_1;
    let mut u64_6: u64 = 3999u64;
    let mut i32_4: i32 = 12965i32;
    let mut u64_7: u64 = 2270u64;
    let mut u32_4: u32 = 5708u32;
    let mut i32_5: i32 = -1892i32;
    let mut i32_6: i32 = crate::common::log2_pow5(i32_5);
    let mut u32_5: u32 = crate::f2s_intrinsics::mul_shift_32(u32_4, u64_7, i32_4);
    let mut u64_8: u64 = crate::d2s_intrinsics::mul_shift_64(u64_6, tuple_1_ref_0, u32_3);
    let mut i32_7: i32 = crate::common::log2_pow5(i32_3);
    let mut u32_6: u32 = crate::common::decimal_length9(u32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_3, exponent: i32_2};
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_1, exponent: i32_1};
    let mut u32_7: u32 = crate::common::log10_pow2(i32_0);
    let mut u64_9: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u32_0: u32 = 6388u32;
    let mut u64_0: u64 = 8531u64;
    let mut i32_0: i32 = 3580i32;
    let mut u32_1: u32 = 4408u32;
    let mut u32_2: u32 = 9795u32;
    let mut i32_1: i32 = 3933i32;
    let mut u32_3: u32 = 4363u32;
    let mut u32_4: u32 = 3989u32;
    let mut u32_5: u32 = 3959u32;
    let mut u32_6: u32 = 3400u32;
    let mut u32_7: u32 = 9641u32;
    let mut u32_8: u32 = 664u32;
    let mut u64_1: u64 = 6144u64;
    let mut i32_2: i32 = 5038i32;
    let mut u32_9: u32 = 2457u32;
    let mut u32_10: u32 = 2260u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_10, u32_9, i32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_1, u32_8);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_7, u32_6);
    let mut u32_12: u32 = crate::f2s_intrinsics::pow5factor_32(u32_5);
    let mut u32_13: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_4, u32_3, i32_1);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_2, u32_1, i32_0);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut i32_0: i32 = 4803i32;
    let mut i32_1: i32 = -15369i32;
    let mut u32_0: u32 = 5418u32;
    let mut u64_0: u64 = 9180u64;
    let mut i32_2: i32 = -20901i32;
    let mut u32_1: u32 = 5528u32;
    let mut u32_2: u32 = 228u32;
    let mut i32_3: i32 = 19470i32;
    let mut u64_1: u64 = 4612u64;
    let mut u64_2: u64 = 1080u64;
    let mut u32_3: u32 = 5727u32;
    let mut u64_3: u64 = 6844u64;
    let mut u64_4: u64 = 4423u64;
    let mut u32_4: u32 = 3421u32;
    let mut u64_5: u64 = 7579u64;
    let mut i32_4: i32 = 16798i32;
    let mut u32_5: u32 = crate::common::log10_pow5(i32_4);
    let mut u64_6: u64 = crate::d2s_intrinsics::div10(u64_5);
    let mut u32_6: u32 = crate::f2s_intrinsics::pow5factor_32(u32_4);
    let mut u64_7: u64 = crate::d2s_intrinsics::div5(u64_4);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_3, u32_3);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u64_8: u64 = crate::d2s_intrinsics::div10(u64_2);
    let mut u64_9: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut u32_7: u32 = crate::common::log10_pow5(i32_3);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_2, u32_1, i32_2);
    let mut u64_10: u64 = crate::d2s_intrinsics::div100(u64_0);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_0, exponent: i32_1};
    let mut i32_5: i32 = crate::common::ceil_log2_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut i32_0: i32 = -6442i32;
    let mut u32_0: u32 = 4779u32;
    let mut u64_0: u64 = 8732u64;
    let mut i32_1: i32 = -9929i32;
    let mut i32_2: i32 = 7539i32;
    let mut u32_1: u32 = 3714u32;
    let mut u32_2: u32 = 191u32;
    let mut u64_1: u64 = 7954u64;
    let mut u32_3: u32 = 8362u32;
    let mut u32_4: u32 = 4950u32;
    let mut u64_2: u64 = 6441u64;
    let mut u32_5: u32 = 2190u32;
    let mut u64_3: u64 = 6706u64;
    let mut u64_4: u64 = 9737u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_5: u64 = 6477u64;
    let mut u32_6: u32 = 1792u32;
    let mut u32_7: u32 = 5125u32;
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_7, u32_6);
    let mut u64_6: u64 = crate::d2s_intrinsics::mul_shift_64(u64_5, tuple_0_ref_0, u32_5);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_2, u32_4);
    let mut u32_8: u32 = crate::f2s_intrinsics::pow5factor_32(u32_3);
    let mut u32_9: u32 = crate::d2s::decimal_length17(u64_1);
    let mut bool_2: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_2, u32_1);
    let mut u32_10: u32 = crate::common::log10_pow5(i32_2);
    let mut u32_11: u32 = crate::common::log10_pow5(i32_1);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_0, u32_0);
    let mut i32_3: i32 = crate::common::ceil_log2_pow5(i32_0);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut i32_0: i32 = 7399i32;
    let mut u64_0: u64 = 1917u64;
    let mut u32_0: u32 = 4428u32;
    let mut u32_1: u32 = 1686u32;
    let mut u32_2: u32 = 5031u32;
    let mut u32_3: u32 = 1651u32;
    let mut u32_4: u32 = 9643u32;
    let mut u64_1: u64 = 3058u64;
    let mut u32_5: u32 = 3015u32;
    let mut u32_6: u32 = 6872u32;
    let mut i32_1: i32 = 424i32;
    let mut u64_2: u64 = 3601u64;
    let mut u32_7: u32 = 9439u32;
    let mut u32_8: u32 = 1804u32;
    let mut u64_3: u64 = 7949u64;
    let mut u64_4: u64 = 4387u64;
    let mut u32_9: u32 = 4365u32;
    let mut u32_10: u32 = 7354u32;
    let mut u32_11: u32 = 6597u32;
    let mut u32_12: u32 = 7680u32;
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_12, u32_11);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_10, u32_9);
    let mut u32_13: u32 = crate::d2s::decimal_length17(u64_4);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_3, u32_8);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_shift_32(u32_7, u64_2, i32_1);
    let mut floatingdecimal32_1: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_6, u32_5);
    let mut u32_15: u32 = crate::d2s::decimal_length17(u64_1);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut floatingdecimal32_2: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_4, u32_3);
    let mut floatingdecimal32_3: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_2, u32_1);
    let mut u32_16: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut u32_0: u32 = 3828u32;
    let mut u64_0: u64 = 9288u64;
    let mut u64_1: u64 = 4395u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 785u64;
    let mut u32_1: u32 = 7448u32;
    let mut u64_3: u64 = 1563u64;
    let mut u64_4: u64 = 2476u64;
    let mut u32_2: u32 = 6922u32;
    let mut u64_5: u64 = 4229u64;
    let mut i32_0: i32 = 5298i32;
    let mut u64_6: u64 = 5731u64;
    let mut u32_3: u32 = 1713u32;
    let mut u64_7: u64 = 9265u64;
    let mut i32_1: i32 = -4278i32;
    let mut u32_4: u32 = 2158u32;
    let mut u32_5: u32 = 2109u32;
    let mut u64_8: u64 = 543u64;
    let mut u32_6: u32 = 1850u32;
    let mut u64_9: u64 = 1715u64;
    let mut u64_10: u64 = crate::d2s_intrinsics::div100(u64_9);
    let mut u32_7: u32 = crate::common::decimal_length9(u32_6);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_8, u32_5);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_4, exponent: i32_1};
    let mut u64_11: u64 = crate::d2s_intrinsics::div5(u64_7);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_shift_32(u32_3, u64_6, i32_0);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_5, u32_2);
    let mut u64_12: u64 = crate::d2s_intrinsics::div10(u64_4);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_3, u32_1);
    let mut u64_13: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut u64_0: u64 = 2934u64;
    let mut i32_0: i32 = -6910i32;
    let mut u64_1: u64 = 2581u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_0: u32 = 5414u32;
    let mut u64_2: u64 = 8006u64;
    let mut i32_1: i32 = -1883i32;
    let mut u32_1: u32 = 2324u32;
    let mut i32_2: i32 = 2916i32;
    let mut u32_2: u32 = 1882u32;
    let mut u32_3: u32 = 8471u32;
    let mut i32_3: i32 = 10859i32;
    let mut u32_4: u32 = 1903u32;
    let mut u32_5: u32 = 9105u32;
    let mut u64_3: u64 = 2391u64;
    let mut i32_4: i32 = 708i32;
    let mut u64_4: u64 = 327u64;
    let mut u32_6: u32 = 6355u32;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_7: u32 = crate::f2s_intrinsics::mul_shift_32(u32_6, u64_4, i32_4);
    let mut u64_5: u64 = crate::d2s_intrinsics::div5(u64_3);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_5, u32_4, i32_3);
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_3, u32_2, i32_2);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_1, exponent: i32_1};
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_2, u32_0);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_10: u32 = crate::d2s::decimal_length17(u64_1);
    let mut i32_5: i32 = crate::common::ceil_log2_pow5(i32_0);
    let mut u64_6: u64 = crate::d2s_intrinsics::div100(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u64_0: u64 = 9530u64;
    let mut u32_0: u32 = 6691u32;
    let mut u32_1: u32 = 3995u32;
    let mut i32_0: i32 = 3270i32;
    let mut u32_2: u32 = 1710u32;
    let mut u32_3: u32 = 7656u32;
    let mut i32_1: i32 = -1710i32;
    let mut u32_4: u32 = 9771u32;
    let mut u32_5: u32 = 1165u32;
    let mut u32_6: u32 = 5747u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_2: i32 = -5393i32;
    let mut u64_1: u64 = 3451u64;
    let mut u32_7: u32 = 4069u32;
    let mut i32_3: i32 = 3202i32;
    let mut u64_2: u64 = 7076u64;
    let mut u64_3: u64 = 3119u64;
    let mut u64_4: u64 = crate::d2s_intrinsics::div100(u64_3);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_2, exponent: i32_3};
    let mut u32_8: u32 = crate::f2s_intrinsics::pow5factor_32(u32_7);
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_1, exponent: i32_2};
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_9: u32 = crate::common::decimal_length9(u32_6);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_5, u32_4, i32_1);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_3, u32_2, i32_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_1, u32_0);
    let mut u64_5: u64 = crate::d2s_intrinsics::div100(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut i32_0: i32 = -3958i32;
    let mut u64_0: u64 = 2634u64;
    let mut u32_0: u32 = 5261u32;
    let mut u32_1: u32 = 9647u32;
    let mut u64_1: u64 = 8210u64;
    let mut u64_2: u64 = 2371u64;
    let mut u64_3: u64 = 4862u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_2: u32 = 7155u32;
    let mut u64_4: u64 = 7045u64;
    let mut u64_5: u64 = 6677u64;
    let mut i32_1: i32 = 2046i32;
    let mut u32_3: u32 = 1657u32;
    let mut u32_4: u32 = 5789u32;
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_4, u32_3);
    let mut i32_2: i32 = crate::common::ceil_log2_pow5(i32_1);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u64_6: u64 = crate::d2s_intrinsics::div100(u64_5);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_4, u32_2);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_5: u32 = crate::d2s::decimal_length17(u64_3);
    let mut u64_7: u64 = crate::d2s_intrinsics::div10(u64_2);
    let mut buffer_4_ref_0: &crate::buffer::Buffer = &mut buffer_4;
    let mut buffer_5: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_4_ref_0);
    let mut buffer_6: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_1);
    let mut u32_6: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut u32_0: u32 = 6664u32;
    let mut u64_0: u64 = 4397u64;
    let mut u64_1: u64 = 3704u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 8198u64;
    let mut i32_0: i32 = -2812i32;
    let mut u64_3: u64 = 2754u64;
    let mut u32_1: u32 = 2380u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u64_4: u64 = 6263u64;
    let mut u64_5: u64 = 1842u64;
    let mut i32_1: i32 = 7782i32;
    let mut u32_2: u32 = 3170u32;
    let mut u32_3: u32 = 3997u32;
    let mut u32_4: u32 = 8533u32;
    let mut u32_5: u32 = 6092u32;
    let mut i32_2: i32 = -20667i32;
    let mut u64_6: u64 = 3994u64;
    let mut u32_6: u32 = 1245u32;
    let mut u32_7: u32 = crate::f2s_intrinsics::mul_shift_32(u32_6, u64_6, i32_2);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_5, u32_4);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_3, u32_2);
    let mut u32_8: u32 = crate::common::log10_pow2(i32_1);
    let mut u32_9: u32 = crate::d2s::decimal_length17(u64_5);
    let mut u64_7: u64 = crate::d2s_intrinsics::div100(u64_4);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_shift_32(u32_1, u64_3, i32_0);
    let mut u64_8: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut u64_0: u64 = 9591u64;
    let mut u32_0: u32 = 1658u32;
    let mut u64_1: u64 = 6528u64;
    let mut i32_0: i32 = -21477i32;
    let mut u32_1: u32 = 8764u32;
    let mut u64_2: u64 = 720u64;
    let mut u32_2: u32 = 8332u32;
    let mut u32_3: u32 = 4788u32;
    let mut i32_1: i32 = 3267i32;
    let mut i32_2: i32 = -17814i32;
    let mut u32_4: u32 = 6182u32;
    let mut u32_5: u32 = 3664u32;
    let mut u32_6: u32 = 6021u32;
    let mut i32_3: i32 = 10770i32;
    let mut u64_3: u64 = 7056u64;
    let mut u32_7: u32 = 6072u32;
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_shift_32(u32_7, u64_3, i32_3);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_6, u32_5);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_9: u32 = crate::f2s_intrinsics::pow5factor_32(u32_4);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_10: u32 = crate::common::log10_pow2(i32_2);
    let mut i32_4: i32 = crate::common::pow5bits(i32_1);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_3, u32_2);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_2, u32_1);
    let mut u32_11: u32 = crate::common::log10_pow2(i32_0);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut bool_3: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_0);
    let mut u64_4: u64 = crate::d2s_intrinsics::div5(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut u32_0: u32 = 5879u32;
    let mut u64_0: u64 = 3473u64;
    let mut u64_1: u64 = 5908u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 454u64;
    let mut i32_0: i32 = 3540i32;
    let mut i32_1: i32 = 2579i32;
    let mut u64_3: u64 = 3246u64;
    let mut u32_1: u32 = 5499u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_2: i32 = -4223i32;
    let mut u64_4: u64 = 6429u64;
    let mut u32_2: u32 = 7382u32;
    let mut u64_5: u64 = 6708u64;
    let mut u32_3: u32 = 7933u32;
    let mut u32_4: u32 = 5304u32;
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_4, u32_3);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_5, u32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_4, exponent: i32_2};
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_5: u32 = crate::f2s_intrinsics::pow5factor_32(u32_1);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_6: u32 = crate::d2s::decimal_length17(u64_3);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u32_7: u32 = crate::common::log10_pow5(i32_1);
    let mut i32_3: i32 = crate::common::pow5bits(i32_0);
    let mut buffer_3_ref_0: &crate::buffer::Buffer = &mut buffer_3;
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_3_ref_0);
    let mut u64_6: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut i32_0: i32 = 11493i32;
    let mut u32_0: u32 = 7133u32;
    let mut u32_1: u32 = 9185u32;
    let mut i32_1: i32 = -12594i32;
    let mut u32_2: u32 = 2198u32;
    let mut u32_3: u32 = 8316u32;
    let mut u32_4: u32 = 8919u32;
    let mut u32_5: u32 = 1816u32;
    let mut u32_6: u32 = 8121u32;
    let mut u32_7: u32 = 8079u32;
    let mut u64_0: u64 = 3937u64;
    let mut u64_1: u64 = 8744u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 6466u64;
    let mut u32_8: u32 = 8550u32;
    let mut u32_9: u32 = 2046u32;
    let mut u64_3: u64 = 5486u64;
    let mut u64_4: u64 = 9135u64;
    let mut tuple_1: (u64, u64) = (u64_4, u64_3);
    let mut tuple_1_ref_0: &(u64, u64) = &mut tuple_1;
    let mut u64_5: u64 = 7497u64;
    let mut u32_10: u32 = 9785u32;
    let mut u64_6: u64 = 9790u64;
    let mut u64_7: u64 = crate::d2s_intrinsics::div100(u64_6);
    let mut u32_11: u32 = crate::f2s_intrinsics::pow5factor_32(u32_10);
    let mut u64_8: u64 = crate::d2s_intrinsics::mul_shift_64(u64_5, tuple_1_ref_0, u32_9);
    let mut u32_12: u32 = crate::common::decimal_length9(u32_8);
    let mut u64_9: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_7);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_6, u32_5);
    let mut u32_13: u32 = crate::common::decimal_length9(u32_4);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_3, u32_2, i32_1);
    let mut u32_15: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_0: i32 = 6104i32;
    let mut i32_1: i32 = -9737i32;
    let mut u64_0: u64 = 7595u64;
    let mut i32_2: i32 = -8129i32;
    let mut i32_3: i32 = -8331i32;
    let mut u32_0: u32 = 9783u32;
    let mut u32_1: u32 = 2259u32;
    let mut u32_2: u32 = 2064u32;
    let mut i32_4: i32 = -5071i32;
    let mut u64_1: u64 = 9679u64;
    let mut u64_2: u64 = 9922u64;
    let mut u64_3: u64 = 5009u64;
    let mut u32_3: u32 = 6322u32;
    let mut u32_4: u32 = 9790u32;
    let mut u64_4: u64 = 1757u64;
    let mut u32_5: u32 = 2652u32;
    let mut u32_6: u32 = 5288u32;
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_6, u32_5);
    let mut u64_5: u64 = crate::d2s_intrinsics::div5(u64_4);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_4, u32_3);
    let mut u32_7: u32 = crate::d2s::decimal_length17(u64_3);
    let mut u64_6: u64 = crate::d2s_intrinsics::div10(u64_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_1, exponent: i32_4};
    let mut u32_8: u32 = crate::f2s_intrinsics::pow5factor_32(u32_2);
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_3);
    let mut i32_5: i32 = crate::common::log2_pow5(i32_2);
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_0, exponent: i32_1};
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut i32_6: i32 = crate::common::ceil_log2_pow5(i32_0);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut i32_0: i32 = -845i32;
    let mut u64_0: u64 = 4321u64;
    let mut u32_0: u32 = 2316u32;
    let mut i32_1: i32 = -5149i32;
    let mut i32_2: i32 = 9017i32;
    let mut u32_1: u32 = 7359u32;
    let mut u64_1: u64 = 9199u64;
    let mut u32_2: u32 = 3213u32;
    let mut u64_2: u64 = 967u64;
    let mut i32_3: i32 = 10581i32;
    let mut u32_3: u32 = 4972u32;
    let mut u32_4: u32 = 1567u32;
    let mut i32_4: i32 = -14553i32;
    let mut u32_5: u32 = 3943u32;
    let mut u32_6: u32 = 3957u32;
    let mut u32_7: u32 = 853u32;
    let mut u32_8: u32 = 3837u32;
    let mut u32_9: u32 = 3351u32;
    let mut u32_10: u32 = crate::f2s_intrinsics::pow5factor_32(u32_9);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_8, u32_7);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_6, u32_5);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_11: u32 = crate::common::log10_pow2(i32_4);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_4, u32_3, i32_3);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_2, u32_2);
    let mut bool_3: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_1);
    let mut i32_5: i32 = crate::common::log2_pow5(i32_2);
    let mut i32_6: i32 = crate::common::pow5bits(i32_1);
    let mut u32_13: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut u32_0: u32 = 5434u32;
    let mut u64_0: u64 = 1073u64;
    let mut u64_1: u64 = 7868u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 4425u64;
    let mut i32_0: i32 = -2085i32;
    let mut u32_1: u32 = 9621u32;
    let mut u32_2: u32 = 8378u32;
    let mut u64_3: u64 = 3491u64;
    let mut u32_3: u32 = 6231u32;
    let mut u32_4: u32 = 3809u32;
    let mut i32_1: i32 = -10939i32;
    let mut u64_4: u64 = 8559u64;
    let mut u32_5: u32 = 3149u32;
    let mut u64_5: u64 = 9012u64;
    let mut i32_2: i32 = -9560i32;
    let mut u32_6: u32 = 3363u32;
    let mut u64_6: u64 = 8570u64;
    let mut i32_3: i32 = -15131i32;
    let mut u32_7: u32 = 3964u32;
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_7, exponent: i32_3};
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_6, u32_6);
    let mut i32_4: i32 = crate::common::log2_pow5(i32_2);
    let mut u32_8: u32 = crate::d2s::decimal_length17(u64_5);
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_shift_32(u32_5, u64_4, i32_1);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_4, u32_3);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u64_7: u64 = crate::d2s_intrinsics::div5(u64_3);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_2, u32_1, i32_0);
    let mut u64_8: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut i32_0: i32 = -9301i32;
    let mut u32_0: u32 = 1529u32;
    let mut u32_1: u32 = 155u32;
    let mut u64_0: u64 = 2126u64;
    let mut u32_2: u32 = 9990u32;
    let mut i32_1: i32 = 13100i32;
    let mut i32_2: i32 = -3723i32;
    let mut u32_3: u32 = 3939u32;
    let mut u32_4: u32 = 3105u32;
    let mut i32_3: i32 = 776i32;
    let mut i32_4: i32 = -17264i32;
    let mut u64_1: u64 = 6563u64;
    let mut u64_2: u64 = 6179u64;
    let mut u32_5: u32 = 2034u32;
    let mut u64_3: u64 = 1564u64;
    let mut i32_5: i32 = 3444i32;
    let mut u32_6: u32 = crate::common::log10_pow2(i32_5);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_3, u32_5);
    let mut u32_7: u32 = crate::d2s::decimal_length17(u64_2);
    let mut u64_4: u64 = crate::d2s_intrinsics::div5(u64_1);
    let mut i32_6: i32 = crate::common::log2_pow5(i32_4);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut i32_7: i32 = crate::common::ceil_log2_pow5(i32_3);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_4, u32_3);
    let mut u32_8: u32 = crate::common::log10_pow5(i32_2);
    let mut i32_8: i32 = crate::common::pow5bits(i32_1);
    let mut u32_9: u32 = crate::f2s_intrinsics::pow5factor_32(u32_2);
    let mut u64_5: u64 = crate::d2s_intrinsics::div10(u64_0);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_1, u32_0);
    let mut i32_9: i32 = crate::common::ceil_log2_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut i32_0: i32 = -500i32;
    let mut u32_0: u32 = 3376u32;
    let mut u32_1: u32 = 4171u32;
    let mut u32_2: u32 = 5563u32;
    let mut u32_3: u32 = 8984u32;
    let mut u64_0: u64 = 9619u64;
    let mut u64_1: u64 = 8744u64;
    let mut i32_1: i32 = 5743i32;
    let mut u64_2: u64 = 5350u64;
    let mut u32_4: u32 = 2156u32;
    let mut u32_5: u32 = 4868u32;
    let mut u64_3: u64 = 8758u64;
    let mut u32_6: u32 = 6291u32;
    let mut i32_2: i32 = -5957i32;
    let mut i32_3: i32 = 1724i32;
    let mut u32_7: u32 = 1024u32;
    let mut u32_8: u32 = 2048u32;
    let mut u32_9: u32 = 2518u32;
    let mut u32_10: u32 = 770u32;
    let mut u64_4: u64 = 3600u64;
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_4, u32_10);
    let mut u32_11: u32 = crate::common::decimal_length9(u32_9);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_8, u32_7, i32_3);
    let mut u32_13: u32 = crate::common::log10_pow5(i32_2);
    let mut u32_14: u32 = crate::common::decimal_length9(u32_6);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_3, u32_5);
    let mut u32_15: u32 = crate::f2s_intrinsics::mul_shift_32(u32_4, u64_2, i32_1);
    let mut u64_5: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut u32_16: u32 = crate::d2s::decimal_length17(u64_0);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_3, u32_2);
    let mut u32_17: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_1, u32_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut i32_0: i32 = 9426i32;
    let mut u32_0: u32 = 8593u32;
    let mut u32_1: u32 = 6398u32;
    let mut u32_2: u32 = 8204u32;
    let mut u32_3: u32 = 2046u32;
    let mut u64_0: u64 = 1008u64;
    let mut u64_1: u64 = 2855u64;
    let mut i32_1: i32 = -8957i32;
    let mut u32_4: u32 = 4146u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_2: i32 = -10379i32;
    let mut u64_2: u64 = 7848u64;
    let mut u32_5: u32 = 2387u32;
    let mut i32_3: i32 = 17681i32;
    let mut u32_6: u32 = 614u32;
    let mut u32_7: u32 = 1918u32;
    let mut u64_3: u64 = 1819u64;
    let mut i32_4: i32 = -9555i32;
    let mut u32_8: u32 = crate::common::log10_pow2(i32_4);
    let mut u32_9: u32 = crate::d2s::decimal_length17(u64_3);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_7, u32_6, i32_3);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_shift_32(u32_5, u64_2, i32_2);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_4, exponent: i32_1};
    let mut u64_4: u64 = crate::d2s_intrinsics::div5(u64_1);
    let mut u64_5: u64 = crate::d2s_intrinsics::div100(u64_0);
    let mut floatingdecimal32_1: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_3, u32_2);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_1, u32_0);
    let mut i32_5: i32 = crate::common::pow5bits(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut i32_0: i32 = -121i32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_0: u32 = 939u32;
    let mut u32_1: u32 = 1835u32;
    let mut u32_2: u32 = 4072u32;
    let mut u32_3: u32 = 9913u32;
    let mut u32_4: u32 = 5571u32;
    let mut u32_5: u32 = 1559u32;
    let mut i32_1: i32 = 7714i32;
    let mut u64_0: u64 = 9975u64;
    let mut u32_6: u32 = 1419u32;
    let mut i32_2: i32 = 4088i32;
    let mut i32_3: i32 = 13280i32;
    let mut u32_7: u32 = 5598u32;
    let mut u32_8: u32 = 7034u32;
    let mut u32_9: u32 = 1206u32;
    let mut u32_10: u32 = 9935u32;
    let mut u64_1: u64 = 5804u64;
    let mut u64_2: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_10, u32_9);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_8, u32_7, i32_3);
    let mut i32_4: i32 = crate::common::ceil_log2_pow5(i32_2);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_shift_32(u32_6, u64_0, i32_1);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_5, u32_4);
    let mut u32_13: u32 = crate::common::decimal_length9(u32_3);
    let mut floatingdecimal32_1: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_2, u32_1);
    let mut u32_14: u32 = crate::f2s_intrinsics::pow5factor_32(u32_0);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut i32_5: i32 = crate::common::pow5bits(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut i32_0: i32 = 8235i32;
    let mut u64_0: u64 = 7873u64;
    let mut u32_0: u32 = 433u32;
    let mut u32_1: u32 = 7660u32;
    let mut u32_2: u32 = 6749u32;
    let mut u64_1: u64 = 638u64;
    let mut u32_3: u32 = 5272u32;
    let mut u32_4: u32 = 7652u32;
    let mut u32_5: u32 = 6488u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_1: i32 = -1547i32;
    let mut u32_6: u32 = 2223u32;
    let mut i32_2: i32 = -2328i32;
    let mut u32_7: u32 = 8982u32;
    let mut u32_8: u32 = 1909u32;
    let mut u64_2: u64 = 7343u64;
    let mut u32_9: u32 = 1198u32;
    let mut u32_10: u32 = 4800u32;
    let mut u64_3: u64 = 7125u64;
    let mut u64_4: u64 = crate::d2s_intrinsics::div10(u64_3);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_10, u32_9);
    let mut u64_5: u64 = crate::d2s_intrinsics::div10(u64_2);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_8, u32_7, i32_2);
    let mut u32_12: u32 = crate::f2s_intrinsics::pow5factor_32(u32_6);
    let mut i32_3: i32 = crate::common::log2_pow5(i32_1);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_5, u32_4);
    let mut u32_13: u32 = crate::f2s_intrinsics::pow5factor_32(u32_3);
    let mut u64_6: u64 = crate::d2s_intrinsics::div5(u64_1);
    let mut floatingdecimal32_1: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_2, u32_1);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u64_0: u64 = 4274u64;
    let mut u64_1: u64 = 7885u64;
    let mut u64_2: u64 = 5019u64;
    let mut u64_3: u64 = 5371u64;
    let mut i32_0: i32 = 5091i32;
    let mut u64_4: u64 = 5244u64;
    let mut u32_0: u32 = 455u32;
    let mut i32_1: i32 = 13090i32;
    let mut u32_1: u32 = 5781u32;
    let mut u32_2: u32 = 3863u32;
    let mut u32_3: u32 = 3197u32;
    let mut u32_4: u32 = 537u32;
    let mut u32_5: u32 = 4811u32;
    let mut i32_2: i32 = -11042i32;
    let mut u64_5: u64 = 4329u64;
    let mut u64_6: u64 = 3212u64;
    let mut u32_6: u32 = 541u32;
    let mut u64_7: u64 = 7765u64;
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_7, u32_6);
    let mut u64_8: u64 = crate::d2s_intrinsics::div5(u64_6);
    let mut u64_9: u64 = crate::d2s_intrinsics::div100(u64_5);
    let mut u32_7: u32 = crate::common::log10_pow2(i32_2);
    let mut u32_8: u32 = crate::common::decimal_length9(u32_5);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_4, u32_3);
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_2, u32_1, i32_1);
    let mut u32_10: u32 = crate::f2s_intrinsics::mul_shift_32(u32_0, u64_4, i32_0);
    let mut u64_10: u64 = crate::d2s_intrinsics::div5(u64_3);
    let mut u64_11: u64 = crate::d2s_intrinsics::div5(u64_2);
    let mut u64_12: u64 = crate::d2s_intrinsics::div5(u64_1);
    let mut u64_13: u64 = crate::d2s_intrinsics::div100(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u32_0: u32 = 4257u32;
    let mut u64_0: u64 = 6533u64;
    let mut u64_1: u64 = 4120u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 4610u64;
    let mut u32_1: u32 = 4863u32;
    let mut u64_3: u64 = 7593u64;
    let mut i32_0: i32 = -8487i32;
    let mut u64_4: u64 = 8147u64;
    let mut u32_2: u32 = 3881u32;
    let mut u32_3: u32 = 9727u32;
    let mut u64_5: u64 = 3148u64;
    let mut u64_6: u64 = 6169u64;
    let mut tuple_1: (u64, u64) = (u64_6, u64_5);
    let mut tuple_1_ref_0: &(u64, u64) = &mut tuple_1;
    let mut u64_7: u64 = 553u64;
    let mut u32_4: u32 = 3498u32;
    let mut i32_1: i32 = -5055i32;
    let mut u32_5: u32 = 2928u32;
    let mut i32_2: i32 = 6661i32;
    let mut u32_6: u32 = 7398u32;
    let mut u64_8: u64 = 884u64;
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_8, u32_6);
    let mut u32_7: u32 = crate::common::log10_pow5(i32_2);
    let mut u32_8: u32 = crate::f2s_intrinsics::pow5factor_32(u32_5);
    let mut u32_9: u32 = crate::common::log10_pow2(i32_1);
    let mut u32_10: u32 = crate::common::decimal_length9(u32_4);
    let mut u64_9: u64 = crate::d2s_intrinsics::mul_shift_64(u64_7, tuple_1_ref_0, u32_3);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_shift_32(u32_2, u64_4, i32_0);
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_3, u32_1);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u64_10: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}
}