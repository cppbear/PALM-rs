// Answer 0

#[test]
fn test_new_with_default_values() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    let rng = Lcg128Xsl64::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_new_with_zero_state_and_stream() {
    let state = 0;
    let stream = 0;
    let rng = Lcg128Xsl64::new(state, stream);
    assert_eq!(rng.state, 1); // Since increment will be 1 when stream is 0
}

#[test]
fn test_new_with_large_state() {
    let state = u128::MAX;
    let stream = 0x1;
    let rng = Lcg128Xsl64::new(state, stream);
    assert_eq!(rng.state, u128::MAX.wrapping_add(3)); // increment will be 3 (1 << 1) | 1
}

#[test]
fn test_new_with_odd_stream() {
    let state = 0x1234567890abcdefabcdefabcdef;
    let stream = 0x00000000000000000000000000000001; // odd stream
    let rng = Lcg128Xsl64::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add(3)); // increment will be 3
}

#[test]
fn test_new_with_even_stream() {
    let state = 0x9876543210abcdefabcdefabcdef;
    let stream = 0x00000000000000000000000000000002; // even stream
    let rng = Lcg128Xsl64::new(state, stream);
    assert_eq!(rng.state, state.wrapping_add(5)); // increment will be 5
}

