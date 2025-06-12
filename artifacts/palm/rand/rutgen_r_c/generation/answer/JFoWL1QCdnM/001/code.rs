// Answer 0

#[test]
fn test_new_with_default_values() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7;
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_minimum_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x0000000000000000;  // Minimum stream value
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_maximum_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0xffffffffffffffff;  // Maximum stream value
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_random_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x123456789abcdef0;  // Random stream value
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_zero_state() {
    let state = 0x0000000000000000;  // Minimum state value
    let stream = 0xa02bdbf7bb3c0a7;
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_large_state() {
    let state = 0xffffffffffffffff;  // Maximum state value
    let stream = 0xa02bdbf7bb3c0a7;
    let rng = Lcg64Xsh32::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

