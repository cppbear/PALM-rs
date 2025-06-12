// Answer 0

#[test]
fn test_from_seed() {
    let seed: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
    ];
    let rng = Lcg128Xsl64::from_seed(seed);
    assert_eq!(rng.state, 0x0000000000000001 | (0x0000000000000002 << 64));
    assert_eq!(rng.increment, (0x0000000000000003 << 1) | 1);
}

#[test]
fn test_from_seed_with_different_values() {
    let seed: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
        0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];
    let rng = Lcg128Xsl64::from_seed(seed);
    assert_eq!(rng.state, 0xFFFFFFFFFFFFFFFF | (0xFFFFFFFFFFFFFFFE << 64));
    assert_eq!(rng.increment, (0x0000000000000008 << 1) | 1);
}

#[test]
fn test_from_seed_increment_is_odd() {
    let seed: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let rng = Lcg128Xsl64::from_seed(seed);
    assert_eq!(rng.increment % 2, 1); // Ensure increment is odd
}

