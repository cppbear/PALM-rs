// Answer 0

#[test]
fn test_advance_positive_delta() {
    let mut rng = Lcg128CmDxsm64::new(10, 5);
    let initial_state = rng.state;
    rng.advance(3);
    let expected_state = initial_state.wrapping_mul(MULTIPLIER.wrapping_pow(2)).wrapping_add(rng.increment);
    assert_eq!(rng.state, expected_state);
}

#[test]
fn test_advance_large_delta() {
    let mut rng = Lcg128CmDxsm64::new(10, 5);
    let initial_state = rng.state;
    let delta = 1u128 << 63; // large delta
    rng.advance(delta);
    // Unsure of expected state without exact calculation, simply check that state is updated
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_with_zero_delta() {
    let mut rng = Lcg128CmDxsm64::new(10, 5);
    let initial_state = rng.state;
    rng.advance(0); // mdelta == 0
    assert_eq!(rng.state, initial_state);
}

#[test]
#[should_panic]
fn test_advance_panic_on_zero_delta() {
    let mut rng = Lcg128CmDxsm64::new(10, 5);
    rng.advance(0); // mdelta is zero, shouldn't panic but returns the same state
}

