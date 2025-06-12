// Answer 0

#[test]
fn test_new_with_default_values() {
    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7;
    let generator = new(state, stream);
    assert_eq!(generator.state(), state);
    assert_eq!(generator.stream(), stream);
}

#[test]
fn test_new_with_zero_state_zero_stream() {
    let state = 0;
    let stream = 0;
    let generator = new(state, stream);
    assert_eq!(generator.state(), state);
    let expected_stream = (stream << 1) | 1; // should be 1
    assert_eq!(generator.stream(), expected_stream);
}

#[test]
fn test_new_with_max_values() {
    let state = u64::MAX; // 0xffffffffffffffff
    let stream = u64::MAX; // 0xffffffffffffffff
    let generator = new(state, stream);
    assert_eq!(generator.state(), state);
    let expected_stream = (stream << 1) | 1; // should be 0xffffffffffffffff (0x1ffffffffffffff)
    assert_eq!(generator.stream(), expected_stream);
}

#[test]
fn test_new_with_odd_stream_value() {
    let state = 0x1234567890abcdef;
    let stream = 0x1; // an odd value
    let generator = new(state, stream);
    assert_eq!(generator.state(), state);
    let expected_stream = (stream << 1) | 1; // should be 0x3
    assert_eq!(generator.stream(), expected_stream);
}

#[test]
#[should_panic]
fn test_new_with_highest_bit_discarded() {
    let state = 0x1234567890abcdef;
    let stream = 0x8000000000000000; // highest bit set, should be discarded
    let generator = new(state, stream);
    let expected_stream = (stream << 1) | 1; // should be 0x1fffffffffffffff
    assert_eq!(generator.stream(), expected_stream);
}

