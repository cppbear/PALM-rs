// Answer 0

#[test]
fn test_advance_positive_delta() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    let initial_state = rng.state;

    // Positive delta, where mdelta should start as a positive number
    let delta: u128 = 3;
    rng.advance(delta);
    assert!(rng.state != initial_state, "State should change with positive delta");
}

#[test]
fn test_advance_power_of_two_delta() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    let initial_state = rng.state;

    // Delta is a power of two (should have mdelta & 1 == 0 in some steps)
    let delta: u128 = 4; // mdelta will reduce to 2, then 1, and finally 0
    rng.advance(delta);
    assert!(rng.state != initial_state, "State should change with power of two delta");
}

#[test]
fn test_advance_zero_delta() {
    let mut rng = Lcg128Xsl64::new(12345, 67890);
    let initial_state = rng.state;

    // Delta is zero (this should not change the state)
    let delta: u128 = 0; 
    rng.advance(delta);
    assert_eq!(rng.state, initial_state, "State should remain unchanged with zero delta");
}

