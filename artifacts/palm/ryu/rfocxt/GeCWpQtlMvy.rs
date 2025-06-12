use crate::common::{log10_pow2, log10_pow5, pow5bits};
use crate::f2s_intrinsics::{
    mul_pow5_div_pow2, mul_pow5_inv_div_pow2, multiple_of_power_of_2_32,
    multiple_of_power_of_5_32,
};
pub use crate::f2s_intrinsics::{FLOAT_POW5_BITCOUNT, FLOAT_POW5_INV_BITCOUNT};
pub const FLOAT_MANTISSA_BITS: u32 = 23;
pub const FLOAT_EXPONENT_BITS: u32 = 8;
const FLOAT_BIAS: i32 = 127;
pub struct FloatingDecimal32 {
    pub mantissa: u32,
    pub exponent: i32,
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn f2d(ieee_mantissa: u32, ieee_exponent: u32) -> FloatingDecimal32 {
    let (e2, m2) = if ieee_exponent == 0 {
        (1 - FLOAT_BIAS - FLOAT_MANTISSA_BITS as i32 - 2, ieee_mantissa)
    } else {
        (
            ieee_exponent as i32 - FLOAT_BIAS - FLOAT_MANTISSA_BITS as i32 - 2,
            (1u32 << FLOAT_MANTISSA_BITS) | ieee_mantissa,
        )
    };
    let even = (m2 & 1) == 0;
    let accept_bounds = even;
    let mv = 4 * m2;
    let mp = 4 * m2 + 2;
    let mm_shift = (ieee_mantissa != 0 || ieee_exponent <= 1) as u32;
    let mm = 4 * m2 - 1 - mm_shift;
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
            let l = FLOAT_POW5_INV_BITCOUNT + pow5bits(q as i32 - 1) - 1;
            last_removed_digit = (mul_pow5_inv_div_pow2(
                mv,
                q - 1,
                -e2 + q as i32 - 1 + l,
            ) % 10) as u8;
        }
        if q <= 9 {
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
            vr_is_trailing_zeros = true;
            if accept_bounds {
                vm_is_trailing_zeros = mm_shift == 1;
            } else {
                vp -= 1;
            }
        } else if q < 31 {
            vr_is_trailing_zeros = multiple_of_power_of_2_32(mv, q - 1);
        }
    }
    let mut removed = 0i32;
    let output = if vm_is_trailing_zeros || vr_is_trailing_zeros {
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
            last_removed_digit = 4;
        }
        vr
            + ((vr == vm && (!accept_bounds || !vm_is_trailing_zeros))
                || last_removed_digit >= 5) as u32
    } else {
        while vp / 10 > vm / 10 {
            last_removed_digit = (vr % 10) as u8;
            vr /= 10;
            vp /= 10;
            vm /= 10;
            removed += 1;
        }
        vr + (vr == vm || last_removed_digit >= 5) as u32
    };
    let exp = e10 + removed;
    FloatingDecimal32 {
        exponent: exp,
        mantissa: output,
    }
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    (value & ((1u32 << p) - 1)) == 0
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn mul_pow5_inv_div_pow2(m: u32, q: u32, j: i32) -> u32 {
    #[cfg(feature = "small")]
    {
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
pub fn multiple_of_power_of_5_32(value: u32, p: u32) -> bool {
    pow5factor_32(value) >= p
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
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow2(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow5(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}
