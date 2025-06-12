// Answer 0

#[test]
fn test_advance_zero_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    let initial_state = rng.state;

    rng.advance(0); // invoke advance with delta = 0

    assert_eq!(rng.state, initial_state); // state should not change
}

#[test]
fn test_advance_large_delta() {
    let mut rng = Lcg128Xsl64::new(1, 1);
    let initial_state = rng.state;

    rng.advance(u128::MAX); // invoke advance with the largest possible delta

    // Check that the state changes, as we are moving forward by a large delta
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_small_delta() {
    let mut rng = Lcg128Xsl64::new(2, 3);
    let initial_state = rng.state;

    rng.advance(1); // invoke advance with delta = 1

    // Check that the state changes from the initial value
    assert!(rng.state != initial_state);
}

