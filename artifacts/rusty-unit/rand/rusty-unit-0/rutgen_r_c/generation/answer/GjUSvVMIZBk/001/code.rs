// Answer 0

#[test]
fn test_from_seed_zero_seed() {
    let seed: [u8; 32] = [0; 32];
    let rng = Lcg128CmDxsm64::from_seed(seed);
    assert_eq!(rng.state, 1); // Since increment will be set odd.
}

#[test]
fn test_from_seed_max_seed() {
    let seed: [u8; 32] = [255; 32];
    let rng = Lcg128CmDxsm64::from_seed(seed);
    assert_ne!(rng.increment, 0); // increment should be odd.
}

#[test]
fn test_from_seed_alternate_bits_seed() {
    let seed: [u8; 32] = [
        0b10101010, 0b10101010, 0b10101010, 0b10101010, 
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
        0b10101010, 0b10101010, 0b10101010, 0b10101010,
    ];
    let rng = Lcg128CmDxsm64::from_seed(seed);
    assert!(rng.increment % 2 == 1); // Check increment is odd
}

#[test]
fn test_from_seed_increment() {
    let seed: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let rng = Lcg128CmDxsm64::from_seed(seed);
    assert_eq!(rng.increment, 1); // Increment should be odd and equal to 1
}

#[test]
#[should_panic]
fn test_from_seed_panic_case() {
    let seed: [u8; 32] = [0; 32]; // Should not panic but testing with evil input from seed array that could cause inconsistencies
    Lcg128CmDxsm64::from_seed(seed);
}

