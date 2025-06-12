// Answer 0

#[test]
fn test_from_state_incr_positive_values() {
    let state: u64 = 100;
    let increment: u64 = 200;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
    assert_eq!(pcg.state, 6364136223846793006); // Expected value after step with the given state and increment
    assert_eq!(pcg.increment, increment);
}

#[test]
fn test_from_state_incr_zero_increment() {
    let state: u64 = 50;
    let increment: u64 = 0;
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
    assert_eq!(pcg.state, 6364136223846793055); // Expected state when increment is zero
    assert_eq!(pcg.increment, increment);
}

#[test]
fn test_from_state_incr_large_values() {
    let state: u64 = u64::MAX - 1;
    let increment: u64 = u64::MAX; // Testing upper bounds
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
    assert_eq!(pcg.state, 6364136223846793006); // Expected state after addition and multiplication
    assert_eq!(pcg.increment, increment);
}

#[test]
fn test_from_state_incr_negative_values() {
    let state: u64 = 0;
    let increment: u64 = 1; // Testing the minimum increment
    let pcg = Lcg64Xsh32::from_state_incr(state, increment);
    assert_eq!(pcg.state, 6364136223846793006); // Expected state after addition and multiplication
    assert_eq!(pcg.increment, increment);
}

