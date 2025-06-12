// Answer 0

#[test]
fn test_from_seed_non_zero_seed() {
    let seed: [u8; 32] = [
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ];
    
    let rng = from_seed(seed);
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_from_seed_zero_seed() {
    let seed: [u8; 32] = [0; 32];
    
    let rng = from_seed(seed);
    assert_eq!(rng.s, [0, 0, 0, 0]); // The state should be non-zero if zeroed in.
}

#[test]
fn test_from_seed_partial_zero_seed() {
    let seed: [u8; 32] = [
        0, 0, 0, 0, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ];
    
    let rng = from_seed(seed);
    assert_ne!(rng.s, [0; 4]);
}

