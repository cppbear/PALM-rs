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
