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

// A table of all two-digit numbers. This is used to speed up decimal digit
// generation by copying pairs of digits into the final output.
pub static DIGIT_TABLE: [u8; 200] = *b"\
    0001020304050607080910111213141516171819\
    2021222324252627282930313233343536373839\
    4041424344454647484950515253545556575859\
    6061626364656667686970717273747576777879\
    8081828384858687888990919293949596979899";

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut u32_0: u32 = 2133u32;
    let mut u64_0: u64 = 8749u64;
    let mut u64_1: u64 = 1463u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 6782u64;
    let mut u32_1: u32 = 640u32;
    let mut u64_3: u64 = 2400u64;
    let mut u32_2: u32 = 7179u32;
    let mut u64_4: u64 = 2084u64;
    let mut u32_3: u32 = 1855u32;
    let mut u32_4: u32 = 4364u32;
    let mut i32_0: i32 = -12298i32;
    let mut u64_5: u64 = 4829u64;
    let mut u64_6: u64 = 5556u64;
    let mut u32_5: u32 = 495u32;
    let mut u64_7: u64 = 8265u64;
    let mut i32_1: i32 = 6177i32;
    let mut u64_8: u64 = 576u64;
    let mut i32_2: i32 = 20521i32;
    let mut i32_3: i32 = crate::common::log2_pow5(i32_2);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_8, exponent: i32_1};
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_7, u32_5);
    let mut u32_6: u32 = crate::d2s::decimal_length17(u64_6);
    let mut u64_9: u64 = crate::d2s_intrinsics::div10(u64_5);
    let mut i32_4: i32 = crate::common::log2_pow5(i32_0);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_4, u32_3);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_4, u32_2);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_3, u32_1);
    let mut u64_10: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    panic!("From RustyUnit with love");
}
}