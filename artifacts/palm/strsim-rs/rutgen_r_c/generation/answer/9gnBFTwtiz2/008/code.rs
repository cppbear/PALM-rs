// Answer 0

#[test]
fn test_generic_hamming_equal_sequences() {
    let result = generic_hamming("hello", "hello");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_different_sequences() {
    let result = generic_hamming("hello", "hallo");
    assert_eq!(result, Ok(1));
}

#[test]
fn test_generic_hamming_different_lengths() {
    let result = generic_hamming("hello", "hell");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let result = generic_hamming("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_longer_first_sequence() {
    let result = generic_hamming("hello world", "hello");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_longer_second_sequence() {
    let result = generic_hamming("hello", "hello world");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

