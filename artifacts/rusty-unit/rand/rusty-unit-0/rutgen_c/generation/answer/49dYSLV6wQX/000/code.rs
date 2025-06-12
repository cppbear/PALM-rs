// Answer 0

#[test]
fn test_advance_basic() {
    let mut rng = Lcg128CmDxsm64::new(10, 20);
    rng.advance(1);
    assert_ne!(rng.state, 10);
}

#[test]
fn test_advance_large_delta() {
    let mut rng = Lcg128CmDxsm64::new(5, 15);
    rng.advance(1000);
    assert_ne!(rng.state, 5);
}

#[test]
fn test_advance_zero_delta() {
    let mut rng = Lcg128CmDxsm64::new(42, 99);
    let initial_state = rng.state;
    rng.advance(0);
    assert_eq!(rng.state, initial_state);
}

#[test]
fn test_advance_high_value() {
    let mut rng = Lcg128CmDxsm64::new(u128::MAX - 1, 1);
    rng.advance(2);
    assert_ne!(rng.state, u128::MAX - 1);
}

#[test]
fn test_advance_negative_delta() {
    let mut rng = Lcg128CmDxsm64::new(10, 20);
    rng.advance(!1); // effectively a large positive delta
    assert_ne!(rng.state, 10);
}

