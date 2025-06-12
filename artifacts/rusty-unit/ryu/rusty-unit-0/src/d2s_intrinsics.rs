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
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut u64_0: u64 = 7024u64;
    let mut i32_0: i32 = 21750i32;
    let mut u32_0: u32 = 8228u32;
    let mut u32_1: u32 = 5110u32;
    let mut u32_2: u32 = 4127u32;
    let mut u32_3: u32 = 2536u32;
    let mut u64_1: u64 = 444u64;
    let mut u32_4: u32 = 231u32;
    let mut u32_5: u32 = 7361u32;
    let mut i32_1: i32 = 9780i32;
    let mut u32_6: u32 = 7746u32;
    let mut u32_7: u32 = 8259u32;
    let mut u64_2: u64 = 3905u64;
    let mut u64_3: u64 = 8654u64;
    let mut i32_2: i32 = -2667i32;
    let mut u32_8: u32 = 7653u32;
    let mut u64_4: u64 = 674u64;
    let mut i32_3: i32 = -14574i32;
    let mut u32_9: u32 = crate::common::log10_pow2(i32_3);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_4, u32_8);
    let mut u32_10: u32 = crate::common::log10_pow2(i32_2);
    let mut u32_11: u32 = crate::d2s_intrinsics::pow5_factor(u64_3);
    let mut u32_12: u32 = crate::d2s_intrinsics::pow5_factor(u64_2);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_7, u32_6);
    let mut u32_13: u32 = crate::common::log10_pow2(i32_1);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_5, u32_4);
    let mut u64_5: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut bool_2: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_3, u32_2);
    let mut u32_14: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_0);
    let mut u32_15: u32 = crate::d2s_intrinsics::pow5_factor(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut u64_0: u64 = 2255u64;
    let mut u64_1: u64 = 890u64;
    let mut i32_0: i32 = -10024i32;
    let mut u32_0: u32 = 0u32;
    let mut u32_1: u32 = 7135u32;
    let mut u64_2: u64 = 3617u64;
    let mut u64_3: u64 = 6951u64;
    let mut i32_1: i32 = -1173i32;
    let mut i32_2: i32 = 1080i32;
    let mut i32_3: i32 = -6890i32;
    let mut u32_2: u32 = 1873u32;
    let mut u32_3: u32 = 4632u32;
    let mut u64_4: u64 = 9863u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u64_5: u64 = 8653u64;
    let mut u64_6: u64 = crate::d2s_intrinsics::div5(u64_5);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_4: u32 = crate::d2s_intrinsics::pow5_factor(u64_4);
    let mut u32_5: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_3, u32_2, i32_3);
    let mut i32_4: i32 = crate::common::ceil_log2_pow5(i32_2);
    let mut u32_6: u32 = crate::common::log10_pow5(i32_1);
    let mut u32_7: u32 = crate::d2s_intrinsics::pow5_factor(u64_3);
    let mut u32_8: u32 = crate::d2s::decimal_length17(u64_2);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_1, u32_0);
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_9: u32 = crate::common::log10_pow2(i32_0);
    let mut u64_7: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut u32_10: u32 = crate::d2s::decimal_length17(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut u64_0: u64 = 4013u64;
    let mut i32_0: i32 = 14800i32;
    let mut u32_0: u32 = 7710u32;
    let mut u32_1: u32 = 6376u32;
    let mut i32_1: i32 = 15186i32;
    let mut u32_2: u32 = 6772u32;
    let mut u32_3: u32 = 7718u32;
    let mut u64_1: u64 = 150u64;
    let mut u64_2: u64 = 9919u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_4: u32 = 4310u32;
    let mut u64_3: u64 = 8886u64;
    let mut u64_4: u64 = 5499u64;
    let mut u32_5: u32 = 9898u32;
    let mut u32_6: u32 = 6584u32;
    let mut u32_7: u32 = 9613u32;
    let mut u32_8: u32 = 4139u32;
    let mut u64_5: u64 = 5120u64;
    let mut u32_9: u32 = crate::d2s_intrinsics::pow5_factor(u64_5);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_8, u32_7);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_6, u32_5);
    let mut u64_6: u64 = crate::d2s_intrinsics::div100(u64_4);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_3, u32_4);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_10: u32 = crate::d2s::decimal_length17(u64_2);
    let mut u32_11: u32 = crate::d2s::decimal_length17(u64_1);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_3, u32_2, i32_1);
    let mut bool_2: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_1, u32_0);
    let mut u32_13: u32 = crate::common::log10_pow5(i32_0);
    let mut u64_7: u64 = crate::d2s_intrinsics::div5(u64_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut i32_0: i32 = -10490i32;
    let mut u32_0: u32 = 4526u32;
    let mut u32_1: u32 = 1503u32;
    let mut u32_2: u32 = 7170u32;
    let mut u32_3: u32 = 8623u32;
    let mut u32_4: u32 = 7482u32;
    let mut i32_1: i32 = 12899i32;
    let mut u64_0: u64 = 5111u64;
    let mut i32_2: i32 = 5829i32;
    let mut u32_5: u32 = 2367u32;
    let mut u64_1: u64 = 6821u64;
    let mut u32_6: u32 = 5000u32;
    let mut u64_2: u64 = 344u64;
    let mut u32_7: u32 = 9395u32;
    let mut u64_3: u64 = 7341u64;
    let mut u64_4: u64 = 403u64;
    let mut u64_5: u64 = crate::d2s_intrinsics::div5(u64_4);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_3, u32_7);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_2, u32_6);
    let mut u32_8: u32 = crate::d2s_intrinsics::pow5_factor(u64_1);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_5, exponent: i32_2};
    let mut u32_9: u32 = crate::d2s_intrinsics::pow5_factor(u64_0);
    let mut u32_10: u32 = crate::common::log10_pow5(i32_1);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u32_11: u32 = crate::common::decimal_length9(u32_4);
    let mut buffer_2_ref_0: &crate::buffer::Buffer = &mut buffer_2;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_2_ref_0);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_3, u32_2);
    let mut u32_12: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_1, u32_0, i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut i32_0: i32 = 7019i32;
    let mut u32_0: u32 = 6108u32;
    let mut u32_1: u32 = 763u32;
    let mut u32_2: u32 = 1611u32;
    let mut u32_3: u32 = 8241u32;
    let mut i32_1: i32 = -4306i32;
    let mut u32_4: u32 = 3421u32;
    let mut u64_0: u64 = 7426u64;
    let mut i32_2: i32 = -11609i32;
    let mut u64_1: u64 = 7613u64;
    let mut u32_5: u32 = 2278u32;
    let mut u64_2: u64 = 3920u64;
    let mut u64_3: u64 = 6212u64;
    let mut i32_3: i32 = -815i32;
    let mut i32_4: i32 = 6767i32;
    let mut u64_4: u64 = 9617u64;
    let mut i32_5: i32 = -13886i32;
    let mut u32_6: u32 = crate::common::log10_pow5(i32_5);
    let mut u64_5: u64 = crate::d2s_intrinsics::div100(u64_4);
    let mut u32_7: u32 = crate::common::log10_pow5(i32_4);
    let mut u32_8: u32 = crate::common::log10_pow2(i32_3);
    let mut u64_6: u64 = crate::d2s_intrinsics::div5(u64_3);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_2, u32_5);
    let mut u32_9: u32 = crate::d2s_intrinsics::pow5_factor(u64_1);
    let mut i32_6: i32 = crate::common::ceil_log2_pow5(i32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_0, u32_4);
    let mut i32_7: i32 = crate::common::pow5bits(i32_1);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_3, u32_2);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_1, u32_0);
    let mut i32_8: i32 = crate::common::pow5bits(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut u64_0: u64 = 8970u64;
    let mut i32_0: i32 = 15560i32;
    let mut i32_1: i32 = 13492i32;
    let mut u32_0: u32 = 858u32;
    let mut u64_1: u64 = 9123u64;
    let mut u32_1: u32 = 8608u32;
    let mut u32_2: u32 = 4755u32;
    let mut i32_2: i32 = 3541i32;
    let mut i32_3: i32 = -13691i32;
    let mut u64_2: u64 = 7328u64;
    let mut i32_4: i32 = -1755i32;
    let mut u32_3: u32 = 2959u32;
    let mut u32_4: u32 = 161u32;
    let mut u32_5: u32 = 1295u32;
    let mut u64_3: u64 = 9161u64;
    let mut u64_4: u64 = 2964u64;
    let mut tuple_0: (u64, u64) = (u64_4, u64_3);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_5: u64 = 264u64;
    let mut u64_6: u64 = crate::d2s_intrinsics::mul_shift_64(u64_5, tuple_0_ref_0, u32_5);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_4, u32_3);
    let mut i32_5: i32 = crate::common::ceil_log2_pow5(i32_4);
    let mut u32_6: u32 = crate::d2s_intrinsics::pow5_factor(u64_2);
    let mut u32_7: u32 = crate::common::log10_pow2(i32_3);
    let mut i32_6: i32 = crate::common::pow5bits(i32_2);
    let mut bool_1: bool = crate::f2s_intrinsics::multiple_of_power_of_2_32(u32_2, u32_1);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_0);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u32_8: u32 = crate::common::log10_pow5(i32_1);
    let mut i32_7: i32 = crate::common::pow5bits(i32_0);
    let mut u64_7: u64 = crate::d2s_intrinsics::div5(u64_0);
    panic!("From RustyUnit with love");
}
}