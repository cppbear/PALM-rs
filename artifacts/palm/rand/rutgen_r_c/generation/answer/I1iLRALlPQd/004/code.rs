// Answer 0

#[test]
fn test_seed_from_u64_non_zero_state() {
    let seed: u64 = 42; // Arbitrary non-zero seed for testing
    let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    assert_ne!(rng.s, [0; 4], "The internal state should not be all zeros.");
}

#[test]
fn test_seed_from_u64_non_zero_state_with_different_seed() {
    let seed: u64 = 100; // Another arbitrary non-zero seed for testing
    let rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    assert_ne!(rng.s, [0; 4], "The internal state should not be all zeros.");
}

#[test]
fn test_seed_from_u64_different_states() {
    let seed1: u64 = 1; // First seed
    let seed2: u64 = 2; // Second seed
    let rng1 = Xoshiro256PlusPlus::seed_from_u64(seed1);
    let rng2 = Xoshiro256PlusPlus::seed_from_u64(seed2);
    assert_ne!(rng1.s, rng2.s, "Different seeds should produce different states.");
}

