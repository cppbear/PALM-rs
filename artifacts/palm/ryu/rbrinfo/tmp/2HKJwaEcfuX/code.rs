pub fn log10_pow2(e: i32) -> u32 /* or u32 -> u32 */ {
    // The first value this approximation fails for is 2^1651 which is just greater than 10^297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 1650);
    (e as u32 * 78913) >> 18
}