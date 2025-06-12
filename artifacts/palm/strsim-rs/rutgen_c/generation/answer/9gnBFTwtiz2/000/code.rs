// Answer 0

#[test]
fn test_hamming_same_length_different_characters() {
    let result = generic_hamming("abc", "abd");
    assert_eq!(result, Ok(1));
}

#[test]
fn test_hamming_same_length_no_differences() {
    let result = generic_hamming("abc", "abc");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_hamming_different_length() {
    let result = generic_hamming("abc", "ab");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_hamming_empty_strings() {
    let result = generic_hamming("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_hamming_one_empty_one_non_empty() {
    let result = generic_hamming("", "a");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

