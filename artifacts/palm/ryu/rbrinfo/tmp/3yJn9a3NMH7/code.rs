pub fn log2_pow5(e: i32) -> i32 /* or u32 -> u32 */ {
    // This approximation works up to the point that the multiplication
    // overflows at e = 3529. If the multiplication were done in 64 bits, it
    // would fail at 5^4004 which is just greater than 2^9297.
    debug_assert!(e >= 0);
    debug_assert!(e <= 3528);
    ((e as u32 * 1217359) >> 19) as i32
}