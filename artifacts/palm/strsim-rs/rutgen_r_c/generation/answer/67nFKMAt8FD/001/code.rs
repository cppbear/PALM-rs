// Answer 0

#[test]
fn test_hamming_equal_length_different_chars() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_equal_length_same_chars() {
    assert_eq!(Ok(0), hamming("abcdef", "abcdef"));
}

#[test]
fn test_hamming_different_length_shorter() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("abc", "ab"));
}

#[test]
fn test_hamming_different_length_longer() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("ab", "abc"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_empty_string() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("a", ""));
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("", "a"));
}

#[test]
fn test_hamming_large_equal_length() {
    assert_eq!(Ok(5), hamming("abcdefghijklmnopqrstuvwxyz", "abcxyzdefghijklmnopqrstuvwxyzz"));
}

#[test]
fn test_hamming_large_different_length() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("abcdefghij", "abcdefghijklmno"));
}

