// Answer 0

#[test]
fn test_from_seed_valid_input() {
    let seed: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]; // Increment will be 1
    let rng = Lcg128Xsl64::from_seed(seed);

    assert_eq!(rng.state, 1);
    assert_eq!(rng.increment, 1);
}

#[test]
fn test_from_seed_edge_case() {
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // All zeros
    let rng = Lcg128Xsl64::from_seed(seed);

    assert_eq!(rng.state, 0);
    assert_eq!(rng.increment, 1); // Should ensure odd increment
}

#[test]
fn test_from_seed_increment_case() {
    let seed: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255]; // Large increment
    let rng = Lcg128Xsl64::from_seed(seed);

    assert_eq!(rng.state, 0);
    assert_eq!(rng.increment, u128::from_le_bytes([255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])); // Increment should be odd
}

