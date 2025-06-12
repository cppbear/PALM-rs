// Answer 0

#[test]
fn test_from_state_incr_small_values() {
    let state: u128 = 1;
    let increment: u128 = 1;
    let pcg = from_state_incr(state, increment);
    assert!(pcg.state != 0);
}

#[test]
fn test_from_state_incr_large_increment() {
    let state: u128 = 1;
    let increment: u128 = u128::MAX;
    let pcg = from_state_incr(state, increment);
    assert!(pcg.state != 0);
}

#[test]
fn test_from_state_incr_zero_increment() {
    let state: u128 = 5;
    let increment: u128 = 0;
    let pcg = from_state_incr(state, increment);
    assert_eq!(pcg.state, 6);
}

#[test]
fn test_from_state_incr_boundary_state() {
    let state: u128 = u128::MAX;
    let increment: u128 = 1;
    let pcg = from_state_incr(state, increment);
    assert!(pcg.state != u128::MAX);
}

#[test]
fn test_from_state_incr_boundary_increment() {
    let state: u128 = 1;
    let increment: u128 = u128::MAX;
    let pcg = from_state_incr(state, increment);
    assert!(pcg.state < state.wrapping_add(increment));
}

