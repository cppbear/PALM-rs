// Answer 0

#[test]
fn test_seed_set_valid_seed() {
    let mut rng = Rng::with_seed(0);
    rng.seed(42);
    assert_eq!(rng.get_seed(), 42);
}

#[test]
fn test_seed_set_zero_seed() {
    let mut rng = Rng::with_seed(0);
    rng.seed(0);
    assert_eq!(rng.get_seed(), 0);
}

#[test]
fn test_seed_set_large_seed() {
    let mut rng = Rng::with_seed(0);
    rng.seed(u64::MAX);
    assert_eq!(rng.get_seed(), u64::MAX);
}

