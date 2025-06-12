use crate::digit_table::DIGIT_TABLE;
use core::ptr;

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent3(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    debug_assert!(k < 1000);
    if k >= 100 {
        *result = b'0' + (k / 100) as u8;
        k %= 100;
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result.offset(1), 2);
        sign as usize + 3
    } else if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
    }
}

#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_exponent2(mut k: isize, mut result: *mut u8) -> usize {
    let sign = k < 0;
    if sign {
        *result = b'-';
        result = result.offset(1);
        k = -k;
    }

    debug_assert!(k < 100);
    if k >= 10 {
        let d = DIGIT_TABLE.as_ptr().offset(k * 2);
        ptr::copy_nonoverlapping(d, result, 2);
        sign as usize + 2
    } else {
        *result = b'0' + k as u8;
        sign as usize + 1
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
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut u32_0: u32 = 2382u32;
    let mut u64_0: u64 = 1863u64;
    let mut i32_0: i32 = -2257i32;
    let mut i32_1: i32 = 27253i32;
    let mut u32_1: u32 = 2714u32;
    let mut u32_2: u32 = 6602u32;
    let mut u32_3: u32 = 8715u32;
    let mut i32_2: i32 = -1160i32;
    let mut u32_4: u32 = 9027u32;
    let mut u64_1: u64 = 4542u64;
    let mut i32_3: i32 = 181i32;
    let mut u64_2: u64 = 7062u64;
    let mut u64_3: u64 = 3534u64;
    let mut u32_5: u32 = 8711u32;
    let mut u32_6: u32 = 5254u32;
    let mut i32_4: i32 = -6953i32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut i32_5: i32 = crate::common::log2_pow5(i32_4);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_6, u32_5);
    let mut u32_7: u32 = crate::d2s::decimal_length17(u64_3);
    let mut u64_4: u64 = crate::d2s_intrinsics::div100(u64_2);
    let mut i32_6: i32 = crate::common::ceil_log2_pow5(i32_3);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_1, u32_4);
    let mut i32_7: i32 = crate::common::ceil_log2_pow5(i32_2);
    let mut u32_8: u32 = crate::common::decimal_length9(u32_3);
    let mut u32_9: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_2, u32_1, i32_1);
    let mut i32_8: i32 = crate::common::log2_pow5(i32_0);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_0, u32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut u32_0: u32 = 8901u32;
    let mut u64_0: u64 = 9697u64;
    let mut u64_1: u64 = 1268u64;
    let mut tuple_0: (u64, u64) = (u64_1, u64_0);
    let mut tuple_0_ref_0: &(u64, u64) = &mut tuple_0;
    let mut u64_2: u64 = 2869u64;
    let mut i32_0: i32 = 23902i32;
    let mut u32_1: u32 = 2798u32;
    let mut u64_3: u64 = 1019u64;
    let mut u64_4: u64 = 8255u64;
    let mut u32_2: u32 = 9944u32;
    let mut u64_5: u64 = 3607u64;
    let mut u32_3: u32 = 8651u32;
    let mut u32_4: u32 = 5429u32;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_4, u32_3);
    let mut bool_1: bool = crate::d2s_intrinsics::multiple_of_power_of_5(u64_5, u32_2);
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut u64_6: u64 = crate::d2s_intrinsics::div10(u64_4);
    let mut bool_2: bool = crate::d2s_intrinsics::multiple_of_power_of_2(u64_3, u32_1);
    let mut u32_5: u32 = crate::common::log10_pow5(i32_0);
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_4_ref_0: &crate::buffer::Buffer = &mut buffer_4;
    let mut buffer_5: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_4_ref_0);
    let mut u64_7: u64 = crate::d2s_intrinsics::mul_shift_64(u64_2, tuple_0_ref_0, u32_0);
    let mut buffer_6: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    panic!("From RustyUnit with love");
}
}