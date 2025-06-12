// Answer 0

#[test]
fn test_get_seed_initial_seed() {
    let rng = Rng::with_seed(42);
    assert_eq!(rng.get_seed(), 42);
}

#[test]
fn test_get_seed_different_seeds() {
    let rng1 = Rng::with_seed(123);
    let rng2 = Rng::with_seed(456);
    assert_eq!(rng1.get_seed(), 123);
    assert_eq!(rng2.get_seed(), 456);
}

#[test]
fn test_get_seed_zero_seed() {
    let rng = Rng::with_seed(0);
    assert_eq!(rng.get_seed(), 0);
}

