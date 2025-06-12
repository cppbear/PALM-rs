// Answer 0

#[test]
fn test_from_seed_non_zero_seed() {
    let seed: [u8; 32] = [
        1, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
}

#[test]
fn test_from_seed_zero_seed() {
    let seed: [u8; 32] = [0; 32];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_eq!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
}

#[test]
fn test_from_seed_partial_zero_seed() {
    let seed: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let rng = Xoshiro256PlusPlus::from_seed(seed);
    assert_ne!(rng, Xoshiro256PlusPlus::seed_from_u64(0));
}

