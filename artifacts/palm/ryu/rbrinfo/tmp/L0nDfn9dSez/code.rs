pub fn multiple_of_power_of_2(value: u64, p: u32) -> bool {
    debug_assert!(value != 0);
    debug_assert!(p < 64);
    // __builtin_ctzll doesn't appear to be faster here.
    (value & ((1u64 << p) - 1)) == 0
}