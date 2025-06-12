// Answer 0

#[test]
fn test_advance_positive_mdelta() {
    let mut rng = Lcg64Xsh32::new(1234567890, 1);
    let initial_state = rng.state;

    rng.advance(10);
    // Verify that the state has changed after advancing
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_even_mdelta() {
    let mut rng = Lcg64Xsh32::new(1234567890, 1);
    let initial_state = rng.state;

    rng.advance(8); // 8 is even, which means (mdelta & 1) != 0 is false
    // Verify that the state has changed after advancing
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_zero_mdelta() {
    let mut rng = Lcg64Xsh32::new(1234567890, 1);
    let initial_state = rng.state;

    rng.advance(0); // mdelta == 0
    // Verify that the state remains the same after advancing with delta 0
    assert_eq!(rng.state, initial_state);
}

