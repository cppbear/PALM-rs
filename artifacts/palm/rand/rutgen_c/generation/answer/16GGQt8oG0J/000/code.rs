// Answer 0

#[test]
fn test_from_seed() {
    let seed: [u8; 32] = [1; 32];
    let rng = StdRng::from_seed(seed);
    assert_eq!(rng.0, Rng::from_seed(seed));
}

#[test]
fn test_from_seed_different_seed() {
    let seed1: [u8; 32] = [1; 32];
    let seed2: [u8; 32] = [2; 32];
    let rng1 = StdRng::from_seed(seed1);
    let rng2 = StdRng::from_seed(seed2);
    assert_ne!(rng1.0, rng2.0);
}

