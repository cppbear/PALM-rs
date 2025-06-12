// Answer 0

#[test]
fn test_uppercase_rng() {
    let mut rng = Rng::with_seed(42);
    let result = rng.uppercase();
    assert!(result.is_ascii_uppercase());
    assert!(result >= 'A' && result <= 'Z');
}

#[test]
fn test_uppercase_rng_multiple() {
    let mut rng = Rng::with_seed(42);
    let results: Vec<_> = (0..10).map(|_| rng.uppercase()).collect();
    for &result in &results {
        assert!(result.is_ascii_uppercase());
        assert!(result >= 'A' && result <= 'Z');
    }
}

