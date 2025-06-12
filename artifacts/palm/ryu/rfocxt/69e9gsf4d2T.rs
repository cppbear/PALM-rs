use self::exponent::{write_exponent2, write_exponent3};
use self::mantissa::{write_mantissa, write_mantissa_long};
use crate::common;
use crate::d2s::{self, d2d, DOUBLE_EXPONENT_BITS, DOUBLE_MANTISSA_BITS};
use crate::f2s::{f2d, FLOAT_EXPONENT_BITS, FLOAT_MANTISSA_BITS};
use core::ptr;
#[cfg(feature = "no-panic")]
use no_panic::no_panic;
pub struct FloatingDecimal64 {
    pub mantissa: u64,
    pub exponent: i32,
}
#[must_use]
#[cfg_attr(feature = "no-panic", no_panic)]
pub unsafe fn format64(f: f64, result: *mut u8) -> usize {
    let bits = f.to_bits();
    let sign = ((bits >> (DOUBLE_MANTISSA_BITS + DOUBLE_EXPONENT_BITS)) & 1) != 0;
    let ieee_mantissa = bits & ((1u64 << DOUBLE_MANTISSA_BITS) - 1);
    let ieee_exponent = (bits >> DOUBLE_MANTISSA_BITS) as u32
        & ((1u32 << DOUBLE_EXPONENT_BITS) - 1);
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
    let kk = length + k;
    debug_assert!(k >= - 324);
    if 0 <= k && kk <= 16 {
        write_mantissa_long(v.mantissa, result.offset(index + length));
        for i in length..kk {
            *result.offset(index + i) = b'0';
        }
        *result.offset(index + kk) = b'.';
        *result.offset(index + kk + 1) = b'0';
        index as usize + kk as usize + 2
    } else if 0 < kk && kk <= 16 {
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        ptr::copy(result.offset(index + 1), result.offset(index), kk as usize);
        *result.offset(index + kk) = b'.';
        index as usize + length as usize + 1
    } else if -5 < kk && kk <= 0 {
        *result.offset(index) = b'0';
        *result.offset(index + 1) = b'.';
        let offset = 2 - kk;
        for i in 2..offset {
            *result.offset(index + i) = b'0';
        }
        write_mantissa_long(v.mantissa, result.offset(index + length + offset));
        index as usize + length as usize + offset as usize
    } else if length == 1 {
        *result.offset(index) = b'0' + v.mantissa as u8;
        *result.offset(index + 1) = b'e';
        index as usize + 2 + write_exponent3(kk - 1, result.offset(index + 2))
    } else {
        write_mantissa_long(v.mantissa, result.offset(index + length + 1));
        *result.offset(index) = *result.offset(index + 1);
        *result.offset(index + 1) = b'.';
        *result.offset(index + length + 1) = b'e';
        index as usize + length as usize + 2
            + write_exponent3(kk - 1, result.offset(index + length + 2))
    }
}
#[cfg_attr(feature = "no-panic", inline)]
pub unsafe fn write_mantissa_long(mut output: u64, mut result: *mut u8) {
    if (output >> 32) != 0 {
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
pub fn d2d(ieee_mantissa: u64, ieee_exponent: u32) -> FloatingDecimal64 {
    let (e2, m2) = if ieee_exponent == 0 {
        (1 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2, ieee_mantissa)
    } else {
        (
            ieee_exponent as i32 - DOUBLE_BIAS - DOUBLE_MANTISSA_BITS as i32 - 2,
            (1u64 << DOUBLE_MANTISSA_BITS) | ieee_mantissa,
        )
    };
    let even = (m2 & 1) == 0;
    let accept_bounds = even;
    let mv = 4 * m2;
    let mm_shift = (ieee_mantissa != 0 || ieee_exponent <= 1) as u32;
    let mut vr: u64;
    let mut vp: u64;
    let mut vm: u64;
    let mut vp_uninit: MaybeUninit<u64> = MaybeUninit::uninit();
    let mut vm_uninit: MaybeUninit<u64> = MaybeUninit::uninit();
    let e10: i32;
    let mut vm_is_trailing_zeros = false;
    let mut vr_is_trailing_zeros = false;
    if e2 >= 0 {
        let q = log10_pow2(e2) - (e2 > 3) as u32;
        e10 = q as i32;
        let k = DOUBLE_POW5_INV_BITCOUNT + pow5bits(q as i32) - 1;
        let i = -e2 + q as i32 + k;
        vr = unsafe {
            mul_shift_all_64(
                m2,
                #[cfg(feature = "small")]
                &compute_inv_pow5(q),
                #[cfg(not(feature = "small"))]
                {
                    debug_assert!(q < DOUBLE_POW5_INV_SPLIT.len() as u32);
                    DOUBLE_POW5_INV_SPLIT.get_unchecked(q as usize)
                },
                i as u32,
                vp_uninit.as_mut_ptr(),
                vm_uninit.as_mut_ptr(),
                mm_shift,
            )
        };
        vp = unsafe { vp_uninit.assume_init() };
        vm = unsafe { vm_uninit.assume_init() };
        if q <= 21 {
            let mv_mod5 = (mv as u32).wrapping_sub(5u32.wrapping_mul(div5(mv) as u32));
            if mv_mod5 == 0 {
                vr_is_trailing_zeros = multiple_of_power_of_5(mv, q);
            } else if accept_bounds {
                vm_is_trailing_zeros = multiple_of_power_of_5(
                    mv - 1 - mm_shift as u64,
                    q,
                );
            } else {
                vp -= multiple_of_power_of_5(mv + 2, q) as u64;
            }
        }
    } else {
        let q = log10_pow5(-e2) - (-e2 > 1) as u32;
        e10 = q as i32 + e2;
        let i = -e2 - q as i32;
        let k = pow5bits(i) - DOUBLE_POW5_BITCOUNT;
        let j = q as i32 - k;
        vr = unsafe {
            mul_shift_all_64(
                m2,
                #[cfg(feature = "small")]
                &compute_pow5(i as u32),
                #[cfg(not(feature = "small"))]
                {
                    debug_assert!(i < DOUBLE_POW5_SPLIT.len() as i32);
                    DOUBLE_POW5_SPLIT.get_unchecked(i as usize)
                },
                j as u32,
                vp_uninit.as_mut_ptr(),
                vm_uninit.as_mut_ptr(),
                mm_shift,
            )
        };
        vp = unsafe { vp_uninit.assume_init() };
        vm = unsafe { vm_uninit.assume_init() };
        if q <= 1 {
            vr_is_trailing_zeros = true;
            if accept_bounds {
                vm_is_trailing_zeros = mm_shift == 1;
            } else {
                vp -= 1;
            }
        } else if q < 63 {
            vr_is_trailing_zeros = multiple_of_power_of_2(mv, q);
        }
    }
    let mut removed = 0i32;
    let mut last_removed_digit = 0u8;
    let output = if vm_is_trailing_zeros || vr_is_trailing_zeros {
        loop {
            let vp_div10 = div10(vp);
            let vm_div10 = div10(vm);
            if vp_div10 <= vm_div10 {
                break;
            }
            let vm_mod10 = (vm as u32).wrapping_sub(10u32.wrapping_mul(vm_div10 as u32));
            let vr_div10 = div10(vr);
            let vr_mod10 = (vr as u32).wrapping_sub(10u32.wrapping_mul(vr_div10 as u32));
            vm_is_trailing_zeros &= vm_mod10 == 0;
            vr_is_trailing_zeros &= last_removed_digit == 0;
            last_removed_digit = vr_mod10 as u8;
            vr = vr_div10;
            vp = vp_div10;
            vm = vm_div10;
            removed += 1;
        }
        if vm_is_trailing_zeros {
            loop {
                let vm_div10 = div10(vm);
                let vm_mod10 = (vm as u32)
                    .wrapping_sub(10u32.wrapping_mul(vm_div10 as u32));
                if vm_mod10 != 0 {
                    break;
                }
                let vp_div10 = div10(vp);
                let vr_div10 = div10(vr);
                let vr_mod10 = (vr as u32)
                    .wrapping_sub(10u32.wrapping_mul(vr_div10 as u32));
                vr_is_trailing_zeros &= last_removed_digit == 0;
                last_removed_digit = vr_mod10 as u8;
                vr = vr_div10;
                vp = vp_div10;
                vm = vm_div10;
                removed += 1;
            }
        }
        if vr_is_trailing_zeros && last_removed_digit == 5 && vr % 2 == 0 {
            last_removed_digit = 4;
        }
        vr
            + ((vr == vm && (!accept_bounds || !vm_is_trailing_zeros))
                || last_removed_digit >= 5) as u64
    } else {
        let mut round_up = false;
        let vp_div100 = div100(vp);
        let vm_div100 = div100(vm);
        if vp_div100 > vm_div100 {
            let vr_div100 = div100(vr);
            let vr_mod100 = (vr as u32)
                .wrapping_sub(100u32.wrapping_mul(vr_div100 as u32));
            round_up = vr_mod100 >= 50;
            vr = vr_div100;
            vp = vp_div100;
            vm = vm_div100;
            removed += 2;
        }
        loop {
            let vp_div10 = div10(vp);
            let vm_div10 = div10(vm);
            if vp_div10 <= vm_div10 {
                break;
            }
            let vr_div10 = div10(vr);
            let vr_mod10 = (vr as u32).wrapping_sub(10u32.wrapping_mul(vr_div10 as u32));
            round_up = vr_mod10 >= 5;
            vr = vr_div10;
            vp = vp_div10;
            vm = vm_div10;
            removed += 1;
        }
        vr + (vr == vm || round_up) as u64
    };
    let exp = e10 + removed;
    FloatingDecimal64 {
        exponent: exp,
        mantissa: output,
    }
}
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
pub fn decimal_length17(v: u64) -> u32 {
    debug_assert!(v < 100000000000000000);
    if v >= 10000000000000000 {
        17
    } else if v >= 1000000000000000 {
        16
    } else if v >= 100000000000000 {
        15
    } else if v >= 10000000000000 {
        14
    } else if v >= 1000000000000 {
        13
    } else if v >= 100000000000 {
        12
    } else if v >= 10000000000 {
        11
    } else if v >= 1000000000 {
        10
    } else if v >= 100000000 {
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
