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

// Returns the number of decimal digits in v, which must not contain more than 9
// digits.
#[cfg_attr(feature = "no-panic", inline)]
pub fn decimal_length9(v: u32) -> u32 {
    // Function precondition: v is not a 10-digit number.
    // (f2s: 9 digits are sufficient for round-tripping.)
    debug_assert!(v < 1000000000);

    if v >= 100000000 {
        9
    } else if v >= 10000000 {
        8
    } else if v >= 1000000 {
        7
    } else if v >= 100000 {
        6
    } else if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    }
}

// Returns e == 0 ? 1 : [log_2(5^e)]; requires 0 <= e <= 3528.
#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn log2_pow5(e: i32) -> i32 /* or u32 -> u32 */ {
    // This approximation works up to the point that the multiplication
    // overflows at e = 3529. If the multiplication were done in 64 bits, it
    // would fail at 5^4004 which is just greater than 2^9297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    ((e as u32 * 1217359) >> 19) as i32
}

// Returns e == 0 ? 1 : ceil(log_2(5^e)); requires 0 <= e <= 3528.
#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 /* or u32 -> u32 */ {
    // This approximation works up to the point that the multiplication
    // overflows at e = 3529. If the multiplication were done in 64 bits, it
    // would fail at 5^4004 which is just greater than 2^9297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}

#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn ceil_log2_pow5(e: i32) -> i32 /* or u32 -> u32 */ {
    log2_pow5(e) + 1
}

// Returns floor(log_10(2^e)); requires 0 <= e <= 1650.
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow2(e: i32) -> u32 /* or u32 -> u32 */ {
    // The first value this approximation fails for is 2^1651 which is just greater than 10^297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}

// Returns floor(log_10(5^e)); requires 0 <= e <= 2620.
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow5(e: i32) -> u32 /* or u32 -> u32 */ {
    // The first value this approximation fails for is 5^2621 which is just greater than 10^1832.
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::clone::Clone;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut u32_0: u32 = 9196u32;
    let mut u64_0: u64 = 6355u64;
    let mut u64_1: u64 = 1458u64;
    let mut u32_1: u32 = 3175u32;
    let mut u64_2: u64 = 3949u64;
    let mut i32_0: i32 = -3908i32;
    let mut u64_3: u64 = 4582u64;
    let mut u32_2: u32 = 741u32;
    let mut i32_1: i32 = -3635i32;
    let mut i32_2: i32 = -13542i32;
    let mut u32_3: u32 = 9743u32;
    let mut u32_4: u32 = 3075u32;
    let mut i32_3: i32 = -3345i32;
    let mut i32_4: i32 = 23938i32;
    let mut i32_5: i32 = 5666i32;
    let mut u32_5: u32 = 8597u32;
    let mut u32_6: u32 = 3932u32;
    let mut i32_6: i32 = -6082i32;
    let mut i32_7: i32 = crate::common::log2_pow5(i32_6);
    let mut u32_7: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_6, u32_5, i32_5);
    let mut i32_8: i32 = crate::common::ceil_log2_pow5(i32_4);
    let mut i32_9: i32 = crate::common::ceil_log2_pow5(i32_3);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_4, u32_3, i32_2);
    let mut u32_9: u32 = crate::common::log10_pow5(i32_1);
    let mut u32_10: u32 = crate::common::decimal_length9(u32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_3, exponent: i32_0};
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_2, u32_1);
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_11: u32 = crate::d2s::decimal_length17(u64_1);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut u32_0: u32 = 2973u32;
    let mut u64_0: u64 = 9352u64;
    let mut u64_1: u64 = 9184u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 4454u64;
    let mut u32_1: u32 = 4739u32;
    let mut u32_2: u32 = 8137u32;
    let mut u64_3: u64 = 1963u64;
    let mut u64_4: u64 = 313u64;
    let mut i32_0: i32 = 15880i32;
    let mut u32_3: u32 = 6162u32;
    let mut u32_4: u32 = 2793u32;
    let mut u64_5: u64 = 6740u64;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_5: u32 = 6185u32;
    let mut i32_1: i32 = -1457i32;
    let mut i32_2: i32 = crate::common::ceil_log2_pow5(i32_1);
    let mut u32_6: u32 = crate::common::decimal_length9(u32_5);
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_7: u32 = crate::d2s::decimal_length17(u64_5);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_4, u32_3, i32_0);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut buffer_2_ref_0: &crate::buffer::Buffer = &mut buffer_2;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_2_ref_0);
    let mut u64_6: u64 = crate::d2s_intrinsics::div5(u64_4);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_3, u32_2);
    let mut u32_9: u32 = crate::common::decimal_length9(u32_1);
    let mut u64_7: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}
}