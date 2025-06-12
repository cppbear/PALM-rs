use crate::d2s;
pub const FLOAT_POW5_INV_BITCOUNT: i32 = d2s::DOUBLE_POW5_INV_BITCOUNT - 64;
pub const FLOAT_POW5_BITCOUNT: i32 = d2s::DOUBLE_POW5_BITCOUNT - 64;
#[cfg_attr(feature = "no-panic", inline)]
pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    (value & ((1u32 << p) - 1)) == 0
}
