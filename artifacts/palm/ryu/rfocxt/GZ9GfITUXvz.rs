#[cfg_attr(feature = "no-panic", inline)]
pub fn pow5bits(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    (((e as u32 * 1217359) >> 19) + 1) as i32
}
