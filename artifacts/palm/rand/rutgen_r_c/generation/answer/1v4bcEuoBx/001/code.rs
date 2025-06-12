// Answer 0

#[test]
fn test_from_state_incr_with_zero_increment() {
    let state: u128 = 42;
    let increment: u128 = 0;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
    assert_eq!(pcg.state, (state.wrapping_add(increment).wrapping_mul(MULTIPLIER).wrapping_add(increment)));
}

#[test]
fn test_from_state_incr_with_large_state() {
    let state: u128 = u128::MAX;
    let increment: u128 = 1;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
    assert_eq!(pcg.state, (state.wrapping_add(increment).wrapping_mul(MULTIPLIER).wrapping_add(increment)));
}

#[test]
fn test_from_state_incr_with_large_increment() {
    let state: u128 = 10;
    let increment: u128 = u128::MAX;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
    assert_eq!(pcg.state, (state.wrapping_add(increment).wrapping_mul(MULTIPLIER).wrapping_add(increment)));
}

#[test]
fn test_from_state_incr_with_high_multiplier_effect() {
    let state: u128 = 15;
    let increment: u128 = 5;
    let pcg = Lcg128Xsl64::from_state_incr(state, increment);
    assert_eq!(pcg.state, (state.wrapping_add(increment).wrapping_mul(MULTIPLIER).wrapping_add(increment)));
}

