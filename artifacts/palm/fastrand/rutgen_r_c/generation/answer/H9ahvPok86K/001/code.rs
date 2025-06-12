// Answer 0

#[test]
fn test_rng_with_seed_valid_seed() {
    let seed: u64 = 123456789;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

#[test]
fn test_rng_with_seed_zero_seed() {
    let seed: u64 = 0;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

#[test]
fn test_rng_with_seed_max_seed() {
    let seed: u64 = u64::MAX;
    let rng = Rng::with_seed(seed);
    assert_eq!(rng.0, seed);
}

