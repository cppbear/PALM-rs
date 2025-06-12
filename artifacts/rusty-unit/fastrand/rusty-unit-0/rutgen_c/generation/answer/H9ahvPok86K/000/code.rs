// Answer 0

#[test]
fn test_rng_with_seed() {
    let rng = Rng::with_seed(42);
    assert_eq!(rng.get_seed(), 42);
}

#[test]
fn test_rng_with_seed_diff_values() {
    let rng1 = Rng::with_seed(100);
    let rng2 = Rng::with_seed(200);
    assert_ne!(rng1.get_seed(), rng2.get_seed());
}

#[test]
fn test_rng_with_seed_zero() {
    let rng = Rng::with_seed(0);
    assert_eq!(rng.get_seed(), 0);
}

#[test]
#[should_panic(expected = "empty range: Unbounded..Included(0)")]
fn test_rng_char_empty_range_inclusive() {
    let mut rng = Rng::with_seed(1);
    rng.char(0..=0); // this will panic
}

#[test]
#[should_panic(expected = "empty range: Included(0)..Excluded(1)")]
fn test_rng_char_empty_range_exclusive() {
    let mut rng = Rng::with_seed(1);
    rng.char(1..1); // this will panic
}

