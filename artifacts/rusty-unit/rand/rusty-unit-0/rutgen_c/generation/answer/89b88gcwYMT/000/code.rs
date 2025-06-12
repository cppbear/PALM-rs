// Answer 0

#[test]
fn test_advance_basic() {
    let mut rng = Lcg64Xsh32::new(42, 0);
    let initial_state = rng.state;
    rng.advance(1);
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_multiple_steps() {
    let mut rng = Lcg64Xsh32::new(42, 0);
    let initial_state = rng.state;
    rng.advance(5);
    assert!(rng.state != initial_state);

    let next_state = rng.state;
    rng.advance(5);
    assert!(rng.state != next_state);
}

#[test]
fn test_advance_zero() {
    let mut rng = Lcg64Xsh32::new(42, 0);
    let initial_state = rng.state;
    rng.advance(0);
    assert_eq!(rng.state, initial_state);
}

#[test]
fn test_advance_large_delta() {
    let mut rng = Lcg64Xsh32::new(42, 0);
    let initial_state = rng.state;
    rng.advance(u64::MAX);
    assert!(rng.state != initial_state);
}

#[test]
fn test_advance_backwards() {
    let mut rng = Lcg64Xsh32::new(42, 0);
    let initial_state = rng.state;
    rng.advance(1);
    let state_after_advance = rng.state;

    rng.advance(u64::MAX);
    // The state should change, but we can't assert what it should be.
    assert!(rng.state != state_after_advance);
}

