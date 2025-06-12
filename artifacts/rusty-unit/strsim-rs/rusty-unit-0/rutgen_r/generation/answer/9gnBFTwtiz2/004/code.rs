// Answer 0

#[test]
fn test_generic_hamming_success() {
    let result = generic_hamming("karolin", "kathrin");
    assert_eq!(result, Ok(3));
}

#[test]
fn test_generic_hamming_different_length_error() {
    let result = generic_hamming(vec![1, 2, 3], vec![1, 2]);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_empty_sequences() {
    let result = generic_hamming("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_generic_hamming_one_empty_one_non_empty() {
    let result = generic_hamming("test", "");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_some_differences() {
    let result = generic_hamming(vec![1, 2, 3], vec![1, 3, 2]);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_generic_hamming_equal_sequences() {
    let result = generic_hamming(vec!['a', 'b', 'c'], vec!['a', 'b', 'c']);
    assert_eq!(result, Ok(0));
}

