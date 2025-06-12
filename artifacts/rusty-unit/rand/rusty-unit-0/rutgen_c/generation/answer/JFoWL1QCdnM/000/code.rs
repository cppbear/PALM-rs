// Answer 0

#[test]
fn test_lcg64xsh32_new_default_values() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7;
    let generator = Lcg64Xsh32::new(state, stream);
    
    assert_eq!(generator.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_lcg64xsh32_new_different_stream() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0x1234567890abcdef;
    let generator = Lcg64Xsh32::new(state, stream);
    
    assert_eq!(generator.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_lcg64xsh32_new_large_values() {
    let state = 0xffffffffffffffff;
    let stream = 0xffffffffffffffff;
    let generator = Lcg64Xsh32::new(state, stream);
    
    assert_eq!(generator.state, state.wrapping_add((stream << 1) | 1));
}

#[test]
fn test_lcg64xsh32_new_zero_values() {
    let state = 0;
    let stream = 0;
    let generator = Lcg64Xsh32::new(state, stream);
    
    assert_eq!(generator.state, (stream << 1) | 1);
}

