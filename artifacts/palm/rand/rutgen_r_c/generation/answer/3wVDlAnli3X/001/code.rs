// Answer 0

#[test]
fn test_from_seed_zero_seed() {
    let seed: [u8; 16] = [0; 16];
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 1); // State should be initialized with 1
    assert_eq!(rng.increment, 1); // Increment should be odd and equal to 1
}

#[test]
fn test_from_seed_max_seed() {
    let seed: [u8; 16] = [255; 16]; // Max value for each byte
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 18446744073709551615); // Expected state after setting max seed
    assert_eq!(rng.increment, 18446744073709551615); // Increment should be odd and max
}

#[test]
fn test_from_seed_odd_increment() {
    let seed: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 5784012483888740524); // Valid expected state
    assert_eq!(rng.increment, 18446744073709551617); // Increment should still be odd
}

#[test]
fn test_from_seed_highest_low_bit_ignored() {
    let seed: [u8; 16] = [0; 8].iter().chain(&[255, 255, 255, 255, 255, 255, 255, 255]).cloned().collect::<Vec<_>>().try_into().unwrap(); // Ignoring the lowest bit
    let rng = Lcg64Xsh32::from_seed(seed);
    assert_eq!(rng.state, 0); // Adjust accordingly for expected state
    assert_eq!(rng.increment, 1); // Increment should be odd
}

