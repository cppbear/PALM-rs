pub fn multiple_of_power_of_5(value: u64, p: u32) -> bool {
    // I tried a case distinction on p, but there was no performance difference.
    pow5_factor(value) >= p
}