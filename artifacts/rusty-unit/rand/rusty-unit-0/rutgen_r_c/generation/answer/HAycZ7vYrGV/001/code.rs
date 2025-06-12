// Answer 0

#[test]
fn test_lcg128xsl64_new_valid_state_and_stream() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    let rng = Lcg128Xsl64::new(state, stream);
    // Check that increment is odd
    assert_eq!(rng.increment % 2, 1);
}

#[test]
fn test_lcg128xsl64_new_zero_stream() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = 0;
    let rng = Lcg128Xsl64::new(state, stream);
    // Check that increment is odd
    assert_eq!(rng.increment % 2, 1);
    assert_eq!(rng.increment, 1); // since (0 << 1) | 1 = 1
}

#[test]
fn test_lcg128xsl64_new_max_stream() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = u128::MAX;
    let rng = Lcg128Xsl64::new(state, stream);
    // Check that increment is odd
    assert_eq!(rng.increment % 2, 1);
}

#[test]
fn test_lcg128xsl64_new_minimum_state() {
    let state: u128 = 0;
    let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    let rng = Lcg128Xsl64::new(state, stream);
    // Check that increment is odd
    assert_eq!(rng.increment % 2, 1);
}

#[test]
fn test_lcg128xsl64_new_large_state() {
    let state: u128 = u128::MAX;
    let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    let rng = Lcg128Xsl64::new(state, stream);
    // Check that increment is odd
    assert_eq!(rng.increment % 2, 1);
}

#[test]
#[should_panic]
fn test_lcg128xsl64_new_invalid_increment() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = 1; // stream leading to an invalid increment
    let rng = Lcg128Xsl64::new(state, stream);
    // Ensure we get the correct state after normal scenarios
    assert_eq!(rng.increment % 2, 1); // This should not trigger if implemented correctly, just a formality
}

