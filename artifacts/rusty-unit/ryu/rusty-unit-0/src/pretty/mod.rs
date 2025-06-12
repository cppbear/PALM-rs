mod exponent;
mod mantissa;

use self::exponent::{write_exponent2, write_exponent3};
use self::mantissa::{write_mantissa, write_mantissa_long};
use crate::common;
use crate::d2s::{self, d2d, DOUBLE_EXPONENT_BITS, DOUBLE_MANTISSA_BITS};
use crate::f2s::{f2d, FLOAT_EXPONENT_BITS, FLOAT_MANTISSA_BITS};
use core::ptr;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;

/// Print f64 to the given buffer and return number of bytes written.
///
/// At most 24 bytes will be written.
///
/// ## Special cases
///
/// This function **does not** check for NaN or infinity. If the input
/// number is not a finite float, the printed representation will be some
/// correctly formatted but unspecified numerical value.
///
/// Please check [`is_finite`] yourself before calling this function, or
/// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
///
/// [`is_finite`]: f64::is_finite
/// [`is_nan`]: f64::is_nan
/// [`is_infinite`]: f64::is_infinite
///
/// ## Safety
///
/// The `result` pointer argument must point to sufficiently many writable bytes
/// to hold Ryū's representation of `f`.
///
/// ## Example
///
/// ```
/// use std::{mem::MaybeUninit, slice, str};
///
/// let f = 1.234f64;
///
/// unsafe {
///     let mut buffer = [MaybeUninit::<u8>::uninit(); 24];
///     let len = ryu::raw::format64(f, buffer.as_mut_ptr() as *mut u8);
///     let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
///     let print = str::from_utf8_unchecked(slice);
///     assert_eq!(print, "1.234");
/// }
/// ```
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format64(f: f64, result: *mut u8) -> usize {
    let bits = f.to_bits();
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent =
        (bits >> DOUBLE_MANTISSA_BITS) as u32 & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);

    let mut index = 0isize;
    if sign {
        *result = b'-';
        index += 1;
    }

    if ieee_exponent == 0 && ieee_mantissa == 0 {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), result.offset(index), 3);
        return sign as usize + 3;
    }

    let v = d2d(ieee_mantissa, ieee_exponent);

    let length = d2s::decimal_length17(v.mantissa) as isize;
    let k = v.exponent as isize;
    let kk = length + k; // 10^(kk-1) <= v < 10^kk
    debug_assert!(k >= -324);

    if 0 <= k && kk <= 16 {
        // 1234e7 -> 12340000000.0
        write_mantissa_long(v.mantissa, result.offset(index + length));
        for i in length..kk {
            *result.offset(index + i) = b'0';
        }
        *result.offset(index + kk) = b'.';
        *result.offset(index + kk + 1) = b'0';
        index as usize + kk as usize + 2
    } else if 0 < kk && kk <= 16 {
        // 1234e-2 -> 12.34
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        ptr::copy(result.offset(index + 1), result.offset(index), kk as usize);
        *result.offset(index + kk) = b'.';
        index as usize + length as usize + 1
    } else if -5 < kk && kk <= 0 {
        // 1234e-6 -> 0.001234
        *result.offset(index) = b'0';
        *result.offset(index + 1) = b'.';
        let offset = 2 - kk;
        for i in 2..offset {
            *result.offset(index + i) = b'0';
        }
        write_mantissa_long(v.mantissa, result.offset(index + length + offset));
        index as usize + length as usize + offset as usize
    } else if length == 1 {
        // 1e30
        *result.offset(index) = b'0' + v.mantissa as u8;
        *result.offset(index + 1) = b'e';
        index as usize + 2 + write_exponent3(kk - 1, result.offset(index + 2))
    } else {
        // 1234e30 -> 1.234e33
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        *result.offset(index) = *result.offset(index + 1);
        *result.offset(index + 1) = b'.';
        *result.offset(index + length + 1) = b'e';
        index as usize
            + length as usize
            + 2
            + write_exponent3(kk - 1, result.offset(index + length + 2))
    }
}

/// Print f32 to the given buffer and return number of bytes written.
///
/// At most 16 bytes will be written.
///
/// ## Special cases
///
/// This function **does not** check for NaN or infinity. If the input
/// number is not a finite float, the printed representation will be some
/// correctly formatted but unspecified numerical value.
///
/// Please check [`is_finite`] yourself before calling this function, or
/// check [`is_nan`] and [`is_infinite`] and handle those cases yourself.
///
/// [`is_finite`]: f32::is_finite
/// [`is_nan`]: f32::is_nan
/// [`is_infinite`]: f32::is_infinite
///
/// ## Safety
///
/// The `result` pointer argument must point to sufficiently many writable bytes
/// to hold Ryū's representation of `f`.
///
/// ## Example
///
/// ```
/// use std::{mem::MaybeUninit, slice, str};
///
/// let f = 1.234f32;
///
/// unsafe {
///     let mut buffer = [MaybeUninit::<u8>::uninit(); 16];
///     let len = ryu::raw::format32(f, buffer.as_mut_ptr() as *mut u8);
///     let slice = slice::from_raw_parts(buffer.as_ptr() as *const u8, len);
///     let print = str::from_utf8_unchecked(slice);
///     assert_eq!(print, "1.234");
/// }
/// ```
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format32(f: f32, result: *mut u8) -> usize {
    let bits = f.to_bits();
    let sign = ((bits >> (FLOAT_MANTISSA_BITS + FLOAT_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u32 << FLOAT_MANTISSA_BITS) - 1);
    let ieee_exponent = (bits >> FLOAT_MANTISSA_BITS) & ((1u32 << FLOAT_EXPONENT_BITS) - 1);

    let mut index = 0isize;
    if sign {
        *result = b'-';
        index += 1;
    }

    if ieee_exponent == 0 && ieee_mantissa == 0 {
        ptr::copy_nonoverlapping(b"0.0".as_ptr(), result.offset(index), 3);
        return sign as usize + 3;
    }

    let v = f2d(ieee_mantissa, ieee_exponent);

    let length = common::decimal_length9(v.mantissa) as isize;
    let k = v.exponent as isize;
    let kk = length + k; // 10^(kk-1) <= v < 10^kk
    debug_assert!(k >= -45);

    if 0 <= k && kk <= 13 {
        // 1234e7 -> 12340000000.0
        write_mantissa(v.mantissa, result.offset(index + length));
        for i in length..kk {
            *result.offset(index + i) = b'0';
        }
        *result.offset(index + kk) = b'.';
        *result.offset(index + kk + 1) = b'0';
        index as usize + kk as usize + 2
    } else if 0 < kk && kk <= 13 {
        // 1234e-2 -> 12.34
        write_mantissa(v.mantissa, result.offset(index + length + 1));
        ptr::copy(result.offset(index + 1), result.offset(index), kk as usize);
        *result.offset(index + kk) = b'.';
        index as usize + length as usize + 1
    } else if -6 < kk && kk <= 0 {
        // 1234e-6 -> 0.001234
        *result.offset(index) = b'0';
        *result.offset(index + 1) = b'.';
        let offset = 2 - kk;
        for i in 2..offset {
            *result.offset(index + i) = b'0';
        }
        write_mantissa(v.mantissa, result.offset(index + length + offset));
        index as usize + length as usize + offset as usize
    } else if length == 1 {
        // 1e30
        *result.offset(index) = b'0' + v.mantissa as u8;
        *result.offset(index + 1) = b'e';
        index as usize + 2 + write_exponent2(kk - 1, result.offset(index + 2))
    } else {
        // 1234e30 -> 1.234e33
        write_mantissa(v.mantissa, result.offset(index + length + 1));
        *result.offset(index) = *result.offset(index + 1);
        *result.offset(index + 1) = b'.';
        *result.offset(index + length + 1) = b'e';
        index as usize
            + length as usize
            + 2
            + write_exponent2(kk - 1, result.offset(index + length + 2))
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
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut i32_0: i32 = -8650i32;
    let mut u64_0: u64 = 5664u64;
    let mut i32_1: i32 = -432i32;
    let mut u32_0: u32 = 4152u32;
    let mut u32_1: u32 = 1910u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut i32_2: i32 = -2693i32;
    let mut u64_1: u64 = 2356u64;
    let mut i32_3: i32 = 6925i32;
    let mut u32_2: u32 = 5364u32;
    let mut u32_3: u32 = 8828u32;
    let mut u32_4: u32 = 3957u32;
    let mut i32_4: i32 = 11932i32;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u32_5: u32 = crate::common::log10_pow5(i32_4);
    let mut u32_6: u32 = crate::common::decimal_length9(u32_4);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::f2d(u32_3, u32_2);
    let mut i32_5: i32 = crate::common::pow5bits(i32_3);
    let mut buffer_2_ref_0: &crate::buffer::Buffer = &mut buffer_2;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_2_ref_0);
    let mut u64_2: u64 = crate::d2s_intrinsics::div100(u64_1);
    let mut u32_7: u32 = crate::common::log10_pow2(i32_2);
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_8: u32 = crate::f2s_intrinsics::mul_pow5_div_pow2(u32_1, u32_0, i32_1);
    let mut u32_9: u32 = crate::d2s::decimal_length17(u64_0);
    let mut buffer_4_ref_0: &crate::buffer::Buffer = &mut buffer_4;
    let mut buffer_5: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_4_ref_0);
    let mut i32_6: i32 = crate::common::ceil_log2_pow5(i32_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut i32_0: i32 = -21699i32;
    let mut u32_0: u32 = 7273u32;
    let mut u32_1: u32 = 6001u32;
    let mut u64_0: u64 = 6573u64;
    let mut u64_1: u64 = 1374u64;
    let mut u32_2: u32 = 1352u32;
    let mut u32_3: u32 = 343u32;
    let mut i32_1: i32 = 8529i32;
    let mut i32_2: i32 = -7441i32;
    let mut i32_3: i32 = -6195i32;
    let mut i32_4: i32 = -27188i32;
    let mut u64_2: u64 = 352u64;
    let mut u32_4: u32 = 9479u32;
    let mut buffer_0: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut buffer_0_ref_0: &crate::buffer::Buffer = &mut buffer_0;
    let mut buffer_1: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_0_ref_0);
    let mut u32_5: u32 = crate::common::decimal_length9(u32_4);
    let mut buffer_2: crate::buffer::Buffer = crate::buffer::Buffer::default();
    let mut floatingdecimal64_0: crate::d2s::FloatingDecimal64 = crate::d2s::FloatingDecimal64 {mantissa: u64_2, exponent: i32_4};
    let mut i32_5: i32 = crate::common::pow5bits(i32_3);
    let mut u32_6: u32 = crate::common::log10_pow5(i32_2);
    let mut i32_6: i32 = crate::common::ceil_log2_pow5(i32_1);
    let mut bool_0: bool = crate::f2s_intrinsics::multiple_of_power_of_5_32(u32_3, u32_2);
    let mut buffer_2_ref_0: &crate::buffer::Buffer = &mut buffer_2;
    let mut buffer_3: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_2_ref_0);
    let mut buffer_1_ref_0: &crate::buffer::Buffer = &mut buffer_1;
    let mut buffer_4: crate::buffer::Buffer = crate::buffer::Buffer::clone(buffer_1_ref_0);
    let mut u64_3: u64 = crate::d2s_intrinsics::div5(u64_1);
    let mut floatingdecimal64_1: crate::d2s::FloatingDecimal64 = crate::d2s::d2d(u64_0, u32_1);
    let mut floatingdecimal32_0: crate::f2s::FloatingDecimal32 = crate::f2s::FloatingDecimal32 {mantissa: u32_0, exponent: i32_0};
    panic!("From RustyUnit with love");
}
}