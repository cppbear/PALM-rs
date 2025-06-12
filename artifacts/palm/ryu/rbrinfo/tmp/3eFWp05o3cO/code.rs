pub fn multiple_of_power_of_2_32(value: u32, p: u32) -> bool {
    // __builtin_ctz doesn't appear to be faster here.
    (value & ((1u32 << p) - 1)) == 0
}