use crate::common::{log10_pow2, log10_pow5, pow5bits};
#[cfg(not(feature = "small"))]
pub use crate::d2s_full_table::{DOUBLE_POW5_INV_SPLIT, DOUBLE_POW5_SPLIT};
use crate::d2s_intrinsics::{
    div10, div100, div5, mul_shift_all_64, multiple_of_power_of_2, multiple_of_power_of_5,
};
#[cfg(feature = "small")]
pub use crate::d2s_small_table::{compute_inv_pow5, compute_pow5};
use core::mem::MaybeUninit;
pub const DOUBLE_MANTISSA_BITS: u32 = 52;
pub const DOUBLE_EXPONENT_BITS: u32 = 11;
pub const DOUBLE_BIAS: i32 = 1023;
pub const DOUBLE_POW5_INV_BITCOUNT: i32 = 125;
pub const DOUBLE_POW5_BITCOUNT: i32 = 125;
pub struct FloatingDecimal64 {
    pub mantissa: u64,
    pub exponent: i32,
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
pub fn log10_pow5(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn log10_pow2(e: i32) -> u32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn div100(x: u64) -> u64 {
    x / 100
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn div10(x: u64) -> u64 {
    x / 10
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2(value: u64, p: u32) -> bool {
    debug_assert!(value != 0);
    debug_assert!(p < 64);
    (value & ((1u64 << p) - 1)) == 0
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_5(value: u64, p: u32) -> bool {
    pow5_factor(value) >= p
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
#[cfg_attr(feature = "no-panic", inline)]
pub fn div5(x: u64) -> u64 {
    x / 5
}
#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}
