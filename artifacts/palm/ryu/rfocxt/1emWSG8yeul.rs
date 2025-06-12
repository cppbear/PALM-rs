#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn ceil_log2_pow5(e: i32) -> i32 {
    log2_pow5(e) + 1
}
#[cfg_attr(feature = "no-panic", inline)]
#[allow(dead_code)]
pub fn log2_pow5(e: i32) -> i32 {
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    ((e as u32 * 1217359) >> 19) as i32
}
