// Answer 0

#[test]
fn test_seed() {
    let mut rng = Rng::with_seed(12345);
    rng.seed(67890);
    assert_eq!(rng.get_seed(), 67890);
}

#[test]
fn test_seed_on_forked_rng() {
    let mut rng = Rng::with_seed(12345);
    let mut forked_rng = rng.fork();
    
    // Seed the forked RNG
    forked_rng.seed(67890);
    assert_eq!(forked_rng.get_seed(), 67890);
    
    // Check that original RNG is unaffected
    assert_eq!(rng.get_seed(), 12345);
}

#[test]
#[should_panic(expected = "empty range")]
fn test_empty_seed() {
    let mut rng = Rng::with_seed(12345);
    rng.seed(0);
}

