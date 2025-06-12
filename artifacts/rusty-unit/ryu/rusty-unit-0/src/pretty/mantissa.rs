use crate::digit_table::DIGIT_TABLE;
use core::ptr;

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_mantissa_long(mut output: u64, mut result: *mut u8) {
    if (output >> 32) != 0 {
        // One expensive 64-bit division.
        let mut output2 = (output - 100_000_000 * (output / 100_000_000)) as u32;
        output /= 100_000_000;

        let c = output2 % 10_000;
        output2 /= 10_000;
        let d = output2 % 10_000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        let d0 = (d % 100) << 1;
        let d1 = (d / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(-2),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(-4),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d0 as isize),
            result.offset(-6),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(d1 as isize),
            result.offset(-8),
            2,
        );
        result = result.offset(-8);
    }
    write_mantissa(output as u32, result);
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_mantissa(mut output: u32, mut result: *mut u8) {
    while output >= 10_000 {
        let c = output - 10_000 * (output / 10_000);
        output /= 10_000;
        let c0 = (c % 100) << 1;
        let c1 = (c / 100) << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c0 as isize),
            result.offset(-2),
            2,
        );
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c1 as isize),
            result.offset(-4),
            2,
        );
        result = result.offset(-4);
    }
    if output >= 100 {
        let c = (output % 100) << 1;
        output /= 100;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c as isize),
            result.offset(-2),
            2,
        );
        result = result.offset(-2);
    }
    if output >= 10 {
        let c = output << 1;
        ptr::copy_nonoverlapping(
            DIGIT_TABLE.as_ptr().offset(c as isize),
            result.offset(-2),
            2,
        );
    } else {
        *result.offset(-1) = b'0' + output as u8;
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut i32_0: i32 = 12863i32;
    let mut u32_0: u32 = 6943u32;
    let mut u32_1: u32 = 8492u32;
    let mut u64_0: u64 = 3294u64;
    let mut u32_2: u32 = 9752u32;
    let mut u64_1: u64 = 72u64;
    let mut u32_3: u32 = 6740u32;
    let mut u64_2: u64 = 2107u64;
    let mut u64_3: u64 = 9715u64;
    let mut tuple_0: (u64, u64) = (u64_3, u64_2);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_4: u64 = 4889u64;
    let mut u64_5: u64 = 8268u64;
    let mut u32_4: u32 = 1481u32;
    let mut u64_6: u64 = 423u64;
    let mut u32_5: u32 = 8746u32;
    let mut u32_6: u32 = 3891u32;
    let mut u64_7: u64 = 602u64;
    let mut i32_1: i32 = -5192i32;
    let mut u32_7: u32 = 5822u32;
    let mut u32_8: u32 = 5466u32;
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_inv_div_pow2(u32_8, u32_7, i32_1);
    let mut bool_0: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_7, u32_6);
    let mut u32_10: u32 = crate::common::decimal_length9(u32_5);
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_6, u32_4);
    let mut u64_8: u64 = crate::d2s_intrinsics::div100(u64_5);
    let mut u64_9: u64 = crate::d2s_intrinsics::mul_shift_64(u64_4, tuple_0_ref_0, u32_3);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_1, u32_2);
    let mut u64_10: u64 = crate::d2s_intrinsics::div100(u64_0);
    let mut u32_11: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_0);
    panic!("From RustyUnit with love");
}
}