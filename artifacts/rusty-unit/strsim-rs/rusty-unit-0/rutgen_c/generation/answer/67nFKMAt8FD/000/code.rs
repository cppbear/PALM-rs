// Answer 0

#[test]
fn test_hamming_equal_strings() {
    assert_eq!(Ok(0), hamming("test", "test"));
}

#[test]
fn test_hamming_different_strings() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_different_lengths() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", "ham"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_empty_string() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", ""));
}

#[test]
fn test_hamming_one_char_different() {
    assert_eq!(Ok(1), hamming("a", "b"));
}

#[test]
fn test_hamming_one_char_same() {
    assert_eq!(Ok(0), hamming("a", "a"));
}

