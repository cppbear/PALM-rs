// Answer 0

#[test]
fn test_from_seed_non_zero_seed() {
    let seed: [u8; 32] = [1; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_from_seed_non_zero_seed_alternating() {
    let seed: [u8; 32] = [0, 255, 0, 255, 0, 255, 0, 255,
                          0, 255, 0, 255, 0, 255, 0, 255,
                          0, 255, 0, 255, 0, 255, 0, 255,
                          0, 255, 0, 255, 0, 255, 0, 255];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_from_seed_large_non_zero_seed() {
    let seed: [u8; 32] = [u8::MAX; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
    assert_ne!(rng.s, [0; 4]);
}

#[test]
fn test_from_seed_small_non_zero_seed() {
    let seed: [u8; 32] = [1, 0, 0, 0, 0, 0, 0, 0, 
                          0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
    assert_ne!(rng.s, [0; 4]);
}

