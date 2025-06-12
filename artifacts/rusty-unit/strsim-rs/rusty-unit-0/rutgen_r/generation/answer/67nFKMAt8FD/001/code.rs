// Answer 0

#[test]
fn test_hamming_equal_length_different_chars() {
    assert_eq!(Ok(3), hamming("hamming", "hammers"));
}

#[test]
fn test_hamming_equal_length_same_chars() {
    assert_eq!(Ok(0), hamming("test", "test"));
}

#[test]
fn test_hamming_different_length_shorter() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("hamming", "ham"));
}

#[test]
fn test_hamming_different_length_longer() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("ham", "hamming"));
}

#[test]
fn test_hamming_empty_strings() {
    assert_eq!(Ok(0), hamming("", ""));
}

#[test]
fn test_hamming_one_empty_string() {
    assert_eq!(Err(StrSimError::DifferentLengthArgs), hamming("a", ""));
}

#[test]
fn test_hamming_all_different() {
    assert_eq!(Ok(4), hamming("abcd", "efgh"));
}

#[test]
fn test_hamming_mixed_case() {
    assert_eq!(Ok(2), hamming("HeLLo", "heLLo"));
}

