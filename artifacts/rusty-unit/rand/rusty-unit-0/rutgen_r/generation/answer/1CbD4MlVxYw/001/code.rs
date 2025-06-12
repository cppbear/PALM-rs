// Answer 0

#[test]
fn test_from_seed_all_zeroes() {
    let seed = [0u8; 32];
    let rng = from_seed(seed);
    // Assert that the generated RNG is initialized with the state corresponding to zero.
    // Assuming the expected behavior leads to this:
    assert_eq!(rng.s, [0, 0, 0, 0]);
}

#[test]
fn test_from_seed_non_zero_seed() {
    let seed = [1u8; 32];
    let rng = from_seed(seed);
    // Assert that the generated RNG is not all zeros
    assert_ne!(rng.s, [0, 0, 0, 0]);
}

#[test]
fn test_from_seed_alternate_zeroes() {
    let seed = [0u8, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 
                0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let rng = from_seed(seed);
    // Assert that the generated RNG is not all zeros
    assert_ne!(rng.s, [0, 0, 0, 0]);
}

