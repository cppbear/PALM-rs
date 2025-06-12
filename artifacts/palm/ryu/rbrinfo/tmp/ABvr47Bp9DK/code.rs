pub fn log10_pow5(e: i32) -> u32 /* or u32 -> u32 */ {
    // The first value this approximation fails for is 5^2621 which is just greater than 10^1832.
    debug_assert!(e >= 0);
    debug_assert!(e <= 2620);
    (e as u32 * 732923) >> 20
}