// Answer 0

#[test]
fn test_new_function_with_default_values() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    
    let pcg = Lcg128CmDxsm64::new(state, stream);
    
    assert_eq!(pcg.state, 0xcafef00dd15ea5e5.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_function_with_zero_state_and_stream() {
    let state: u128 = 0;
    let stream: u128 = 0;
    
    let pcg = Lcg128CmDxsm64::new(state, stream);
    
    assert_eq!(pcg.state, 0.wrapping_add(1)); // increment should be 1
}

#[test]
fn test_new_function_with_max_u128_state() {
    let state: u128 = u128::MAX;
    let stream: u128 = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    
    let pcg = Lcg128CmDxsm64::new(state, stream);
    
    assert_eq!(pcg.state, u128::MAX.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_function_with_min_stream() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = 1; // minimum odd stream
    
    let pcg = Lcg128CmDxsm64::new(state, stream);
    
    assert_eq!(pcg.state, state.wrapping_add(3)); // increment should be (1 << 1) | 1 = 3
}

#[test]
fn test_new_function_with_large_stream() {
    let state: u128 = 0xcafef00dd15ea5e5;
    let stream: u128 = u128::MAX;
    
    let pcg = Lcg128CmDxsm64::new(state, stream);
    
    assert_eq!(pcg.state, state.wrapping_add(u128::MAX << 1 | 1));
}

