// Answer 0

#[test]
fn test_get_seed_initial_value() {
    let rng = Rng::with_seed(42);
    assert_eq!(rng.get_seed(), 42);
}

#[test]
fn test_get_seed_another_seed() {
    let rng = Rng::with_seed(100);
    assert_eq!(rng.get_seed(), 100);
}

#[test]
fn test_get_seed_zero_seed() {
    let rng = Rng::with_seed(0);
    assert_eq!(rng.get_seed(), 0);
}

#[test]
fn test_get_seed_large_seed() {
    let rng = Rng::with_seed(u64::MAX);
    assert_eq!(rng.get_seed(), u64::MAX);
}

