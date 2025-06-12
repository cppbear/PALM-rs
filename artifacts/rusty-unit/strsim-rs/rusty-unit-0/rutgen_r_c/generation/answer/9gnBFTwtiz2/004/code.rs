// Answer 0

#[test]
fn test_generic_hamming_different_length_args() {
    let result = generic_hamming("abc", "ab");
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));

    let result = generic_hamming(vec![1, 2, 3], vec![1, 2]);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));

    let result = generic_hamming(vec![true, false], vec![true]);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));

    let result = generic_hamming(vec!['a', 'b'], vec!['a']);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));

    let result = generic_hamming(vec![1, 2, 3], vec![1]);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

